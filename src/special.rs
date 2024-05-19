use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

use crate::utils::{init_webgl_context, create_shader};

#[wasm_bindgen]
pub fn special(
    canvas_id: &str,
    selected_color: Option<Vec<f32>>,
) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&gl);

    // define the vertices of the square
    let vertices: [f32; 8] = [
        -0.9, -0.9, // bottom left
        0.9, -0.9, // bottom right
        -0.9, 0.9, // top left
        0.9, 0.9, // top right
    ];

    // bind the verticies to the buffer 
    setup_vertices(&gl, &vertices, &shader_program);

    // set the color to shade these verticies
    let color = selected_color.unwrap_or(vec![1.0, 0.0, 0.0, 1.0]);
    let color_location = gl
        .get_uniform_location(&shader_program, "fragColor")
        .unwrap();
    gl.uniform4fv_with_f32_array(Some(&color_location), &color);
    
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(
        WebGlRenderingContext::TRIANGLE_STRIP,
        0,
        (vertices.len() / 2) as i32,
    );

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