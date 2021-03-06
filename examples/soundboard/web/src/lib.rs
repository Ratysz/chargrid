use chargrid_web::{Context, Size};
use general_audio_web::WebAudioPlayer;
use soundboard_app::app;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    let player = WebAudioPlayer::new_with_mime("video/ogg");
    let context = Context::new(Size::new(16, 8), "content");
    context.run_app(app(player));
    Ok(())
}
