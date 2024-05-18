use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

use crate::utils::{init_webgl_context, setup_shaders, setup_vertices};

#[wasm_bindgen]
pub fn draw_square(
    canvas_id: &str,
    selected_color: Option<Vec<f32>>,
) -> Result<WebGlRenderingContext, JsValue> {

    // create gl context and shader program
    let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&gl).unwrap();

    // define the vertices of the square
    let vertices: [f32; 12] = [
        -0.5, -0.5, 0.0, // bottom left
        0.5, -0.5, 0.0, // bottom right
        -0.5, 0.5, 0.0, // top left
        0.5, 0.5, 0.0, // top right
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
        (vertices.len() / 3) as i32,
    );

    Ok(gl)
}
