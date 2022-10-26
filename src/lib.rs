// use std::cell::RefCell;
// use std::error;
// use std::rc::Rc;
use std::io::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, Element, Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
// use rand::Rng;
use rand::seq::IteratorRandom;
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
    let mut rng = rand::thread_rng();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let container = document.create_element("div")?;
    container.set_id("container");

    // add text input for guess
    let input: Element = document.create_element("textarea")?;
    input.set_id("input");

    let task_text = document.create_element("p")?;
    task_text.set_id("task-text");

    let full_value = document.create_element("span")?;
    full_value.set_id("full-value");

    let good_value = document.create_element("span")?;
    good_value.set_id("good-value");

    let bad_value = document.create_element("span")?;
    bad_value.set_id("bad-value");

    let entered_value = document.create_element("p")?;
    entered_value.set_id("entered-value");

    let task_status = document.create_element("p")?;
    task_status.set_id("task-status");

    body.append_child(&container);
    container.append_child(&task_text);
    container.append_child(&input);
    container.append_child(&entered_value);
    container.append_child(&task_status);

    task_text.append_child(&good_value);
    task_text.append_child(&bad_value);
    task_text.append_child(&full_value);


    document
    .get_element_by_id("task-status")
    .expect("#task-status should exist")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("#task-status should be a HtmlElement")
    .set_inner_html("empty");


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
    let splitted_text = text.lines();
    let mut task_text = String::new();
    for n in 0..15 {
        task_text.push_str(&splitted_text.clone().choose(&mut rng).unwrap());
        if n != 14 {
            task_text.push_str(" ");
        }
    }
    let copy_task_text = task_text.clone();

    let cb = Closure::wrap(Box::new(move |e: Event| {
        let input = e
            .current_target()
            .unwrap()
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .unwrap();
        let text_value = &input.value();
        process_text(&text_value, &copy_task_text);
    }) as Box<dyn FnMut(_)>);
    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;
    cb.forget();


    document
    .get_element_by_id("full-value")
    .expect("#full-value should exist")
    .dyn_into::<web_sys::HtmlElement>()
    .expect("#full-value should be a HtmlElement")
    .set_inner_html(&task_text);

    Ok(())
}

fn process_text(entered_text: &str, task_text: &str) {
    let entered_text_len = entered_text.chars().count();
    let task_text_len = task_text.chars().count();
    let task_to_check_string: String = task_text.chars().skip(0).take(entered_text_len).collect::<String>();
    let task_to_check = task_to_check_string.as_str();
    // let mut entered_correct = "";
    // let mut remaining_full = "";
    // let remaining_full_string: String;
    // let mut entered_wrong = "";
    // let mut entered_correct_len = 0;
    // let mut entered_wrong_len = 0;
    // let mut entered_wrong_string: String;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut good_value: String = String::from("");
    let mut bad_value: String = String::from("");
    let mut remaining: String;
    let mut good_value_len = 0;
    let mut bad_value_len = 0;
    // let mut remaining_string: String;
    if entered_text == "" {
        bad_value = String::from("");
        good_value = String::from("");

        document
        .get_element_by_id("bad-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&bad_value);

        document
        .get_element_by_id("full-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&task_text);

        document
        .get_element_by_id("good-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&good_value);

        document
        .get_element_by_id("input")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .style().set_property("background-color", "white");
    }

    for (index, char) in entered_text.chars().enumerate() {
        if char == task_text.chars().nth(index).unwrap() && entered_text == task_to_check {
            good_value.push(char);
            good_value_len = good_value.chars().count();
            remaining = task_text.chars().skip(good_value_len).take(task_text_len).collect::<String>();
            document
            .get_element_by_id("input")
            .expect("#task-text should exist")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("#task-text should be a HtmlElement")
            .style().set_property("background-color", "white");
        }
        else {
            bad_value.push(char);
            bad_value_len = bad_value.chars().count();
            remaining = task_text.chars().skip(good_value_len + bad_value_len).take(task_text_len).collect::<String>();
            document
            .get_element_by_id("input")
            .expect("#task-text should exist")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("#task-text should be a HtmlElement")
            .style().set_property("background-color", "yellow");
        }

        document
        .get_element_by_id("good-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&good_value);

        document
        .get_element_by_id("bad-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&bad_value);

        document
        .get_element_by_id("full-value")
        .expect("#task-text should exist")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("#task-text should be a HtmlElement")
        .set_inner_html(&remaining);
    }
}
