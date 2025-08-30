// src/config.rs
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GalleryConfig {
    pub gallery_title: String,
    pub main_heading: String,
    pub description_html: String,
    pub visit_counter_url: String,
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            gallery_title: "Image Gallery".to_string(),
            main_heading: "Welcome!".to_string(),
            description_html: r#"This is the gallery description that you can customize with GALLERY_DESCRIPTION env variable and even supports <a href="https://git.prisma.moe/aichan/image_gallery">links like this</a>."#.to_string(),
            visit_counter_url: "https://visitcounter.aichan.ovh/counter/gallery_example/svg?label=Visits&background_label=00000000&background_counter=00000000&shadow_opacity=0&grad_stop1_color=00000000&grad_stop1_opacity=0&grad_stop1_opacity=0&grad_stop2_opacity=0".to_string(),
        }
    }
}

impl GalleryConfig {
    pub async fn load_async() -> Self {
        // Load configuration from /static/gallery-config.json
        web_sys::console::log_1(&"Loading config from /static/gallery-config.json".into());
        match Request::get("/static/gallery-config.json").send().await {
            Ok(response) => {
                web_sys::console::log_1(&format!("Config response status: {}", response.status()).into());
                match response.json::<GalleryConfig>().await {
                    Ok(config) => {
                        web_sys::console::log_1(&"Successfully loaded config".into());
                        config
                    },
                    Err(e) => {
                        web_sys::console::warn_1(&format!("Failed to parse config JSON: {:?}, using defaults", e).into());
                        Self::default()
                    }
                }
            }
            Err(e) => {
                web_sys::console::warn_1(&format!("Failed to load config file: {:?}, using defaults", e).into());
                Self::default()
            }
        }
    }
}

// Component for rendering HTML content
#[derive(Properties, PartialEq)]
pub struct HtmlContentProps {
    pub content: String,
}

#[function_component(HtmlContent)]
pub fn html_content(props: &HtmlContentProps) -> Html {
    // Basic HTML support without sanitization
    Html::from_html_unchecked(AttrValue::from(props.content.clone()))
}