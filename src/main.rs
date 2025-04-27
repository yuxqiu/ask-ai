use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use directories::ProjectDirs;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    Rect, WebContext, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
};

mod layout;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();

    // https://github.com/rust-windowing/winit/issues/862
    // - set window non-resizable enables floating
    let window = WindowBuilder::new()
        .with_decorations(false)
        .build(&event_loop)?;

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let fixed = {
        use gtk::prelude::*;
        use tao::platform::unix::WindowExtUnix;
        let fixed = gtk::Fixed::new();
        let vbox = window
            .default_vbox()
            .context("fail to fix gtk window on linux")?;
        vbox.pack_start(&fixed, true, true, 0);
        fixed.show_all();
        fixed
    };

    let build_webview = |builder: WebViewBuilder<'_>| -> wry::Result<wry::WebView> {
        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = builder.build(&window)?;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let webview = {
            use wry::WebViewBuilderExtUnix;
            builder.build_gtk(&fixed)?
        };

        Ok(webview)
    };

    let size = window.inner_size().to_logical::<u32>(window.scale_factor());
    let input_height = 1;

    let data_dir = if let Some(proj_dirs) = ProjectDirs::from("", "yuxqiu", "ai") {
        proj_dirs.config_dir().to_owned()
    } else {
        Path::new("./data").to_path_buf()
    };
    let mut context = WebContext::new(Some(data_dir));

    let builder = WebViewBuilder::with_web_context(&mut context)
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(size.width / 2, size.height - input_height).into(),
        })
        .with_url("https://chatgpt.com");
    let webview = build_webview(builder)?;
    let webview = Arc::new(Mutex::new(webview));

    let builder2 = WebViewBuilder::with_web_context(&mut context)
        .with_bounds(Rect {
            position: LogicalPosition::new(size.width / 2, 0).into(),
            size: LogicalSize::new(size.width / 2, size.height - input_height).into(),
        })
        .with_url("https://grok.com");
    let webview2 = build_webview(builder2)?;
    let webview2 = Arc::new(Mutex::new(webview2));

    // Bottom input area (as a WebView)
    let webview_for_input = webview.clone();
    let webview2_for_input = webview2.clone();
    let input_height = 50;
    let input_webview_builder = WebViewBuilder::new()
        .with_bounds(Rect {
            position: LogicalPosition::new(0, size.height - input_height).into(),
            size: LogicalSize::new(size.width, input_height).into(),
        })
        .with_html(r#"
<!DOCTYPE html>
<html>
<body style="margin:0;padding:0;background:#222;">
  <textarea id="userInput"
    style="width:100%;height:100%;font-size:20px;background:#333;color:white;border:none;outline:none;padding:10px;resize:none;overflow:hidden;word-wrap:break-word;white-space:pre-wrap;box-sizing:border-box;"
    placeholder="Type here and press Enter..."
    autofocus
    onkeydown="if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        window.ipc.postMessage(document.getElementById('userInput').value);
        document.getElementById('userInput').value = '';
    }"
  ></textarea>
</body>
</html>
"#)
        .with_ipc_handler(move |message| {
            println!("User entered: {}", message.body());
            let js_code = format!(
                "document.body.innerHTML += '<p>{}</p>';",
                message.body()
            );
            {
                webview_for_input.lock().unwrap().evaluate_script(&js_code).unwrap();
            }
            {
                webview2_for_input.lock().unwrap().evaluate_script(&js_code).unwrap();
            }
        });
    let input_webview = build_webview(input_webview_builder)?;
    let input_webview = Arc::new(Mutex::new(input_webview));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let size = size.to_logical::<u32>(window.scale_factor());
                {
                    webview
                        .lock()
                        .unwrap()
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(size.width / 2, size.height - input_height)
                                .into(),
                        })
                        .unwrap();
                }
                {
                    webview2
                        .lock()
                        .unwrap()
                        .set_bounds(Rect {
                            position: LogicalPosition::new(size.width / 2, 0).into(),
                            size: LogicalSize::new(size.width / 2, size.height - input_height)
                                .into(),
                        })
                        .unwrap();
                }
                {
                    input_webview
                        .lock()
                        .unwrap()
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, size.height - input_height).into(),
                            size: LogicalSize::new(size.width, input_height).into(),
                        })
                        .unwrap();
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
