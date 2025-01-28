// src/pages/home.rs

use yew::prelude::*;

use gloo_net::http::Request;
use web_sys::MouseEvent;
use yew_hooks::use_event_with_window;
use crate::{
    components::gallery::Gallery,
    models::ImageData
};

#[function_component(Home)]
pub fn home() -> Html {
    let images = use_state(Vec::new);
    let modal_open = use_state(|| false);
    let current_index = use_state(|| 0);
    let scale = use_state(|| 1.0);
    let translate = use_state(|| (0.0, 0.0));
    let _is_dragging = use_state(|| false); // Prefix with underscore since it's not used yet

    // Fetch images
    {
        let images = images.clone();
        use_effect_with((), move |_| {
            let images = images.clone();
            wasm_bindgen_futures::spawn_local(async move {
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

    // Update the html! block in the App component:
    html! {
        <>
            <div class="card">
                <h1>{ "¡Hola, soy Blue!" }</h1>
                <p>{
                    "Nací el 5 de agosto de 2022. Mi origen es un misterio, pero fui rescatado de un pequeño pueblo antes de encontrar un hogar con "
                    }<a href="https://github.com/AitorAstorga">{ "Aitor Astorga" }</a>{
                    ". En esta galería, tienes el honor de admirar mi espléndida y adorable presencia. ¡Espero que disfrutes de mi fotogénico encanto tanto como yo disfruto posar para la cámara!"
                }</p>

                <div class="gallery" id="gallery">
                    <Gallery
                        images={(*images).clone()}
                        on_click={Callback::from(move |index| {
                            current_index.set(index);
                            modal_open.set(true);
                        })}
                    />
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
            </div>

            <footer class="footer">
                <div class="footer-main">
                    <p align="left" class="visit-counter">  <img 
                        src="https://visitcounter.aichan.ovh/counter/michi.blue/svg?label=Visitante Nº&background_label=00000000&background_counter=00000000&shadow_opacity=0&grad_stop1_color=00000000&grad_stop1_opacity=0&grad_stop1_opacity=0&grad_stop2_opacity=0" height=20
                        alt="Visit Counter" />
                    </p>
                </div>
                <a
                    href="https://github.com/AitorAstorga/Image-Gallery"
                    aria-label="GitHub Repository"
                    class="github"
                >
                    <p class="github-label">{ "GitHub" }</p>
                    <img
                        src="/static/resources/github-logo.svg"
                        alt="GitHub Logo"
                        class="github-logo"
                    />
                </a>
            </footer>
        </>
    }
}
