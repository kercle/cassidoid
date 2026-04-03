use kernel::Kernel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct CassidaKernel {
    inner: Kernel,
}

#[wasm_bindgen]
impl CassidaKernel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, input: &str) -> String {
        let res = self.inner.execute(input.to_string());
        serde_json::to_string(&res).expect("Serde not expected to fail here.")
    }
}

#[wasm_bindgen]
pub fn eval_input(input: &str) -> String {
    let kernel = Kernel::default();
    let res = kernel.execute(input.to_string());
    serde_json::to_string(&res).expect("Serde not expected to fail here.")
}
