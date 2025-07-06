mod components;
mod pages;
mod router;

fn main() {
    yew::Renderer::<router::App>::new().render();
}
