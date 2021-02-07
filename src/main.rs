#![recursion_limit = "256"]

mod components;

use components::color_scheme::ColorScheme;
use components::form::button;
use components::form::input;
use yew::prelude::*;

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
        let onclick = &self.link.callback(|_| Msg::Submit);
        html! {
            <div class="container mx-auto">
                <div class="grid grid-cols-6 gap-5">
                    <div class="col-start-1 col-end-3">
                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Input show_message=true/>
                    </div>
                    <div class="col-start-1 col-end-3">
                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Error show_message=true/>
                    </div>
                    <div class="col-start-1 col-end-3">
                        <input::Input label="Name".to_owned() value=&self.input_value.to_owned() onupdate=onupdate message="Please enter a value".to_owned() color_scheme=ColorScheme::Readonly show_message=true/>
                    </div>
                    <div class="col-start-1 col-end-3">
                        <button::Button label="Submit".to_owned() onclick=onclick/>
                    </div>
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
