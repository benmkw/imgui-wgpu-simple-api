// cargo run --example basic-simple-api

fn main() {
    imgui_wgpu_simple_api::run(Default::default(), (), |ui, _| {
        imgui::Window::new("hello world").build(ui, || {
            ui.text("Hello world!");
        });
    });
}
