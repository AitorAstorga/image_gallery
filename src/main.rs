mod components;
mod config;
mod models;
mod pages;
mod router;

use yew::prelude::*;
use yew_router::BrowserRouter;
use crate::router::AppRouter;

// Main App Component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppRouter />
        </BrowserRouter>
    }
}

// Main function
fn main() {
    yew::Renderer::<App>::new().render();
}
