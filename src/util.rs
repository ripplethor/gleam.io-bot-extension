use js_sys::Promise;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    }))
    .await
    .unwrap();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! elog {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

pub fn _get_random_u32_between(min: u32, under: u32) -> u32 {
    let crypto = window().unwrap().crypto().unwrap();
    let mut random = [0; 4];
    crypto.get_random_values_with_u8_array(&mut random).unwrap();
    min + (u32::from_be_bytes(random) % (under - min))
}
