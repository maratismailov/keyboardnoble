// use std::cell::RefCell;
// use std::error;
// use std::rc::Rc;
use std::io::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, Element, Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
// use web_sys::{Document, Element};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // proper errors in console
    // console_error_panic_hook::set_once();

    // let board = Rc::new(RefCell::new(board::Board::new_wasm("rusty".to_string())));
    // let mut our_board = Rc::clone(&board); // 0xff00ab11

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    // let mut contents = "ttttt";
    // println!("contents are {}", contents);
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // add text input for guess
    let input: Element = document.create_element("input")?;
    input.set_attribute("placeholder", "guess a word")?;

    let text_block = document.create_element("p")?;
    text_block.set_id("text-block");

    let entered_value = document.create_element("p")?;
    entered_value.set_id("entered-value");

    body.append_child(&input);
    body.append_child(&text_block);
    body.append_child(&entered_value);


    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);


    let url = "dictionary/rlnc_top100.txt";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    // request
    //     .headers()
    //     .set("Accept", "application/vnd.github.v3+json")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let jstext = JsFuture::from(resp.text()?).await?;
    let text = &jstext.as_string().unwrap();
    let copy = text.clone();
    // let text2 = text.as_str();

    // console_log!("{}", text);




    // let container = Rc::new(RefCell::new(document.create_element("div").unwrap()));
    // let dom_board = our_board.borrow().to_dom(&document)?;
    // container.borrow_mut().append_child(&dom_board)?;

    // let board_ref = Rc::clone(&board);
    // let container_ref = Rc::clone(&container);
    let cb = Closure::wrap(Box::new(move |e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        let text_value = &input.value();
        // cons(&text_value);
        change_text_block(&copy, &text_value);
    }) as Box<dyn FnMut(_)>);
    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;
    cb.forget();

    // body.append_child(&contai

    document
    .get_element_by_id("text-block")
    .expect("#text-block should exist")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("#text-block should be a HtmlElement")
    .set_inner_html(&text);

    Ok(())
}


// #[wasm_bindgen]
// pub fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

// fn cons(text: &str) {
//     console_log!("this is text: {}", &text);
// }

fn change_text_block(value: &str, text: &str) {
    console_log!("test");
    let window = web_sys::window().expect("no global `window` exists");
    // let document = web_sys::window().unwrap().document().unwrap();
    let document = window.document().expect("should have a document on window");
    document
    .get_element_by_id("text-block")
    .expect("#text-block should exist")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("#text-block should be a HtmlElement")
    .set_inner_html(&value);

    document
    .get_element_by_id("entered-value")
    .expect("#text-block should exist")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("#text-block should be a HtmlElement")
    .set_inner_html(&text);
    // body.append_child(&text_block)?;
    // Ok(())
}