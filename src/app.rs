use crate::context::WGPUContext;

use winit::{
    event::WindowEvent,
    event_loop,
    window::{Window, WindowBuilder},
};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{html, AttrValue, Callback, Component, Context, ContextProvider, Html};

pub enum AppMsg {
    Initializing(WGPUContext),
    Initialized(WGPUContext),
    Redraw,
    Nothing,
}

#[derive(PartialEq, Properties, Default)]
pub struct AppProperties {}

pub struct App {
    canvas: NodeRef,
    context: Option<WGPUContext>,
}

fn emit_context(window: Window, context_cb: Callback<WGPUContext>) {
    spawn_local(async move {
        let context = WGPUContext::new(window).await;
        context_cb.emit(context);
    });
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AppMsg::Redraw);
        let event_loop = event_loop::EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap();
        let context_cb = ctx.link().callback(AppMsg::Initializing);
        emit_context(window, context_cb);
        App {
            canvas: NodeRef::default(),
            context: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Initialized(context) => {
                self.context = Some(context);
            }
            AppMsg::Initializing(_) => {
                return false;
            }
            AppMsg::Redraw => {
                
            }
            AppMsg::Nothing => {
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <div>
              <canvas ref = {self.canvas.clone()}/>
            </div>
        )
    }
}
