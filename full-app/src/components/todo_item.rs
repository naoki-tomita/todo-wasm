use crate::models::todo::Todo;
use yew::{html, Callback, Component, ComponentLink, Html, Properties};

pub enum Msg {
    Change,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub item: Todo,
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
    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        TodoItem { props, link }
    }
    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Change => {
                self.props.ondonechange.emit(self.props.item.id);
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
            <li class="collection-item">
                <label>
                    <input type="checkbox" checked=self.props.item.done onclick=self.link.callback(|_| Msg::Change) />
                    <span>{ &self.props.item.text }</span>
                </label>
            </li>
        }
    }
}
