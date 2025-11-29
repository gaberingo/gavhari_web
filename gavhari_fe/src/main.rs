mod api;
mod routes;

fn main() {
    yew::Renderer::<routes::App>::new().render();
}
