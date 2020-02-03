use yew::Html;
use yew::services::ConsoleService;
use yew::{html, Callback, Component, ComponentLink, InputData, Properties};

pub struct TodoInput {
    text: String,
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug)]
pub enum Msg {
    Input(String),
    Complete,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub oncomplete: Callback<String>,
}

impl Component for TodoInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: yew::html::ComponentLink<Self>) -> Self {
        TodoInput {
            link,
            text: "".to_string(),
            props,
        }
    }

    fn update(&mut self, message: <Self as yew::html::Component>::Message) -> bool {
        match message {
            Msg::Input(text) => {
                self.text = text;
                true
            }
            Msg::Complete => {
                self.props.oncomplete.emit(self.text.clone());
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value=self.text
                    oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                />
                <button
                    onclick=self.link.callback(|_| Msg::Complete)
                >{"Add todo"}</button>
            </div>
        }
    }
}
