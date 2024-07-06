use std::cell::RefCell;

use rand::{thread_rng, Rng};
use wasm_bindgen::{closure, prelude::*};
use web_sys::{window, Document, HtmlElement, HtmlInputElement, MouseEvent, WebGlProgram, WebGlRenderingContext, Window};
use euclid::{self, default, Box2D};

extern crate js_sys;

use crate::utils::default_gl;

// define the state
#[derive(Clone)]
struct STATE {
    rects: Vec<Box2D<f64, f64>>,
    x: f32,
    c: f32,
    moving: bool,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        rects: vec![
            Box2D::new(euclid::point2(0.7, 0.3), euclid::point2(-0.5, -0.5)),
            Box2D::new(euclid::point2(0.0, 0.0), euclid::point2(0.5, 0.5)),
        ],
        x: 0.0,
        c: 0.1,
        moving: false,
    });
}

#[wasm_bindgen]
pub fn user_init() {
    
    let document = window().unwrap().document().unwrap();

    // add moving checkbox
    let moving_input: HtmlInputElement = document.get_element_by_id("move").unwrap().dyn_into().unwrap();
    let moving_read = moving_input.clone();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        web_sys::console::log_1(&moving_read.checked().into());
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.moving = moving_read.checked();
        });
    }));
    moving_input.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    // add shake slider
    let shake_input: HtmlInputElement = document.get_element_by_id("shake").unwrap().dyn_into().unwrap();
    let shake_output: HtmlElement = document.get_element_by_id("shake_output").unwrap().dyn_into().unwrap();
    let shake_read = shake_input.clone();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.c = shake_read.value().parse::<f32>().unwrap() / 10.0;
        });
        shake_output.set_inner_html(&("shake: ".to_owned() + &shake_read.value()));
    }));
    shake_input.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    // do mouse input stuff
    let canvas: HtmlElement = document.get_element_by_id("user_input").unwrap().dyn_into().unwrap();
    let bounding_rect = canvas.get_bounding_client_rect();

    let closure: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |event: MouseEvent| {
        
        let mouse_x = -1.0 + 2.0 * (event.client_x() as f64 - bounding_rect.x()) / bounding_rect.width();
        let mouse_y = -1.0 + 2.0 * (event.client_y() as f64 - bounding_rect.y()) / bounding_rect.height();

        // add a new rect at this position
        let new_rect = Box2D::new(
            euclid::point2(mouse_x, -mouse_y), 
            euclid::point2(mouse_x + 0.05, -mouse_y + 0.05)
        );
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.rects.push(new_rect);
        });
    }));

    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();

    closure.forget(); // Keep the closure alive

    // start animation loop
    user_draw(default_gl());

}


fn user_draw(gl: WebGlRenderingContext) {

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    let mut rng = thread_rng();
    let mut i = 0;

    STATE.with(|state| {

        let mut state = state.borrow_mut();
        state.x += 0.05;

        // draw rects
        for rect in state.rects.iter() {
            i += 1;
            let bottom_left: euclid::Point2D<f64, f64> = rect.min;
            let top_right: euclid::Point2D<f64, f64> = rect.max;
            let top_left: euclid::Point2D<f64, f64> = euclid::point2(bottom_left.x, top_right.y);
            let bottom_right: euclid::Point2D<f64, f64> = euclid::point2(top_right.x, bottom_left.y);

            let mut data = vec![
                top_left.x as f32, top_left.y as f32, 1.0, 0.0, 0.0,
                bottom_left.x as f32, bottom_left.y as f32, 0.0, 1.0, 0.0,
                bottom_right.x as f32, bottom_right.y as f32, 0.0, 0.0, 1.0,
                top_right.x as f32, top_right.y as f32, 1.0, 1.0, 1.0,
            ];
            
            if state.moving {
                data = data.iter().map(|x| 
                    *x 
                    + ((state.x + rng.gen_range(0.0..state.c)).sin() / 5.0)
                    * if i % 2 == 0 { 1.0 } else { -1.0 }
                    - 0.1
                ).collect::<Vec<f32>>();
            }

            // fill ARRAY_BUFFER with the vertex data
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &(unsafe { js_sys::Float32Array::view(&data).into() }),
                WebGlRenderingContext::STATIC_DRAW,
            );
        
            // draw on the screen
            gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
        }
        

        // request for another animation frame, with changes to state if needed
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                user_draw(gl.clone());
        }));
        web_sys::window().unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();

    });
}
