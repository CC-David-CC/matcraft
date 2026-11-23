//! MatCraft Studio browser entry point.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_arch = "wasm32")]
mod web {
    use std::cell::RefCell;

    use matcraft_ui_egui::{MatApp, MatAppOptions};
    use wasm_bindgen::prelude::*;

    thread_local! {
        static RUNNER: RefCell<Option<eframe::WebRunner>> = const { RefCell::new(None) };
        static CONTEXT: RefCell<Option<egui::Context>> = const { RefCell::new(None) };
    }

    pub fn start() {
        wasm_bindgen_futures::spawn_local(async {
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Some(canvas) =
                document.get_element_by_id("matcraft_canvas").and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            else {
                return;
            };
            let hosted = canvas.has_attribute("data-webos-workspace");
            let compact = canvas.client_width() < 840;
            let runner = eframe::WebRunner::new();
            let result = runner
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(move |context| {
                        let mut app = MatApp::new(context, MatAppOptions::default());
                        if hosted {
                            app.enable_workspace_files(compact);
                            CONTEXT.with_borrow_mut(|slot| *slot = Some(context.egui_ctx.clone()));
                        }
                        Ok(Box::new(app))
                    }),
                )
                .await;
            if result.is_ok() {
                RUNNER.with_borrow_mut(|slot| *slot = Some(runner));
            }
            if let Some(loading) = document.get_element_by_id("matcraft_loading") {
                if result.is_ok() {
                    loading.remove();
                } else {
                    let _ = loading.set_attribute("data-start-error", "");
                    loading.set_text_content(Some("MatCraft requires a browser with WebGPU or WebGL2 enabled."));
                }
            }
        });
    }

    fn with_app<T>(operation: impl FnOnce(&mut MatApp) -> T) -> Result<T, JsValue> {
        RUNNER.with_borrow(|slot| {
            let runner = slot.as_ref().ok_or_else(|| JsValue::from_str("MatCraft is still starting."))?;
            let mut app = runner.app_mut::<MatApp>().ok_or_else(|| JsValue::from_str("MatCraft is busy."))?;
            Ok(operation(&mut app))
        })
    }

    fn repaint() {
        CONTEXT.with_borrow(|slot| {
            if let Some(context) = slot {
                context.request_repaint();
            }
        });
    }

    #[wasm_bindgen]
    pub fn workspace_ready() -> bool {
        RUNNER.with_borrow(Option::is_some)
    }

    #[wasm_bindgen]
    pub fn replace_script(name: String, text: Option<String>) -> Result<bool, JsValue> {
        let changed = with_app(|app| app.workspace_replace(name, text))?;
        repaint();
        Ok(changed)
    }

    #[wasm_bindgen]
    pub fn script_snapshot() -> Result<String, JsValue> {
        with_app(|app| app.workspace_snapshot().map(|value| value.to_string()).unwrap_or_default())
    }

    #[wasm_bindgen]
    pub fn script_saved(name: String, text: &str) -> Result<bool, JsValue> {
        let clean = with_app(|app| app.workspace_saved(name, text))?;
        repaint();
        Ok(clean)
    }

    #[wasm_bindgen]
    pub fn run_script() -> Result<String, JsValue> {
        let output = with_app(|app| app.workspace_run().map(|value| value.to_string()).unwrap_or_default())?;
        repaint();
        Ok(output)
    }

    #[wasm_bindgen]
    pub fn take_file_request() -> Result<String, JsValue> {
        with_app(|app| app.workspace_take_request().map(|value| value.to_string()).unwrap_or_default())
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    web::start();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let _ = matcraft_ui_egui::MatAppOptions::default();
    let _ = matcraft_engine::Session::new();
    let _ = egui::Color32::BLACK;
    let _ = serde_json::Value::Null;
    eprintln!("matcraft-web targets wasm32-unknown-unknown; build with `trunk build --release` or check with `cargo xtask wasm`");
}
