## Why No Tauri

1. The multi-webview layout is broken on Linux: [GitHub Issue #13071](https://github.com/tauri-apps/tauri/issues/13071).

## Why Tao + Wry

To build a multi-webview window from scratch, I have to use Tao and Wry.
- Supporting it on Linux is a bit tricky: [GitHub Issue #1314](https://github.com/tauri-apps/wry/issues/1314).

Handling cookies is also challenging:
- Initially, I tried to find APIs for getting and setting cookies. However, I encountered [this issue](https://github.com/tauri-apps/wry/issues/1511#issue-2892146782), which was disappointing.
- Ultimately, I solved the problem using `WebContext`, which allows us to instantiate a webview with a `data_directory` (though this is not supported on macOS: [GitHub Issue #1321](https://github.com/tauri-apps/wry/issues/1321)).

## Why No Tray Icon

Simply put, it’s not very useful. I could bind a keyboard shortcut to start the app (as described below).

## Why No Floating Window

Although it is possible to create a floating window by setting windows to non-resizable, on Wayland + Sway, it’s difficult to control where the floating window appears. Therefore, I decided to build a standalone app instead.
- Floating via non-resizable: [GitHub Issue #862](https://github.com/rust-windowing/winit/issues/862).

## Why No Keyboard Binding

Simply put, it’s not easy (at least, I don’t know how to do it). On Wayland, this requires configuration within the compositor. Since controlling a floating window is already challenging, I chose not to pursue this further, especially since I can easily bind a shortcut to start the app directly from my compositor.