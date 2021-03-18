#![recursion_limit = "1024"]

mod components;

use yew::prelude::*;

use components::color_scheme::ColorScheme;
use components::form::button;
use components::form::input;
use components::form::select;
use components::message::message;

enum Msg {
    UpdateValue(String),
    Submit,
}

struct App {
    link: ComponentLink<Self>,
    input_value: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_value: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(value) => {
                self.input_value = value;
            }
            Msg::Submit => {
                self.input_value = "".to_owned();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onupdate = &self.link.callback(|value: String| Msg::UpdateValue(value));
        let onclick = Some(self.link.callback(|_| Msg::Submit));
        html! {
            <div class="flex-1 gb-gray-50">
                <div class="p-5 m-5 bg-white shadow-sm rounded">

                        <message::Message text="Error".to_owned() color_scheme=ColorScheme::Input/>
                        <message::Message text="Error".to_owned() color_scheme=ColorScheme::Error/>
                        <message::Message text="Error".to_owned() color_scheme=ColorScheme::Readonly/>

                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Input show_message=true/>
                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Error show_message=true/>
                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Readonly show_message=true/>

                        <select::Select
                            label="Name".to_owned()
                            options=vec![("one".to_owned(), "Label one".to_owned()),("two".to_owned(), "Label two".to_owned())]
                            value="two".to_owned()
                            onupdate=onupdate
                            message="Please enter a value".to_owned()
                            color_scheme=ColorScheme::Input
                            show_message=true/>

                        <button::Button label="Submit".to_owned() onclick=onclick/>
                        <button::Button label="Submit".to_owned()/>

                    <div class="col-start-1 col-end-3">
                        <div>{&self.input_value}</div>
                    </div>

                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
