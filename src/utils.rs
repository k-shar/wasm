use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};

pub fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
    
    // get the canvas element from the DOM
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    
    // spawn WebGL context
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    // set the size of the gl viewport to match the canvas
    gl.viewport( 0, 0,
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap(),
    );

    Ok(gl)
}

// generic helper function to create a shader
pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> WebGlShader {

    let shader = gl.create_shader(shader_type).unwrap();

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        shader
    } else {
        let error_message = gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into());
        panic!("Error compiling shader: {}", error_message);
    }
}

// generic helper function to link shaders
pub fn link_shaders(
    gl: &WebGlRenderingContext, 
    vertex_shader_source: &str, 
    fragment_shader_source: &str,
) -> WebGlProgram {

    let logging = false;
    if logging {web_sys::console::log_1(&"Compiling shaders...".into());}

    let vertex_shader = create_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vertex_shader_source);
    if logging {web_sys::console::log_1(&"Successfully compiled vertex shader.".into());}

    let fragment_shader = create_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fragment_shader_source);
    if logging {web_sys::console::log_1(&"Successfully compiled fragment shader.".into());}

    let shader_program = gl.create_program().unwrap();

    gl.attach_shader(&shader_program, &vertex_shader);
    if logging {web_sys::console::log_1(&"Successfully attached vertex shader.".into());}

    gl.attach_shader(&shader_program, &fragment_shader);
    if logging {web_sys::console::log_1(&"Successfully attached fragment shader.".into());}

    gl.link_program(&shader_program);
    if logging {web_sys::console::log_1(&"Successfully linked shader program.".into());}

    // ensure the program was linked successfully
    if gl
        .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        // Set the shader program as active.
        gl.use_program(Some(&shader_program));
        shader_program
    } else {
        let error_message = gl.get_program_info_log(&shader_program)
            .unwrap_or_else(|| "Unknown error linking program".into());
        panic!("Error linking shader program: {}", error_message);
    }
}


pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
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
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}