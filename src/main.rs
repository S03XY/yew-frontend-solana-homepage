use yew::prelude::*;


mod componets;
mod constants;


use componets::navbar::Navbar;
use componets::title::TitleComponent;
// use componets::button::ButtonComponent;
use constants::links::HOME_NAVIGATION_LINKS;

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <Navbar links={Vec::from(HOME_NAVIGATION_LINKS.map(|value| {value.to_string()}))}/>
        <TitleComponent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// all the styles should be inside the src and there are few assets types that is supported in yew
// class would also work and classes! would also work
