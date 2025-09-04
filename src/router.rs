// src/router.rs
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

// Define the routes
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // Routes that use the main layout
    #[at("/")]
    Home,
    // Fallback route
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}
