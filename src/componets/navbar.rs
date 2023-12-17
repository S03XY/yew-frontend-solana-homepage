use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {
    pub links: Vec<String>,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let urls = props.links.clone();

    html! {
        <div class="bg-black py-6 px-[150px] text-white flex justify-between items-center backdrop-blur-sm sticky top-0">
            <div class="flex justify-center items-center space-x-2">
            <div class="h-[24px] w-[24px]">
            <img src="assets/solana-logo.svg" alt="logo"  class="h-full w-full object-contain"/>
            </div>
            <p class="font-poppins font-normal text-xl uppercase tracking-wider">{"Solana"}</p>
            </div>
            <div class="flex justify-end space-x-8">
                {
                    urls.iter().map(|value| {
                        html! {
                            <div class="font-poppins text-white/50" key={value.clone()}>{value}</div>
                        }
                    }).collect::<Html>() 
                }

            </div>
        </div>
    }
}
