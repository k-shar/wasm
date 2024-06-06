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
    pixels: Vec<f32>,
    colours: Vec<Srgb>,
    pointwise: bool,
}


// Initialize the state
thread_local! {
    static STATE: RefCell<STATE> = RefCell::new(STATE {
        resolution: 1,
        pointwise: false,
        pixels: make_pixels(1),
        colours: make_colours(make_pixels(1), false),
    });
}


// return two triangles
fn make_square(res: i32, x: i32, y: i32) -> Vec<f32> {

    let triangles = vec![
        x, y,
        x + 1, y,
        x, y + 1,
        x, y + 1,
        x + 1, y,
        x + 1, y + 1,
    ];

    // map triangles onto range -1 to 1
    triangles.iter().map(|v| (*v as f32 / res as f32) * 2.0 - 1.0).collect::<Vec<f32>>()
}

fn make_pixels(resolution: i32) -> Vec<f32> {
    (0..resolution*2)
        .flat_map(|x| (0..resolution*2)
        .flat_map(move |y| make_square(resolution, x, y)))
        .collect()
}

fn make_colours(pixels: Vec<f32>, pointwise: bool) -> Vec<Srgb> {
    if pointwise {
        pointwise_colours(pixels)
    } else {
        individual_colours(pixels)
    }
} 

fn pointwise_colours(pixels: Vec<f32>) -> Vec<Srgb> {

    let mut colours: HashMap<String, palette::rgb::Rgb> = HashMap::new();
    let mut rng = rand::thread_rng();

    pixels.chunks(2).map(|p| {

        // get the hash of the pixel
        let p = format!("{},{}", (p[0] * 1000.0) as i32, (p[1] * 1000.0) as i32);

        // get the colour from the hashmap or insert a new one
        colours.entry(p).or_insert_with(|| {
            Srgb {
                red: rng.gen_range(0.0..1.0),
                green: rng.gen_range(0.0..1.0),
                blue: rng.gen_range(0.0..1.0),
                standard: std::marker::PhantomData,
            }
            
        }).clone()
    }).collect()
}

fn individual_colours(pixels: Vec<f32>) -> Vec<Srgb> {
    let mut rng = rand::thread_rng();
    pixels
        .chunks(3)
        .map(|_p| {
            // random colour
            Srgb {
                red: rng.gen_range(0.0..1.0),
                green: rng.gen_range(0.0..1.0),
                blue: rng.gen_range(0.0..1.0),
                standard: std::marker::PhantomData,
            }
        }).collect()
}

#[wasm_bindgen]
pub fn p_update_box(checked: bool) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.pointwise = checked;
        state.colours = make_colours(state.pixels.clone(), state.pointwise);
    });
}


#[wasm_bindgen]
pub fn p_update_resolution(res: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.resolution = res;
        state.pixels = make_pixels(res);
        state.colours = make_colours(state.pixels.clone(), state.pointwise);
    });
}



#[wasm_bindgen]
pub fn point_draw(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {

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
