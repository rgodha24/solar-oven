#[cfg(not(target_arch = "wasm32"))]
pub fn export_types() {
    specta::export::ts("src/lib/types.ts").unwrap();
}
