use std::cell::{RefCell, RefMut};

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MouseEvent, WebGlRenderingContext};
use euclid::{self, Rect};

extern crate js_sys;

use crate::utils::default_gl;

#[derive(Clone)]
struct Draggable {
    rect: Rect<f32, f32>,
    colour: [f32; 3],
}

// define the state
#[derive(Clone)]
struct STATE {
    spaces: Vec<Draggable>,
    mouse_pos: euclid::Point2D<f32, f32>,
    mouse_down: bool,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        spaces: vec![
            Draggable {
                rect: Rect::new(euclid::point2(0.0, 0.0), euclid::size2(0.2, 0.2)),
                colour: [1.0, 0.0, 0.0],
            },
            Draggable {
                rect: Rect::new(euclid::point2(0.2, 0.2), euclid::size2(0.2, 0.2)),
                colour: [0.0, 1.0, 0.0],
            },
            Draggable {
                rect: Rect::new(euclid::point2(-0.2, -0.2), euclid::size2(0.2, 0.2)),
                colour: [0.0, 0.0, 1.0],
            },
            Draggable {
                rect: Rect::new(euclid::point2(-0.2, 0.2), euclid::size2(0.2, 0.2)),
                colour: [1.0, 1.0, 0.0],
            },
            Draggable {
                rect: Rect::new(euclid::point2(0.2, -0.2), euclid::size2(0.2, 0.2)),
                colour: [0.0, 1.0, 1.0],
            },
        ],
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
        
        let mouse_x = -1.0 + 2.0 * (event.client_x() as f64 - bounding_rect.x()) / bounding_rect.width();
        let mouse_y = -1.0 + 2.0 * (event.client_y() as f64 - bounding_rect.y()) / bounding_rect.height();
        let mouse_pos = euclid::point2(mouse_x as f32, -mouse_y as f32);
        
        // update mouse collision
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.mouse_pos = mouse_pos
        });
        
        // update mouse clicked
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.mouse_down = event.buttons() == 1;
        });


        // see if any rects are being collided,
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if state.mouse_down {
                for space in state.spaces.iter_mut() {
                    if space.rect.contains(mouse_pos) {
                        // set their center to the mouse position
                        space.rect.origin = mouse_pos - euclid::vec2(space.rect.size.width / 2.0, space.rect.size.height / 2.0);
                    }
                }
            }
        });
    }));
    
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();

    closure.forget(); // Keep the closure alive

    // start animation loop
    user_draw(default_gl());

}


fn user_draw(gl: WebGlRenderingContext) {

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    STATE.with(|state| {

        let state: RefMut<STATE> = state.borrow_mut();
        let mut i = 0;

        // draw rects
        for space in state.spaces.iter() {
            i += 1;

            let top_left = space.rect.origin;
            let bottom_left = space.rect.origin + euclid::vec2(space.rect.size.width, 0.0);
            let bottom_right = space.rect.origin + space.rect.size;
            let top_right = space.rect.origin + euclid::vec2(0.0, space.rect.size.height);

            let mut data = vec![
                top_left.x as f32, top_left.y as f32, space.colour[0], space.colour[1], space.colour[2],
                bottom_left.x as f32, bottom_left.y as f32, space.colour[0], space.colour[1], space.colour[2],
                bottom_right.x as f32, bottom_right.y as f32, space.colour[0], space.colour[1], space.colour[2],
                top_right.x as f32, top_right.y as f32, space.colour[0], space.colour[1], space.colour[2], 
            ];

            if i == 1 {
                // draw square at mouse position
                data = vec![
                    state.mouse_pos.x - 0.05, state.mouse_pos.y + 0.05, 1.0, 0.0, 0.0,
                    state.mouse_pos.x - 0.05, state.mouse_pos.y - 0.05, 1.0, 0.0, 0.0,
                    state.mouse_pos.x + 0.05, state.mouse_pos.y - 0.05, 1.0, 0.0, 0.0,
                    state.mouse_pos.x + 0.05, state.mouse_pos.y + 0.05, 1.0, 0.0, 0.0,
                ]
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
