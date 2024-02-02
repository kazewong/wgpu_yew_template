use yew::{html, Callback, Component, Context, ContextProvider, Html};
use yew::prelude::*;


pub enum AppMsg {
    Redraw,
    Nothing
}

#[derive(PartialEq, Properties)]
pub struct AppProperties {}

pub struct App {
    canvas: NodeRef,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        ctx.link().send_message(App:: Redraw);
        App {
          canvas : NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <div>
              <canvas ref = {self.canvas.clone()}/>
            </div>
        )
    }
}
