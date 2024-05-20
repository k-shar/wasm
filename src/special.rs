use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use palette::{Hsv, Srgb, FromColor};
extern crate js_sys;

use crate::utils::{init_webgl_context, create_shader};

fn rainbow_chase(time: i32) -> Vec<f32> {
    let hsv_color = Hsv::new(time as f64, 1.0, 1.0);
    let color: Srgb = Srgb::from_color(hsv_color).into();
    vec![color.red, color.green, color.blue, 1.0]
}

#[wasm_bindgen]
pub fn special(
    canvas_id: &str,
    i: i32,
) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&gl);

    let vertices = [
        -0.5, 0.5, // top left
        -0.5, -0.5, // bottom left
        0.5, -0.5, // bottom right
        0.5, 0.5, // top right
    ];
    setup_vertices(&gl, &vertices, &shader_program);

    // get a pointer to the uniform vec4 fragColor
    let color_location = gl.get_uniform_location(&shader_program, "fragColor").unwrap();

    // set fragColo to our value
    let color = rainbow_chase(i);
    gl.uniform4fv_with_f32_array(Some(&color_location), &color);
    
    // draw on the screen
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(
        WebGlRenderingContext::TRIANGLE_FAN,
        0,
        (vertices.len() / 2) as i32,
    );

    // log this to the user

    let output= web_sys::window().unwrap().document().unwrap().get_element_by_id("output").unwrap();

    let r = (color[0] * 255.0) as u8;
    let g = (color[1] * 255.0) as u8;
    let b = (color[2] * 255.0) as u8;

    output.set_inner_html(format!("Red: {} Green: {}, Blue: {}", r, g, b).as_str());
    
    // return success
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