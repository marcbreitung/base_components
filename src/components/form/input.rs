use yew::prelude::*;
use crate::components::color_scheme::{ColorScheme, create_default_color_scheme};

pub enum Msg {
    UpdateValue(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or(false)]
    pub show_message: bool,
    #[prop_or_default]
    pub message: String,
    #[prop_or_else(create_default_color_scheme)]
    pub color_scheme: ColorScheme,
    pub onupdate: Callback<String>,
}

pub struct Input {
    link: ComponentLink<Self>,
    value: String,
    label: String,
    message: String,
    show_message: bool,
    color_scheme: ColorScheme,
    onupdate: Callback<String>,
}

impl Input {
    pub fn element_class(&self) -> Vec<&str> {
        let mut class = vec!["text-sm"];
        let mut class_input = vec!["text-gray-700"];
        let mut class_error = vec!["text-red-700"];
        let mut class_readonly = vec!["text-gray-300"];
        self.build_class_list(
            &mut class,
            &mut class_input,
            &mut class_error,
            &mut class_readonly,
        )
    }

    pub fn label_class(&self) -> Vec<&str> {
        let mut class = vec!["block", "font-medium"];
        let mut class_input = vec![];
        let mut class_error = vec![];
        let mut class_readonly = vec![];
        self.build_class_list(
            &mut class,
            &mut class_input,
            &mut class_error,
            &mut class_readonly,
        )
    }

    pub fn input_class(&self) -> Vec<&str> {
        let mut class = vec!["mt-1", "p-2", "block", "w-full", "border"];
        let mut class_input = vec!["border-gray-700"];
        let mut class_error = vec!["border-red-700"];
        let mut class_readonly = vec!["border-gray-300"];
        self.build_class_list(
            &mut class,
            &mut class_input,
            &mut class_error,
            &mut class_readonly,
        )
    }

    pub fn message_class(&self) -> Vec<&str> {
        let mut class = vec!["mt-1"];
        let mut class_input = vec!["text-gray-700"];
        let mut class_error = vec!["text-red-700"];
        let mut class_readonly = vec!["text-gray-300"];
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

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value,
            label: props.label,
            message: props.message,
            show_message: props.show_message,
            color_scheme: props.color_scheme,
            onupdate: props.onupdate,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(value) => {
                self.onupdate.emit(value);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.label = props.label;
        self.onupdate = props.onupdate;
        true
    }

    fn view(&self) -> Html {
        let onupdate = self
            .link
            .callback(|ev: InputData| Msg::UpdateValue(ev.value));

        html! {
            <div class=self.element_class()>
                <label class=self.label_class()>{ &self.label }</label>
                <input type="text" value=&self.value oninput=onupdate class=self.input_class()/>
                {
                    if self.show_message && !self.message.is_empty() {
                        html! {
                            <p class=self.message_class()>{ &self.message }</p>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
