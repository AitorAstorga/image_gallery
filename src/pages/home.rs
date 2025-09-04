// src/pages/home.rs
use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::{MouseEvent, window};
use yew_hooks::use_event_with_window;
use crate::{
    components::gallery::Gallery,
    models::ImageData,
    config::{GalleryConfig, HtmlContent}
};

#[function_component(Home)]
pub fn home() -> Html {
    let config = use_state(|| None::<GalleryConfig>);
    let images = use_state(Vec::new);
    let modal_open = use_state(|| false);
    let current_index = use_state(|| 0);
    let scale = use_state(|| 1.0);
    let translate = use_state(|| (0.0, 0.0));
    let _is_dragging = use_state(|| false); // Prefix with underscore since it's not used yet

    // Load configuration and images
    {
        let config = config.clone();
        let images = images.clone();
        use_effect_with((), move |_| {
            let config = config.clone();
            let images = images.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Load configuration
                let loaded_config = GalleryConfig::load_async().await;
                config.set(Some(loaded_config));

                // Load images
                let fetched: Vec<ImageData> = Request::get("/static/images.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                images.set(fetched);
            });
            || ()
        });
    }

    // Update document title when config changes
    {
        let config = config.clone();
        use_effect_with(config.clone(), move |config_state| {
            if let Some(cfg) = config_state.as_ref() {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        let _ = document.set_title(&cfg.gallery_title);
                    }
                }
            }
            || ()
        });
    }

    // Keyboard navigation
    {
        let modal_open = modal_open.clone();
        let current_index = current_index.clone();
        let images = images.clone();

        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if *modal_open {
                match e.key().as_str() {
                    "ArrowRight" => current_index.set((*current_index + 1) % images.len()),
                    "ArrowLeft" => {
                        current_index.set((*current_index + images.len() - 1) % images.len())
                    }
                    "Escape" => modal_open.set(false),
                    _ => (),
                }
            }
        });
    }

    // Image navigation callbacks
    let show_next = {
        let current_index = current_index.clone();
        let images = images.clone();
        let scale = scale.clone();
        let translate = translate.clone();

        Callback::from(move |_| {
            current_index.set((*current_index + 1) % images.len());
            scale.set(1.0);
            translate.set((0.0, 0.0));
        })
    };

    let show_prev = {
        let current_index = current_index.clone();
        let images = images.clone();
        let scale = scale.clone();
        let translate = translate.clone();

        Callback::from(move |_| {
            current_index.set((*current_index + images.len() - 1) % images.len());
            scale.set(1.0);
            translate.set((0.0, 0.0));
        })
    };

    // Zoom functionality
    let toggle_zoom = {
        let scale = scale.clone();
        let translate = translate.clone();
        Callback::from(move |_e: MouseEvent| {
            // Add underscore to unused event parameter
            if *scale == 1.0 {
                scale.set(2.0);
            } else {
                scale.set(1.0);
                translate.set((0.0, 0.0));
            }
        })
    };

    // Modal callbacks
    let on_modal_click = {
        let modal_open = modal_open.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::Element>() {
                if target.id() == "modal" {
                    modal_open.set(false);
                }
            }
        })
    };

    // Clone needed values for modal rendering
    let modal_open_clone = modal_open.clone();
    let current_index_clone = current_index.clone();
    let images_clone = images.clone();

    html! {
        <>
            if let Some(cfg) = config.as_ref() {
                <div class="color-header">
                    <header>
                        <h1>{ cfg.gallery_title.clone() }</h1>
                    </header>
                </div>

                <div class="gallery-container">
                    <div class="card">
                        <h1>{ cfg.main_heading.clone() }</h1>
                        <div>
                            <HtmlContent content={ cfg.description_html.clone() } />
                        </div>

                <div class="gallery" id="gallery">
                    <Gallery
                        images={(*images).clone()}
                        on_click={Callback::from(move |index| {
                            current_index.set(index);
                            modal_open.set(true);
                        })}
                    />
                </div>
                </div>
            </div>

            if *modal_open_clone {
                    <div id="modal"
                        class="modal"
                        aria-hidden="false"
                        role="dialog"
                        aria-labelledby="modalTitle"
                        onclick={on_modal_click.clone()}
                    >
                        <div class="modal-header">
                            <div class="modal-controls">
                                <a
                                    id="downloadBtn"
                                    href={format!("/static/images/{}", images_clone[*current_index_clone].filename)}
                                    download="image.jpg"
                                    aria-label="Download Image"
                                    class="download-btn"
                                >
                                    <img
                                        src="/static/resources/circle-down-sharp-solid.svg"
                                        alt="Download Icon"
                                        class="download-icon"
                                    />
                                </a>
                            </div>
                            <span
                                class="close"
                                aria-label="Close"
                                onclick={Callback::from(move |_| modal_open_clone.set(false))}
                            >
                                { "×" }
                            </span>
                        </div>
                        <div class="modal-content">
                            <button class="prev" aria-label="Previous Image" onclick={show_prev.clone()}>
                                { "❮" }
                            </button>
                            <div class="image-container">
                                <img
                                    class="modal-image"
                                    id="modalImage"
                                    src={format!("/static/images/{}", images_clone[*current_index_clone].filename)}
                                    alt={format!("Image {}", *current_index_clone + 1)}
                                    onclick={toggle_zoom.clone()}
                                    style={format!(
                                        "transform: scale({}) translate({}px, {}px);",
                                        *scale,
                                        translate.0,
                                        translate.1
                                    )}
                                />
                            </div>
                            <button class="next" aria-label="Next Image" onclick={show_next.clone()}>
                                { "❯" }
                            </button>
                        </div>
                        <div class="modal-caption" id="modalCaption">
                            { format!("Image {} of {}", *current_index_clone + 1, images_clone.len()) }
                        </div>
                    </div>
                }

                <footer class="footer">
                    <div class="footer-main">
                        <p align="left" class="visit-counter">  <img 
                            src={ cfg.visit_counter_url.clone() } height=20
                            alt="Visit Counter" />
                        </p>
                    </div>
                    <a
                        href="https://git.prisma.moe/aichan/image_gallery"
                        aria-label="Source Code"
                        class="git"
                    >
                        <p class="git-label">{ "Source Code" }</p>
                        <img
                            src="/static/resources/git-logo.svg"
                            alt="Git Logo"
                            class="git-logo"
                        />
                    </a>
                </footer>
            } else {
                <div class="loading">{ "Loading..." }</div>
            }
        </>
    }
}
