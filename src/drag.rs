use std::cell::{RefCell, RefMut};

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MouseEvent, WebGlRenderingContext};
use euclid::{self, Rect};

extern crate js_sys;

use crate::utils::default_gl;

#[derive(Clone)]
struct Space {
    verticies: Vec<Draggable>,
}

#[derive(Clone)]
struct Draggable {
    rect: Rect<f32, f32>,
    colour: [f32; 3],
}

// define the state
#[derive(Clone)]
struct STATE {
    spaces: Vec<Space>,
    mouse_pos: euclid::Point2D<f32, f32>,
    mouse_down: bool,
    mouse_cursor: Draggable,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        spaces: vec![
            Space {
                verticies: vec![
                    Draggable {
                        rect: Rect::new(euclid::point2(-0.5, -0.5), euclid::size2(0.1, 0.1)),
                        colour: [1.0, 1.0, 0.0],
                    },
                    Draggable {
                        rect: Rect::new(euclid::point2(0.5, 0.5), euclid::size2(0.1, 0.1)),
                        colour: [0.0, 1.0, 1.0],
                    },
                    Draggable {
                        rect: Rect::new(euclid::point2(-0.5, 0.5), euclid::size2(0.1, 0.1)),
                        colour: [1.0, 0.0, 1.0],
                    },
                    Draggable {
                        rect: Rect::new(euclid::point2(0.5, -0.5), euclid::size2(0.1, 0.1)),
                        colour: [0.0, 1.0, 0.0],
                    },
                ],
            },
        ],
        mouse_cursor: Draggable {
            rect: Rect::new(euclid::point2(0.0, 0.0), euclid::size2(0.1, 0.1)),
            colour: [1.0, 0.0, 0.0],
        },
        mouse_pos: euclid::point2(0.0, 0.0),
        mouse_down: false,
    });

}

#[wasm_bindgen]
pub fn drag_init() {
    
    let document = window().unwrap().document().unwrap();


    // do mouse input stuff
    let canvas: HtmlElement = document.get_element_by_id("user_input").unwrap().dyn_into().unwrap();
    let bounding_rect = canvas.get_bounding_client_rect();

    // mousemove
    let closure: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |event: MouseEvent| {
    
        // update mouse position
        let mouse_pos: euclid::Point2D<f32, f32> = euclid::point2(
            -1.0 + 2.0 * (event.client_x() as f64 - bounding_rect.x()) as f32 / bounding_rect.width() as f32,
            1.0 + -2.0 * (event.client_y() as f64 - bounding_rect.y()) as f32 / bounding_rect.height() as f32
        );

        // give state this knoeledge
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.mouse_pos = mouse_pos;
            state.mouse_down = event.buttons() == 1;
            state.mouse_cursor.rect.origin = mouse_pos - euclid::vec2(0.05, 0.05);
        });

    }));
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); // Keep the closure alive

    // start animation loop
    user_draw(default_gl());

}


fn draw_draggable(gl: WebGlRenderingContext, d: Draggable) {

    let rect = d.rect;

    let top_left = rect.origin;
    let bottom_left = rect.origin + euclid::vec2(rect.size.width, 0.0);
    let bottom_right = rect.origin + rect.size;
    let top_right = rect.origin + euclid::vec2(0.0, rect.size.height);

    let data = vec![
        top_left.x as f32, top_left.y as f32, d.colour[0], d.colour[1], d.colour[2],
        bottom_left.x as f32, bottom_left.y as f32, d.colour[0], d.colour[1], d.colour[2],
        bottom_right.x as f32, bottom_right.y as f32, d.colour[0], d.colour[1], d.colour[2],
        top_right.x as f32, top_right.y as f32, d.colour[0], d.colour[1], d.colour[2], 
    ];

    // fill ARRAY_BUFFER with the vertex data
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &(unsafe { js_sys::Float32Array::view(&data).into() }),
        WebGlRenderingContext::STATIC_DRAW,
    );
    gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
}


fn user_draw(gl: WebGlRenderingContext) {

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    STATE.with(|state| {

        let state: RefMut<STATE> = state.borrow_mut();

        // draw mouse cursor 
        draw_draggable(gl.clone(), state.mouse_cursor.clone());

        // draw spaces
        for space in state.spaces.iter() {
            for draggable in space.verticies.iter() {
                draw_draggable(gl.clone(), draggable.clone());
            }
        }
    });

    // request for another animation frame
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || { user_draw(gl.clone()); }));
    web_sys::window().unwrap().request_animation_frame(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

}
