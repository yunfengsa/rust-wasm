// mod utils;
extern crate image;
extern crate base64;
use image::DynamicImage;
use image::ImageFormat;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;
use base64::{encode};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    return img;
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64,", stt);
    // log(&together);
    return together;
}

#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    let base64_str = get_image_as_base64(img);
    return append_img(base64_str);
    // return get_image_as_array(img);
}

pub fn append_img(image_src: String) -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("img")?;
    // val.set_inner_html("Hello from Rust!");
    val.set_attribute("src", &image_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;

    Ok(())
}



