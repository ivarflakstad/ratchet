use js_sys::{ArrayBuffer, Uint8Array, JSON};

use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub(crate) fn to_error(value: JsValue) -> JsError {
    JsError::new(
        JSON::stringify(&value)
            .map(|js_string| {
                js_string
                    .as_string()
                    .unwrap_or(String::from("An unknown error occurred."))
            })
            .unwrap_or(String::from("An unknown error occurred."))
            .as_str(),
    )
}

pub(crate) fn js_error(message: &str) -> JsError {
    JsError::new(message)
}

pub(crate) async fn to_future<T>(promise: js_sys::Promise) -> Result<T, JsError>
where
    T: JsCast,
{
    let result = JsFuture::from(promise).await.map_err(to_error)?;
    result.dyn_into::<T>().map_err(to_error)
}

pub(crate) async fn fetch(url: &str) -> Result<Response, JsError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).map_err(to_error)?;

    let window = web_sys::window().unwrap();
    let promise = window.fetch_with_request(&request);
    to_future(promise).await
}