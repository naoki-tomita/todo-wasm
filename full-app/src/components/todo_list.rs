use crate::components::todo_item::TodoItem;
use crate::models::todo::{Todo, Todos};
use yew::Callback;
use yew::Html;
use yew::{html, Component, ComponentLink, Properties};

pub struct TodoList {
    props: Props,
    link: ComponentLink<Self>,
}
pub enum Msg {
    Change(usize),
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    #[props(required)]
    pub list: Todos,

    #[props(required)]
    pub ondonechange: Callback<usize>,
}

impl Component for TodoList {
    type Message = Msg;
    type Properties = Props;
    fn create(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        TodoList { props, link }
    }
    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::Change(id) => {
                self.props.ondonechange.emit(id);
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
            <div>
                <ul class="collection">
                    { for self.props.list.list.iter().map(|item| self.render_item(&item)) }
                </ul>
            </div>
        }
    }
}

impl TodoList {
    fn render_item(&self, todo: &Todo) -> Html {
        html! {
            <TodoItem item=todo ondonechange=self.link.callback(|e| Msg::Change(e)) />
        }
    }
}
