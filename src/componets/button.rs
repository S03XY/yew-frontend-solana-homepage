use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub title: String,
    pub is_gradient: Option<bool>,
    pub has_border: Option<bool>,
}

#[function_component]
pub fn ButtonComponent(props: &ButtonProps) -> Html {
    let title = props.title.clone();
    let is_gradient = props.is_gradient.unwrap().clone();

    html! {
       if is_gradient { <button class={"rounded-full p-4 px-10 text-white uppercase gr-bg "}>{title}</button>} else {
        <button class={"rounded-full p-4 px-10 text-white uppercase border"}>{title}</button>
       }
    }
}
