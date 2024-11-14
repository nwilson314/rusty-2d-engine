use anyhow::Result;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use rusty_2d_engine::prelude::run;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(main))]
fn main() -> Result<()> {
    run()
}