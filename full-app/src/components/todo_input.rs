use yew::Html;
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
                if self.text.is_empty() {
                    return false;
                }
                self.props.oncomplete.emit(self.text.clone());
                self.text = "".to_string();
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card" style="display: flex;">
                <input type="text" style="width: 70%"
                    value=self.text
                    oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                />
                <button type="button" style="width: 30%; margin-left: 12px" onclick=self.link.callback(|_| Msg::Complete)
                >{"Create"}</button>
            </div>
        }
    }
}
