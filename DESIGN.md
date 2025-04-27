## Why no Tauri

1. Multiwebview layout is incorrect on Linux: https://github.com/tauri-apps/tauri/issues/13071

## Why tao + wry

So, I have to use tao + wry to build a multiwebview window from scratch.
- Supporting it on Linux is a bit tricky: https://github.com/tauri-apps/wry/issues/1314

Supporting cookie is a bit tricky:
- Initially, I tried to find APIs to get cookies and set cookies. However, found: https://github.com/tauri-apps/wry/issues/1511#issue-2892146782, which is a bit disappointing.
- At the end, this problem was fixed by using `WebContext`, which allows us to instantiate webview with `data_directory` (though not supported on MacOS: https://github.com/tauri-apps/wry/issues/1321).

## Why no tray icon

Simply said, it's not that useful. I could bind a keyboard shortcut to start it (see below).

## Why no floating window

Although floating window can be made possible by setting windows to non-resizable, on wayland + sway, it's hard to precisely control where the floating window shows up. So, eventually, I decided to simply build a standalone app.
- Float via non-resizable: https://github.com/rust-windowing/winit/issues/862

## Why no keyboard binding

Simply, it's not easy (at least I have no idea). At the end, on wayland, this needs to be configured within the compositor. Given that it's hard to do floating window, I decided to not proceed this further as well (since I could easily bind a shortcut to start the app on my compositor).