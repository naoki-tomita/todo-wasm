use yew::services::ConsoleService;
use yew::{Component, ComponentLink, html, InputData, Properties, Callback};

pub struct TodoInput {
    text: String,
    link: ComponentLink<Self>,
    console: ConsoleService,
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
    pub oncomplete: Callback<String>
}


impl Component for TodoInput {
    type Message = Msg;
    type Properties = Props;
    fn create(
        props: Props,
        link: yew::html::ComponentLink<Self>,
    ) -> Self {
        TodoInput { link, text: "".to_string(), console: ConsoleService::new(), props }
    }
    fn update(&mut self, message: <Self as yew::html::Component>::Message) -> bool {
        self.console.log(format!("{:?}", message).as_str());
        match message {
            Msg::Input(text) => {
                self.text = text;
                true
            },
            Msg::Complete => {
                self.props.oncomplete.emit(self.text.clone());
                true
            }
        }
    }
    fn view(&self) -> yew::virtual_dom::vnode::VNode {
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
