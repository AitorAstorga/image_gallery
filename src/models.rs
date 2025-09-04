// src/models.rs
use serde::{Deserialize, Serialize};
use yew::prelude::{Properties, Callback};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageData {
    pub filename: String,
}

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub images: Vec<ImageData>,
    pub on_click: Callback<usize>,
}
