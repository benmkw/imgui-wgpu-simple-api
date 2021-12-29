// cargo run --example fullscreen-simple-api

use imgui::Condition;

struct State {
    last_frame: std::time::Instant,
    height: f32,
    width: f32,
    high_dpi_factor: f64,
}

fn main() {
    let config = imgui_wgpu_simple_api::Config {
        on_resize: &|input, state: &mut State, hdpi| {
            state.height = input.height as f32;
            state.width = input.width as f32;
            state.high_dpi_factor = hdpi;
        },
        ..Default::default()
    };

    let state = State {
        last_frame: std::time::Instant::now(),
        height: 100.0,
        width: 100.0,
        high_dpi_factor: 2.0,
    };

    imgui_wgpu_simple_api::run(config, state, |ui, state| {
        let now = std::time::Instant::now();

        imgui::Window::new("full-window example")
            .position([0.0, 0.0], Condition::Always)
            .collapsible(false)
            .resizable(false)
            .size(
                [
                    state.width / state.high_dpi_factor as f32,
                    state.height / state.high_dpi_factor as f32,
                ],
                Condition::Always,
            )
            .menu_bar(true)
            .build(ui, || {
                ui.text("Hello world!");
            });

        state.last_frame = now;
    });
}
