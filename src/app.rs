use crate::context::WGPUContext;

use yew::{html, Callback, Component, Context, ContextProvider, Html};
use yew::prelude::*;

use winit::{
    event::WindowEvent, event_loop, window::{Window, WindowBuilder}
};


pub enum AppMsg {
    Redraw,
    Nothing
}

#[derive(PartialEq, Properties, Default)]
pub struct AppProperties {}

pub struct App {
    canvas: NodeRef,
    context: WGPUContext,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AppMsg:: Redraw);
        let event_loop = event_loop::EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();
        let context = WGPUContext::new(window);
        App {
          canvas : NodeRef::default(),
          context
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
        // match msg {
            
        // }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <div>
              <canvas ref = {self.canvas.clone()}/>
            </div>
        )
    }
}
