use yew::prelude::*;

pub enum Msg {
    Click,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
}

pub struct Button {
    link: ComponentLink<Self>,
    label: String,
    onclick: Option<Callback<()>>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            label: props.label,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                if let Some(onclick) = &self.onclick {
                    onclick.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        self.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Click);
        let button_class = vec![
            "border",
            "shadow-sm",
            "text-white",
            "text-sm",
            "rounded",
            "p-2",
            "px-4",
            "bg-indigo-700",
        ];

        html! {
             <button onclick=onclick class=button_class>
                    <span class="hidden md:block">
                         {&self.label}
                    </span>
             </button>
        }
    }
}
