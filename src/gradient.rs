use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use palette::{Hsv, Srgb, FromColor};
use std::cell::RefCell;

extern crate js_sys;

use std::f64::consts::PI;
use crate::utils::{init_webgl_context, link_shaders};

// define the state
struct STATE {
    side_count: i32,
    vertices: Vec<Vertex>,
    time: i32,
    rotate_speed: i32,
}

#[derive(Clone)]
struct Vertex {
    x: f32,
    y: f32,
    theta: f32,
}

// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        side_count: 8,
        vertices: get_coords_of_ngon(8),
        time: 0,
        rotate_speed: 5,
    });
}

// update number of sides on the shape we're displaying
#[wasm_bindgen]
pub fn g_update_sides(n: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.vertices = get_coords_of_ngon(n);
        state.side_count = n;
    });
}

// generate the coordinates of an n-gon using roots of unity/ polar coords
fn get_coords_of_ngon(n: i32) -> Vec<Vertex> {
    (0..n).map(|k| {
        let theta = 2.0 * PI * ((k) as f64) / (n as f64);
        Vertex {
            x: theta.cos() as f32,
            y: theta.sin() as f32,
            theta: theta as f32,
        }
    }).collect()
}

// rotate coordinates counter clockwise by angle in radians
fn rotate_2d_coords(coords: Vec<Vertex>, angle: f32) -> Vec<Vertex> {
    coords.iter()
        .map(|c| {
            Vertex {
                x: c.x * angle.cos() - c.y * angle.sin(), 
                y: c.x * angle.sin() + c.y * angle.cos(),
                theta: c.theta + angle,
            }
        }).collect()
}


// draw the state to the screen given 
#[wasm_bindgen]
pub fn gradient_draw(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    
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

    let shader_program: WebGlProgram = link_shaders(&gl, vertex_shader_source, fragment_shader_source);

    // spawn the ARRAY_BUFFER for the vertices to use each frame
    gl.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER, 
        Some(&gl.create_buffer().unwrap())
    );

    // specify how the coordinates attribute should read from the vertex buffer
    let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates") as u32;
    gl.vertex_attrib_pointer_with_i32(
        coordinates_location, 
        2, 
        WebGlRenderingContext::FLOAT, 
        false, 
        5 * std::mem::size_of::<f32>() as i32,
        0
    );
    gl.enable_vertex_attrib_array(coordinates_location);

    // specify how the colour attribute should read from from the vertex buffer
    let colour_location = gl.get_attrib_location(&shader_program, "colour") as u32; 
    gl.vertex_attrib_pointer_with_i32(
        colour_location, 
        3, 
        WebGlRenderingContext::FLOAT, 
        false, 
        5 * std::mem::size_of::<f32>() as i32,
        2 * std::mem::size_of::<f32>() as i32,
    );
    gl.enable_vertex_attrib_array(colour_location);


    STATE.with(|state: &RefCell<STATE>| {

        // get a mutable version state 
        let mut state: std::cell::RefMut<STATE> = state.borrow_mut();
        
        state.time += 1; 

        let rotated_verts: Vec<Vertex> = rotate_2d_coords(
            state.vertices.clone(), 
            state.time as f32 * state.rotate_speed as f32 * 0.001
        );

        let vertex_colours: Vec<Srgb> = get_coords_of_ngon(state.side_count).iter()
            .map(|c| {
                let hue = 255.0 * c.theta / (2.0 * PI) as f32;
                let hsv = Hsv::new(hue, 1.0, 1.0);
                Srgb::from_color(hsv).into()
            }).collect();

        // zip the two lists together
        let data: Vec<f32> = rotated_verts.iter()
            .zip(vertex_colours.iter())
            .flat_map(|(v, c)| vec![v.x, v.y, c.red, c.green, c.blue])
            .collect::<Vec<f32>>();

        
        // fill ARRAY_BUFFER with the vertex data
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &(unsafe { js_sys::Float32Array::view(&data).into() }),
            WebGlRenderingContext::STATIC_DRAW,
        );
       
        // draw on the screen
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        let vertices_count = (state.vertices.len()) as i32;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, vertices_count);

    });

    Ok(gl)
}
