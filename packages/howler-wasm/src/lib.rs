use wasm_bindgen::prelude::*;
use web_sys::console;

// ---------- Raw JS bindings --------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type Howler;

    #[wasm_bindgen(static_method_of = Howler)]
    fn init();

    #[wasm_bindgen(static_method_of = Howler)]
    fn volume(vol: f32);

    #[wasm_bindgen]
    type Howl;

    #[wasm_bindgen(constructor)]
    fn new(init: &JsValue) -> Howl;

    #[wasm_bindgen(method)]
    fn load(this: &Howl);

    #[wasm_bindgen(method)]
    fn play(this: &Howl) -> i32;

    #[wasm_bindgen(method)]
    fn pause(this: &Howl) -> Howl;

    #[wasm_bindgen(method)]
    fn stop(this: &Howl) -> Howl;

    #[wasm_bindgen(method)]
    fn seek(this: &Howl, seek: u64) -> JsValue;

    #[wasm_bindgen(method, js_name = loop_)]
    fn set_loop(this: &Howl, l: bool) -> Howl;

    #[wasm_bindgen(method, js_name = unload)]
    fn unload(this: &Howl);
}

#[wasm_bindgen]
pub struct JsHowl(Howl);

#[wasm_bindgen]
pub fn set_global_volume(vol: f32) {
    Howler::volume(vol);
}

#[wasm_bindgen]
pub fn init() {
    Howler::init();
}

#[wasm_bindgen]
impl JsHowl {
    /// Create the main API handle.
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Self {
        console::log_1(&format!("Howler WASM wrapper with URL: {}", url).into());
        let init = js_sys::Object::new();
        js_sys::Reflect::set(&init, &"src".into(), &JsValue::from(vec![url])).unwrap();
        js_sys::Reflect::set(&init, &"autoplay".into(), &JsValue::from_bool(true)).unwrap();
        js_sys::Reflect::set(&init, &"html5".into(), &JsValue::from_bool(true)).unwrap();
        let howl = Howl::new(&init);
        Self(howl)
    }

    /// Play a sound and return the *sound ID* (i32) that controls this concrete instance.
    #[wasm_bindgen]
    pub fn play(&self) {
        console::log_1(&"play".into());
        self.0.play();
    }

    #[wasm_bindgen]
    pub fn pause(&self) {
        self.0.pause();
    }

    #[wasm_bindgen]
    pub fn stop(&self) {
        self.0.stop();
    }

    #[wasm_bindgen]
    pub fn unload(&self) {
        self.0.unload();
    }

    #[wasm_bindgen]
    pub fn seek(&self, seconds: u64) {
        self.0.seek(seconds);
    }

    #[wasm_bindgen]
    pub fn set_loop(&self, l: bool) {
        self.0.set_loop(l);
    }
}
