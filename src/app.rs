use crate::context::WGPUContext;
use log::info;
use winit::{event_loop, window::{self, Window, WindowBuilder}};

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
    initialized: bool,
}



impl Component for App {
    type Message = AppMsg;
    type Properties = AppProperties;

    fn create(ctx: &Context<Self>) -> Self {
        info!("Creating App");
        let canvas = NodeRef::default();
        let context_cb: Callback<WGPUContext> = ctx.link().callback(AppMsg::Initialized);
        App {
            canvas: canvas,
            context: None,
            callback: context_cb,
            initialized: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Initialized(context) => {
                info!("Initialized");
                self.context = Some(context);
            }
            AppMsg::Initializing => {
                self.initialized = true;
                info!("Initializing");
                App::create_context(self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap(), self);
            }
            AppMsg::Redraw => {
                false;
            }
            AppMsg::Nothing => {
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.initialized != true {
            ctx.link().send_message(AppMsg::Initializing);
        }
        html! (
            <div>
              <canvas ref = {self.canvas.clone()}/>
            </div>
        )
    }
}

#[wasm_bindgen]
impl App{

    fn emit_context(window: Window, context_cb: Callback<WGPUContext>) {
        spawn_local(async move {
            info!("Emitting context");
            let context = WGPUContext::new(window).await;
            context_cb.emit(context);
        });
    }

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
        #[cfg(target_arch = "wasm32")]{
            use winit::platform::web::WindowExtWebSys;
            info!("Window: {:?}", window.inner_size());
        }
        info!("Window: {:?}", window.inner_size());
        let cb = ctx.callback.clone();
        App::emit_context(window, cb);
    }

    pub fn render(&self) {
        info!("Rendering");
    }
}