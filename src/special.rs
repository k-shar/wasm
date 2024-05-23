use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use palette::{Hsv, Srgb, FromColor};
use std::cell::RefCell;
extern crate js_sys;

use std::f64::consts::PI;
use crate::utils::{init_webgl_context, link_shaders};

// define the state
struct STATE {
    vertices: Vec<f32>,
    time: i32,
    rotate_speed: i32,
    colour_speed: i32,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        vertices: get_coords_of_ngon(8),
        time: 0,
        rotate_speed: 50,
        colour_speed: 50,
    });
}

// update number of sides on the shape we're displaying
#[wasm_bindgen]
pub fn update_sides(n: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.vertices = get_coords_of_ngon(n);
    });
}

// update the speed of the rotation
#[wasm_bindgen]
pub fn update_rotation_speed(s: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.rotate_speed = s;
    });
}

// update the speed of the rainbow chase
#[wasm_bindgen]
pub fn update_colour_speed(s: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.colour_speed = s;
    });
}

// generate a color based on time
fn rainbow_chase(time: i32) -> Vec<f32> {
    let hsv_color = Hsv::new(time as f64, 1.0, 1.0);
    let color: Srgb = Srgb::from_color(hsv_color).into();
    vec![color.red, color.green, color.blue, 1.0]
}

// generate the coordinates of an n-gon using roots of unity/ polar coords
fn get_coords_of_ngon(n: i32) -> Vec<f32> {
    (0..n).map(|k| {
        let theta = 2.0 * PI * ((k) as f64) / (n as f64);
        vec!(theta.cos() as f32, theta.sin() as f32)
    }).into_iter().flat_map(|v| v).collect()
}

fn rotate_2d_coords(coords: Vec<f32>, angle: f32) -> Vec<f32> {
    coords.chunks(2)
        .map(|c| {
            vec![
                c[0] * angle.cos() - c[1] * angle.sin(),
                c[0] * angle.sin() + c[1] * angle.cos(), 
            ]
        }).flatten().collect()
}



// draw the state to the screen given 
#[wasm_bindgen]
pub fn draw(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    
    let vertex_shader_source =
        "
        attribute vec2 coordinates;

        void main(void) {
            gl_Position = vec4(coordinates, 1.0, 1.0);
        }
        ";
    let fragment_shader_source = 
        "
        precision mediump float;

        uniform vec4 fragColor;

        void main(void) {
            gl_FragColor = fragColor;
        }
        ";
    let shader_program: WebGlProgram = link_shaders(&gl, vertex_shader_source, fragment_shader_source).unwrap();

    STATE.with(|state| {

        // get the state for this program
        let mut state = state.borrow_mut();
        state.time += 1; 

        let rotated_verts = rotate_2d_coords(
            state.vertices.clone(), 
            state.time as f32 * state.rotate_speed as f32 * 0.001
        );

        // bind the vertices to the shader program
        setup_vertices(&gl, &rotated_verts, &shader_program);

        // set fragment shader to colour the right color
        let color = rainbow_chase(state.time * state.colour_speed / 50);
        let color_location = gl.get_uniform_location(&shader_program, "fragColor").unwrap();
        gl.uniform4fv_with_f32_array(Some(&color_location), &color);
        
        // draw on the screen
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        let vertices_count = (state.vertices.len() / 2) as i32;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, vertices_count);
    });

    Ok(gl)
}

// bind the vertices to the shader program
fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {

    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };

    let vertex_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates");
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(coordinates_location as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}