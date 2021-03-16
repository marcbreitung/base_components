use yew::prelude::*;

pub enum Msg {
    Close,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
}

pub struct Message {
    text: String,
    visible: bool,
    link: ComponentLink<Self>,
}

impl Component for Message {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            text: props.text,
            visible: true,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.visible = false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.text = props.text;
        true
    }

    fn view(&self) -> Html {
        if self.visible {
            let onclose = self.link.callback(|_| Msg::Close);
        html! {
        <div class="p-5 m-5 bg-green-300 shadow-sm rounded flex justify-between content-center">
            <div>{&self.text}</div>
                <button class="flex w-4 h-4" onclick=onclose>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-4 h-4">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
        </div>
        }
        } else {
            html! {}
        }
    }

}
