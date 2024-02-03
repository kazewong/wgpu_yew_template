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


#[wasm_bindgen]
pub struct App {
    canvas: NodeRef,
    context: Option<WGPUContext>,
    callback: Callback<WGPUContext>,
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
        let context_cb: Callback<WGPUContext> = ctx.link().callback(AppMsg::Initialized);
        App {
            canvas: canvas,
            context: None,
            callback: context_cb,
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
                let context_cb: Callback<WGPUContext> = ctx.link().callback(AppMsg::Initialized);
                App::create_context(self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap(), self);
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

#[wasm_bindgen]
impl App{

    pub fn create_context(canvas: web_sys::HtmlCanvasElement, ctx: &mut App) {
        let event_loop = event_loop::EventLoop::new().unwrap();
        let builder = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));
        #[cfg(target_arch = "wasm32")]
        let builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            info!("Canvas: {:?}", canvas.height());
            builder.with_canvas(Some(canvas))
        };
        let window = builder
        .build(&event_loop)
        .unwrap();
        let cb = ctx.callback.clone();
        emit_context(window, cb);
    }
}