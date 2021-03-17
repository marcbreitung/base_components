use crate::components::color_scheme::{create_default_color_scheme, ColorScheme};
use yew::prelude::*;

pub enum Msg {
    Close,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    #[prop_or_else(create_default_color_scheme)]
    pub color_scheme: ColorScheme,
}

pub struct Message {
    text: String,
    visible: bool,
    link: ComponentLink<Self>,
    color_scheme: ColorScheme,
}

impl Message {
    pub fn element_class(&self) -> Vec<&str> {
        let mut class = vec![
            "p-5",
            "m-5",
            "shadow-sm",
            "text-white",
            "rounded",
            "flex",
            "justify-between",
            "content-center",
        ];
        let mut class_input = vec!["bg-green-700"];
        let mut class_error = vec!["bg-red-700"];
        let mut class_readonly = vec!["bg-gray-700"];
        self.build_class_list(
            &mut class,
            &mut class_input,
            &mut class_error,
            &mut class_readonly,
        )
    }

    pub fn build_class_list<'a>(
        &self,
        base: &mut Vec<&'a str>,
        input: &mut Vec<&'a str>,
        error: &mut Vec<&'a str>,
        readonly: &mut Vec<&'a str>,
    ) -> Vec<&'a str> {
        match self.color_scheme {
            ColorScheme::Input => {
                base.append(input);
                base.to_vec()
            }
            ColorScheme::Error => {
                base.append(error);
                base.to_vec()
            }
            ColorScheme::Readonly => {
                base.append(readonly);
                base.to_vec()
            }
        }
    }
}

impl Component for Message {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            text: props.text,
            visible: true,
            link,
            color_scheme: props.color_scheme,
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
        self.color_scheme = props.color_scheme;
        true
    }

    fn view(&self) -> Html {
        if self.visible {
            let onclose = self.link.callback(|_| Msg::Close);
            html! {
            <div class=self.element_class()>
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
