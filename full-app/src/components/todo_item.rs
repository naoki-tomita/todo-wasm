use crate::models::todo::Todo;
use yew::{html, Component, ComponentLink, Properties, Html,Callback};
use yew::services::ConsoleService;

pub enum Msg {
    Change,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub item: Todo,
    pub index: usize,
    #[props(required)]
    pub ondonechange: Callback<usize>,
}

pub struct TodoItem {
    props: Props,
    link: ComponentLink<Self>,
}
impl Component for TodoItem {
    type Message = Msg;
    type Properties = Props;
    fn create(
        props: Props,
        link: ComponentLink<Self>,
    ) -> Self {
        TodoItem { props, link }
    }
    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Change => {
                self.props.ondonechange.emit(self.props.index);
                true
            }
        }
    }
    fn change(&mut self, props: Props) -> bool {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        html! {
            <li>
                { &self.props.item.text }
                <input type="checkbox" checked=self.props.item.done onclick=self.link.callback(|_| Msg::Change) />
            </li>
        }
    }
}
