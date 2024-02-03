use crate::context::WGPUContext;
use log::info;
use winit::{event_loop, window::{self, Window, WindowBuilder}};

use yew::{context, platform::spawn_local};
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
                ctx.link().send_message(AppMsg::Redraw);
            }
            AppMsg::Initializing => {
                self.initialized = true;
                info!("Initializing");
                let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();
                canvas.set_height(500);
                canvas.set_width(500);
                App::create_context(self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap(), self);
            }
            AppMsg::Redraw => {
                info!("Redrawing");
                self.context.as_mut().unwrap().render();
                true;
            }
            AppMsg::Nothing => {
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // if self.initialized == true {
        //     ctx.link().send_message(AppMsg::Redraw);
        // }
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

    fn emit_context(window: Window, height: u32, width: u32, context_cb: Callback<WGPUContext>) {
        spawn_local(async move {
            info!("Emitting context");
            let context = WGPUContext::new(window, height, width).await;
            context_cb.emit(context);
        });
    }

    pub fn create_context(canvas: web_sys::HtmlCanvasElement, ctx: &mut App) {
        let height = canvas.height();
        let width = canvas.width();
        let event_loop = event_loop::EventLoop::new().unwrap();
        let builder = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(height, width));
        #[cfg(target_arch = "wasm32")]
        let builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            builder.with_canvas(Some(canvas))
        };
        let window = builder
        .build(&event_loop)
        .unwrap();
        let cb = ctx.callback.clone();
        App::emit_context(window, height, width, cb);
    }

    pub fn render(&self) {
        info!("Rendering");
    }
}