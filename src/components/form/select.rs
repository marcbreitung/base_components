use crate::components::color_scheme::{create_default_color_scheme, ColorScheme};
use yew::prelude::*;

pub enum Msg {
    UpdateValue(ChangeData),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub options: Vec<(String, String)>,
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

pub struct Select {
    link: ComponentLink<Self>,
    options: Vec<(String, String)>,
    value: String,
    label: String,
    message: String,
    show_message: bool,
    color_scheme: ColorScheme,
    onupdate: Callback<String>,
}

impl Select {
    pub fn element_class(&self) -> Vec<&str> {
        let mut class = vec![
            "p-2.5",
            "grid",
            "grid-cols-6",
            "gap-5",
            "place-content-center",
            "text-sm",
        ];
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
        let mut class = vec!["w-40", "block", "p-2"];
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
        let mut class = vec!["border", "text-gray-500", "rounded", "col-span-2", "p-2"];
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
        let mut class = vec!["p-2"];
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
    fn get_options(&self) -> Html {
        html! { self.options.iter().map(|(value, label)| {
            if &self.value == value {
                html! { <option value={value} selected={true}>{label}</option> }
            } else {
                html! { <option value={value}>{label}</option> }
            }
        }).collect::<Html>() }
    }
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            options: props.options,
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
            Msg::UpdateValue(value) => match value {
                ChangeData::Select(value) => {
                    self.onupdate.emit(value.value());
                }
                _ => {}
            },
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.options = props.options;
        self.value = props.value;
        self.label = props.label;
        self.onupdate = props.onupdate;
        true
    }

    fn view(&self) -> Html {
        let onchange = self.link.callback(|ev: ChangeData| Msg::UpdateValue(ev));

        html! {
            <div class=self.element_class()>
                <label class=self.label_class()>{ &self.label }</label>
                <select class=self.input_class() onchange=onchange>{self.get_options()}</select>
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
