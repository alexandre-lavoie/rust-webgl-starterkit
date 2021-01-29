use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use std::rc::Rc;
use std::cell::RefCell;

/// Creates a WebGL shader from source.
///
/// ```
/// compile_shader(gl, include_str!("../resources/shaders/vertex.glsl"));
/// ```
pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

/// Links the vertex + fragment shader to WebGL.
///
/// ```
/// link_program(gl, vert_shader, frag_shader);
/// ```
pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

/// Prints to browser console.
///
/// ```
/// log!("Hello World!");
/// ```
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// Method to loop in-sync with the browser.
fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) {
    web_sys::window()
        .expect("No window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Should register animation");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Panic warnings are sent to the web browswer console.
    console_error_panic_hook::set_once();

    // Binds to the HTML canvas.
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    // Gets the WebGL instance from the HTML canvas.
    let gl = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    // Compiles vertex shader.
    let vertex_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("../resources/shaders/vertex.glsl"),
    )?;

    // Compiles fragment shader.
    let framgent_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("../resources/shaders/fragment.glsl"),
    )?;

    // Prints hello world in the browser console.
    // Just an example :)
    log!("Hello World!");

    // Links vertex + fragment to a program.
    let program = link_program(&gl, &vertex_shader, &framgent_shader)?;

    // Pointer to lambda functions.
    let f = Rc::new(RefCell::new(None));

    // Pointer to lambda functions.
    let g = f.clone();

    // GL Update loop definition.
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_: f32| {
        // Uses program.
        gl.use_program(Some(&program));

        // Adds Z-buffer.
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        // Adds clearing.
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // Clears the context to black.
        gl.clear_color(0., 0., 0., 1.);

        // Clears Z-buffer.
        gl.clear_depth(1.);

        // INSERT GL CODE HERE!

        // Loop!
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f32)>));

    // GL Update.
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
