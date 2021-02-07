use yew::prelude::*;

pub enum Msg {
    Click,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    pub onclick: Callback<()>,
}

pub struct Button {
    link: ComponentLink<Self>,
    label: String,
    onclick: Callback<()>,
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
               self.onclick.emit(());
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
       let button_class = vec!["inline-flex", "itmes-center", "justify-center", "bg-gray-700", "p-5", "pt-2", "pb-2", "text-white"];

       html! {
            <button onclick=onclick class=button_class>{&self.label}</button>
       }
   }
}
