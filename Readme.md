# imgui-wgpu-simple-api

easy to use imgui wgpu integration

split out from [imgui-wgpu-rs](https://github.com/Yatekii/imgui-wgpu-rs/pull/38)

```rust
fn main() {
    imgui_wgpu_simple::run(Default::default(), (), |ui, _| {
        imgui::Window::new("hello world").build(ui, || {
            ui.text("Hello world!");
        });
    });
}
```

For usage instructions visit the `examples` folder.

```toml
imgui-wgpu-simple-api = { git = "https://github.com/benmkw/imgui-wgpu-simple-api" }
```
