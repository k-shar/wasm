use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, HtmlInputElement, WebGlProgram, WebGlRenderingContext, Window};
use palette::{Hsv, Srgb, FromColor};
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use euclid::{self, Box2D};

extern crate js_sys;

use crate::utils::{init_webgl_context, link_shaders};


// define the state
#[derive(Clone)]
struct STATE {
    rects: Vec<Box2D<f64, f64>>,
    x: f64
}


#[wasm_bindgen]
pub fn user_init() {

    let state = STATE {
        rects: vec![
            Box2D::new(euclid::point2(0.0, 0.0), euclid::point2(0.5, 0.5)),
            Box2D::new(euclid::point2(0.7, 0.3), euclid::point2(-0.5, -0.5)),
        ],
        x: 0.0,
    };

    let gl = init_webgl_context("user_input").unwrap();
    
    //  create shader program
    let vertex_shader_source =
        "
        attribute vec2 coordinates;
        attribute vec3 colour;
        varying vec3 out_colour;

        void main(void) {
            gl_Position = vec4(coordinates, 0.0, 1.0);
            out_colour = colour;
        }
        ";
    let fragment_shader_source = 
        "
        precision mediump float;
        varying vec3 out_colour;

        void main(void) {
            gl_FragColor = vec4(out_colour, 1.0);
        }
        ";

        
    // spawn the ARRAY_BUFFER for the vertices to use each frame
    let shader_program: WebGlProgram = link_shaders(&gl, vertex_shader_source, fragment_shader_source); 
    gl.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER, 
        Some(&gl.create_buffer().unwrap())
    );

    // specify how the coordinates attribute should read from the vertex buffer
    let coordinate_location = gl.get_attrib_location(&shader_program, "coordinates") as u32;
    gl.vertex_attrib_pointer_with_i32(
        coordinate_location, 2, WebGlRenderingContext::FLOAT, 
        false, 5 * std::mem::size_of::<f32>() as i32, 0
    );
    gl.enable_vertex_attrib_array(coordinate_location);

    // specify how the colour attribute should read from from the vertex buffer
    let colour_location = gl.get_attrib_location(&shader_program, "colour") as u32; 
    gl.vertex_attrib_pointer_with_i32(
        colour_location, 3, WebGlRenderingContext::FLOAT, false, 
        5 * std::mem::size_of::<f32>() as i32,
        2 * std::mem::size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(colour_location);

    let window = web_sys::window().expect("should have a Window");
    let closure = Closure::wrap(
        Box::new(move || user_draw(gl.clone(), state.clone())) as Box<dyn FnMut()>
    );

    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    closure.forget();

}


fn user_draw(gl: WebGlRenderingContext, mut state: STATE) {

    state.x += 0.1;
    web_sys::console::log_1(&state.x.into());

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    for rect in state.rects.iter() {
        let bottom_left: euclid::Point2D<f64, f64> = rect.min;
        let top_right: euclid::Point2D<f64, f64> = rect.max;
        let top_left: euclid::Point2D<f64, f64> = euclid::point2(bottom_left.x, top_right.y);
        let bottom_right: euclid::Point2D<f64, f64> = euclid::point2(top_right.x, bottom_left.y);

        let data = vec![
            (state.x + top_left.x) as f32, top_left.y as f32, 1.0, 0.0, 0.0,
            bottom_left.x as f32, bottom_left.y as f32, 0.0, 1.0, 0.0,
            bottom_right.x as f32, bottom_right.y as f32, 0.0, 0.0, 1.0,
            top_right.x as f32, top_right.y as f32, 1.0, 1.0, 1.0,
        ];
        // fill ARRAY_BUFFER with the vertex data
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &(unsafe { js_sys::Float32Array::view(&data).into() }),
            WebGlRenderingContext::STATIC_DRAW,
        );
    
        // draw on the screen
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
    }
}
