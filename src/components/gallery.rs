// src/components/gallery.rs

use yew::prelude::*;
use crate::models::GalleryProps;

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    props
        .images
        .iter()
        .enumerate()
        .map(|(index, image)| {
            let onclick = {
                let cb = props.on_click.clone();
                move |_| cb.emit(index)
            };

            html! {
                <img
                    key={image.filename.clone()}
                    src={format!("/static/images/{}", image.filename)}
                    alt={format!("Image {}", index + 1)}
                    class="gallery-image"
                    {onclick}
                    loading="lazy"
                />
            }
        })
        .collect()
}