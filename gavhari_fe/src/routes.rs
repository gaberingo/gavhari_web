use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/web")]
    Web,
}

fn main_switch(route: Route) -> Html {
    match route {
        Route::Home => html! {"Home"},
        Route::Web => html! {"Web"},
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
       <BrowserRouter>
            <Switch<Route> render={main_switch}/>
        </BrowserRouter>
    }
}
