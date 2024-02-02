use crate::context::WGPUContext;
use winit::{
    event_loop,
    window::{Window, WindowBuilder},
};

use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{html, Callback, Component, Context, Html};
use wasm_bindgen::prelude::*;

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
        App {
            canvas: NodeRef::default(),
            context: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Initialized(context) => {
                self.context = Some(context);
            }
            AppMsg::Initializing(_) => {
                self.create_context(self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap(), ctx);
            }
            AppMsg::Redraw => {}
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

#[wasm_bindgen(start)]
impl App{

    fn create_context(canvas: web_sys::HtmlCanvasElement, ctx: &Context<Self>){
        let event_loop = event_loop::EventLoop::new().unwrap();
        #[cfg(target_arch = "wasm32")]{
            use winit::platform::web::WindowExtWebSys;
            let window = WindowBuilder::new()
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
                .with_canvas(canvas)
                .build(&event_loop)
                .unwrap();
            let context_cb = ctx.link().callback(AppMsg::Initializing);
            emit_context(window, context_cb);
        }

    }
}