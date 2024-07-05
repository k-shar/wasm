use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, HtmlInputElement, WebGlProgram, WebGlRenderingContext, Window};
use euclid::{self, Box2D};

extern crate js_sys;

use crate::utils::{init_webgl_context, link_shaders};

// define the state
#[derive(Clone)]
struct STATE {
    rects: Vec<Box2D<f64, f64>>,
    x: f32
}

#[wasm_bindgen]
pub fn user_init() {

    // creat initial state
    let state = STATE {
        rects: vec![
            Box2D::new(euclid::point2(0.7, 0.3), euclid::point2(-0.5, -0.5)),
            Box2D::new(euclid::point2(0.0, 0.0), euclid::point2(0.5, 0.5)),
        ],
        x: 0.0,
    };

    
    // create shader program
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


    let gl = init_webgl_context("user_input").unwrap();
        
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


    // call initial request animation frame, do again at the end of user_draw
    let closure: Closure<dyn FnMut()> = Closure::wrap(
        Box::new(move || user_draw(gl.clone(), state.clone()))
    );
    web_sys::window().expect("should have a Window")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    closure.forget();

}


fn user_draw(gl: WebGlRenderingContext, mut state: STATE) {

    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    let mut rng = thread_rng();
    let mut i = 0;

    for rect in state.rects.iter() {
        i += 1;
        let bottom_left: euclid::Point2D<f64, f64> = rect.min;
        let top_right: euclid::Point2D<f64, f64> = rect.max;
        let top_left: euclid::Point2D<f64, f64> = euclid::point2(bottom_left.x, top_right.y);
        let bottom_right: euclid::Point2D<f64, f64> = euclid::point2(top_right.x, bottom_left.y);

        let data = vec![
            top_left.x as f32, top_left.y as f32, 1.0, 0.0, 0.0,
            bottom_left.x as f32, bottom_left.y as f32, 0.0, 1.0, 0.0,
            bottom_right.x as f32, bottom_right.y as f32, 0.0, 0.0, 1.0,
            top_right.x as f32, top_right.y as f32, 1.0, 1.0, 1.0,
        ].iter().map(|x| 
            *x 
            + ((state.x + rng.gen_range(0.0..0.3)).sin() / 5.0)
            * if i % 2 == 0 { 1.0 } else { -1.0 }
            - 0.1
        ).collect::<Vec<f32>>();

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
    let closure: Closure<dyn FnMut()> = Closure::wrap(
        Box::new(move || {
            state.x += 0.05;
            user_draw(gl.clone(), state.clone())
        })
    );
    web_sys::window().expect("should have a Window")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
    closure.forget();

}
