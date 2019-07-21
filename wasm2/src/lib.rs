extern crate grid_2d;
extern crate js_sys;
extern crate prototty_grid;
pub extern crate prototty_input;
pub extern crate prototty_render;
extern crate wasm_bindgen;
extern crate web_sys;

use grid_2d::Coord;
pub use grid_2d::Size;
use js_sys::Function;
use prototty_input::Input;
use prototty_render::{Rgb24, View, ViewContext, ViewContextDefault, ViewTransformRgb24};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, Node, Performance, WheelEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct WebColourConversion;
impl prototty_grid::ColourConversion for WebColourConversion {
    type Colour = String;
    fn convert_foreground_rgb24(&mut self, Rgb24 { r, g, b }: Rgb24) -> Self::Colour {
        format!("rgb({},{},{})", r, g, b)
    }
    fn convert_background_rgb24(&mut self, Rgb24 { r, g, b }: Rgb24) -> Self::Colour {
        format!("rgb({},{},{})", r, g, b)
    }
    fn default_foreground(&mut self) -> Self::Colour {
        "rgb(255,255,255)".to_string()
    }
    fn default_background(&mut self) -> Self::Colour {
        "rgb(0,0,0)".to_string()
    }
}

struct ElementCell {
    element: HtmlElement,
}

pub struct Context {
    element_grid: grid_2d::Grid<ElementCell>,
    prototty_grid: prototty_grid::Grid<WebColourConversion>,
}

impl Context {
    pub fn new(size: Size, container: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container_node = document
            .get_element_by_id(container)
            .unwrap()
            .dyn_into::<Node>()
            .unwrap();
        let element_grid = grid_2d::Grid::new_fn(size, |coord| {
            let element = document
                .create_element("span")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            ElementCell { element }
        });
        for y in 0..size.height() {
            for x in 0..size.width() {
                container_node
                    .append_child(&element_grid.get_checked(Coord::new(x as i32, y as i32)).element)
                    .unwrap();
            }
            container_node
                .append_child(document.create_element("br").unwrap().dyn_ref::<HtmlElement>().unwrap())
                .unwrap();
        }
        let prototty_grid = prototty_grid::Grid::new(size, WebColourConversion);
        let style_text = format!(
            "#{} {{
                font-family: monospace;
                font-size: 24px;
            }}",
            container
        );
        let style_element = document
            .create_element("style")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        style_element.set_inner_text(&style_text);
        document.head().unwrap().append_child(&style_element).unwrap();
        Self {
            element_grid,
            prototty_grid,
        }
    }

    pub fn default_context(&self) -> ViewContextDefault {
        ViewContext::default_with_size(self.prototty_grid.size())
    }

    fn render_internal(&mut self) {
        for (prototty_cell, element_cell) in self.prototty_grid.iter().zip(self.element_grid.iter_mut()) {
            let string = match prototty_cell.character {
                ' ' => "&nbsp;".to_string(),
                other => other.to_string(),
            };
            element_cell.element.set_inner_html(&string);
            let element_style = element_cell.element.style();
            element_style
                .set_property("color", &prototty_cell.foreground_colour)
                .unwrap();
            element_style
                .set_property("background-color", &prototty_cell.background_colour)
                .unwrap();
            if prototty_cell.underline {
                element_style.set_property("text-decoration", "underline").unwrap();
            } else {
                element_style.remove_property("text-decoration").unwrap();
            }
            if prototty_cell.bold {
                element_style.set_property("font-weight", "bold").unwrap();
            } else {
                element_style.remove_property("font-weight").unwrap();
            }
        }
    }

    pub fn render_at<V: View<T>, T, R: ViewTransformRgb24>(&mut self, view: &mut V, data: T, context: ViewContext<R>) {
        self.prototty_grid.clear();
        view.view(data, context, &mut self.prototty_grid);
        self.render_internal();
    }

    pub fn render<V: View<T>, T>(&mut self, view: &mut V, data: T) {
        let context = self.default_context();
        self.render_at(view, data, context);
    }
}

pub trait EventHandler {
    fn on_input(&mut self, input: Input, context: &mut Context);
    fn on_frame(&mut self, since_last_frame: Duration, context: &mut Context);
}

pub fn run_event_handler<E: EventHandler + 'static>(mut event_handler: E, mut context: Context) {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    let f: Rc<RefCell<Option<Closure<_>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut last_frame_time_stamp = performance.now();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let frame_time_stamp = performance.now();
        let since_last_frame = frame_time_stamp - last_frame_time_stamp;
        last_frame_time_stamp = frame_time_stamp;
        event_handler.on_frame(Duration::from_millis(since_last_frame as u64), &mut context);
        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));
    g.borrow()
        .as_ref()
        .unwrap()
        .as_ref()
        .unchecked_ref::<Function>()
        .call0(&JsValue::NULL)
        .unwrap();
}