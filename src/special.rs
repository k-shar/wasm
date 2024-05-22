use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use palette::{Hsv, Srgb, FromColor};
use std::cell::RefCell;
extern crate js_sys;

use std::f64::consts::PI;
use crate::utils::{init_webgl_context, create_shader};

// define the state
struct STATE {
    vertices: Vec<f32>,
    time: i32,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        vertices: get_coords_of_ngon(8),
        time: 0,
    });
}

// update sides
#[wasm_bindgen]
pub fn update_sides(n: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.vertices = get_coords_of_ngon(n);
    });
}

// generate a color based on time
fn rainbow_chase(time: i32) -> Vec<f32> {
    let hsv_color = Hsv::new(time as f64, 1.0, 1.0);
    let color: Srgb = Srgb::from_color(hsv_color).into();
    vec![color.red, color.green, color.blue, 1.0]
}

// return a vec of vertices for an n-gon
pub fn get_coords_of_ngon(n: i32) -> Vec<f32> {

    // generate the vertices of an n-gon
    let vertices: Vec<f32> = (0..n)
        .map(|k| {
            let theta = 2.0 * PI * (k as f64) / (n as f64);
            vec!(theta.cos() as f32, theta.sin() as f32)
        }).into_iter().flat_map(|v| v).collect();

    // log the coords to the page
    let r = web_sys::window().unwrap().document().unwrap().get_element_by_id("roots").unwrap();
    r.set_inner_html(format!("{:?}", vertices).as_str());

    vertices
}

// draw the state to the screen given 
#[wasm_bindgen]
pub fn draw(
    canvas_id: &str,
) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&gl);

    STATE.with(|state| {

        // get the state for this program
        let mut state = state.borrow_mut();
        state.time += 1; 

        // bind the vertices to the shader program
        setup_vertices(&gl, &state.vertices, &shader_program);

        // set fragment shader to colour the right color
        let color = rainbow_chase(state.time);
        let color_location = gl.get_uniform_location(&shader_program, "fragColor").unwrap();
        gl.uniform4fv_with_f32_array(Some(&color_location), &color);
        
        // draw on the screen
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        let vertices_count = (state.vertices.len() / 2) as i32;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, vertices_count);

        // log the RGB valaues to the user
        let output = web_sys::window().unwrap().document().unwrap().get_element_by_id("rgb_values").unwrap();
        output.set_inner_html(format!(
            "Red: {} Green: {}, Blue: {}",
            (color[0] * 255.0) as u8, 
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8
        ).as_str());
    });

    Ok(gl)
}


// load the vertex and fragment shaders
fn setup_shaders(gl: &WebGlRenderingContext) -> WebGlProgram {

    let vertex_shader = create_shader(&gl,
        WebGlRenderingContext::VERTEX_SHADER,
        "
        attribute vec2 coordinates;

        void main(void) {
            gl_Position = vec4(coordinates, 1.0, 1.0);
        }
        "
    ).unwrap();

    let fragment_shader = create_shader(&gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        "
        precision mediump float;

        uniform vec4 fragColor;

        void main(void) {
            gl_FragColor = fragColor;
        }
        "
    ).unwrap();

    // link these shaders into a program
    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    gl.use_program(Some(&shader_program));
    shader_program
}

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
    gl.vertex_attrib_pointer_with_i32(
        coordinates_location as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}