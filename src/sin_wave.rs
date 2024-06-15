use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext};
use palette::Srgb;
use std::{cell::RefCell, collections::HashMap};

use rand::Rng;

extern crate js_sys;

use crate::utils::{init_webgl_context, link_shaders};

// define the state
#[derive(Debug)]
struct STATE {
    resolution: i32,
    wavelength: f32,
    pixels: Vec<f32>,
    colours: Vec<Srgb>,
}


// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        resolution: 100,
        wavelength: 10.0,
        pixels: make_pixels(1),
        colours: pointwise_colours(make_pixels(1), 10.0),
    });
}


// return two triangles that make a square with coords normalized to -1.0 to 1.0
fn make_square(res: i32, x: i32, y: i32) -> Vec<f32> {
    vec![
        x, y, x + 1, y, x, y + 1, x, y + 1, x + 1, y, x + 1, y + 1
    ].iter().map(|v| (*v as f32 / res as f32) * 2.0 - 1.0).collect::<Vec<f32>>()
}

// return a grid of pixels
fn make_pixels(resolution: i32) -> Vec<f32> {
    (0..resolution*2)
        .flat_map(|x| (0..resolution*2)
        .flat_map(move |y| make_square(resolution, x, y)))
        .collect()
}

fn pointwise_colours(pixels: Vec<f32>, w: f32) -> Vec<Srgb> {

    let mut rng = rand::thread_rng();
    let mut colours: HashMap<String, palette::rgb::Rgb> = HashMap::new();

    pixels.chunks(2).map(|p| {

        let dist = ((p[0]).powi(2) + (p[1]).powi(2)).sqrt();
        let val = (1.0 + (w * dist).cos()) / 2.0;

        let p = format!("{},{}", (p[0] * 1000.0) as i32, (p[1] * 1000.0) as i32);
        colours.entry(p).or_insert_with(|| {
            Srgb {
                red: rng.gen_range(0.0..0.2),
                green: rng.gen_range(0.0..0.2),
                blue: val,
                standard: std::marker::PhantomData,
            }
        }).clone()

    }).collect()
}


#[wasm_bindgen]
pub fn s_update_resolution(res: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.resolution = res;
        state.pixels = make_pixels(res);
        state.colours = pointwise_colours(state.pixels.clone(), state.wavelength);
    });
}

#[wasm_bindgen]
pub fn s_update_wavelength(w: f32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.wavelength = w;
        state.colours = pointwise_colours(state.pixels.clone(), state.wavelength);
    });
}


#[wasm_bindgen]
pub fn sin_draw(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {

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
        let state: std::cell::RefMut<STATE> = state.borrow_mut();

        // zip the two lists together
        let data: Vec<f32> = state.pixels
            .chunks(2)
            .zip(state.colours.iter())
            .flat_map(|(v, c)| vec![v[0], v[1], c.red, c.green, c.blue])
            .collect::<Vec<f32>>();

        // draw on the screen
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // draw shape
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &(unsafe { js_sys::Float32Array::view(&data).into() }),
            WebGlRenderingContext::STATIC_DRAW,
        );
        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, data.len() as i32 / 5);

    });

    Ok(gl)
}
