use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGlProgram, WebGlRenderingContext};
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
    mouse_pos: Vec<f32>,
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
        mouse_pos: vec![0.0, 0.0],
    });
}

// update number of sides on the shape we're displaying
#[wasm_bindgen]
pub fn p_update_sides(n: i32) {
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
            x: 0.3 * theta.cos() as f32,
            y: 0.3 * theta.sin() as f32,
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

fn translate_coords(coords: Vec<Vertex>, x: f32, y: f32) -> Vec<Vertex> {
    coords.iter()
        .map(|c| {
            Vertex {
                x: c.x + x,
                y: c.y + y,
                theta: c.theta,
            }
        }).collect()
}


#[wasm_bindgen]
pub fn point_init(canvas_id: &str) {

    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id(canvas_id)
        .expect("Failed to find canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to convert element to HTMLCanvasElement");
    
    let closure = Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let canvas_rect = canvas.get_bounding_client_rect();
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.mouse_pos = vec![
                -1.0 + 2.0 * (mouse_event.client_x() as f32 - canvas_rect.left() as f32) / canvas_rect.width() as f32,
                 1.0 - 2.0 * (mouse_event.client_y() as f32 - canvas_rect.top() as f32) / canvas_rect.height() as f32,
            ];
        });
    }) as Box<dyn FnMut(_)>);
    
    web_sys::window().expect("No window found")
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");
    
    closure.forget();
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
        let mut state: std::cell::RefMut<STATE> = state.borrow_mut();
        
        state.time += 1; 

        let rotated_verts: Vec<Vertex> = rotate_2d_coords(
            state.vertices.clone(), 
            state.time as f32 * state.rotate_speed as f32 * 0.001
        );
        let translated_coords: Vec<Vertex> = translate_coords(
            rotated_verts.clone(), 
            state.mouse_pos[0], 
            state.mouse_pos[1]
        );
        
        let verts = translated_coords.clone();

        let vertex_colours: Vec<Srgb> = get_coords_of_ngon(state.side_count).iter()
            .map(|c| {
                let hue = 255.0 * c.theta / (2.0 * PI) as f32;
                let hsv = Hsv::new(hue, 1.0, 1.0);
                Srgb::from_color(hsv).into()
            }).collect();

        // zip the two lists together
        let mut data: Vec<f32> = verts.iter()
            .zip(vertex_colours.iter())
            .flat_map(|(v, c)| vec![v.x, v.y, c.red, c.green, c.blue])
            .collect::<Vec<f32>>();

        // draw on the screen
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // draw shape
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &(unsafe { js_sys::Float32Array::view(&data).into() }),
            WebGlRenderingContext::STATIC_DRAW,
        );
        let vertices_count = (state.vertices.len()) as i32;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, vertices_count);

        // draw point at location of mouse pointer
        data = vec![state.mouse_pos[0], state.mouse_pos[1], 255.0, 255.0, 255.0];
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", state.mouse_pos)));
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &(unsafe { js_sys::Float32Array::view(&data).into() }),
            WebGlRenderingContext::STATIC_DRAW,
        );
        gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);

    });

    Ok(gl)
}
