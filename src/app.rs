use crate::context::WGPUContext;
use log::info;
use web_sys::{console::info, Node};
use winit::{ event_loop, window::{Window, WindowBuilder}};

use yew::platform::spawn_local;
use yew::prelude::*;
use yew::{html, Callback, Component, Context, Html};
use wasm_bindgen::prelude::*;



pub enum AppMsg {
    Initializing,
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
        info!("Emitting context");
        let context = WGPUContext::new(window).await;
        context_cb.emit(context);
    });
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let canvas = NodeRef::default();
        ctx.link().send_message(AppMsg::Initializing);
        App {
            canvas: canvas,
            context: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Initialized(context) => {
                info!("Initialized");
                self.context = Some(context);
            }
            AppMsg::Initializing => {
                info!("Initializing");
                App::create_context(self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap(), ctx);
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

impl App{

    pub fn create_context(canvas: web_sys::HtmlCanvasElement, ctx: &Context<Self>){
            let event_loop = event_loop::EventLoop::new().unwrap();
            let builder = WindowBuilder::new()
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));
            #[cfg(target_arch = "wasm32")]
            let builder = {
                use winit::platform::web::WindowBuilderExtWebSys;
                builder.with_canvas(Some(canvas))
            };
            let window = builder
            .build(&event_loop)
            .unwrap();
            let context_cb = ctx.link().callback(AppMsg::Initialized);
            emit_context(window, context_cb);
    }
}