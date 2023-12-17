use yew::prelude::*;

use super::button::ButtonComponent;

#[function_component]
pub fn TitleComponent() -> Html {
    html! {
        <>
        <div class="text-white font-poppin font-bold tracking-wide mx-auto text-6xl flex-1  mt-[20vh] p-4">
        <h1 class="w-fit mx-auto" >{format!("Powerful for developers.")}</h1>
        <h1 class="w-fit mx-auto">{"Fast for everyone."}</h1>
        </div>
        <div class="text-white text-center mt-14">
        <p class="font-medium text-lg max-w-[600px] mx-auto">
        {"Bring blockchain to the people. Solana supports experiences for power users, new consumers, and everyone in between."}
        </p>
        </div>
        <div class="flex justify-center items-center space-x-4 mt-14">
            <ButtonComponent title={"Start Building"} is_gradient={Some(true)} has_border={Some(false)} />
            <ButtonComponent title={"Read Docs"} is_gradient={Some(false)} has_border={Some(true)} />
        </div>
        </>
    }
}
