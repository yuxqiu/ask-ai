use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use directories::ProjectDirs;
use input::Input;
use layout::{compute_bounds, compute_optimal_window_layout};
use providers::{ModelProvider, chatgpt::ChatGPT, grok::Grok};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::dpi::LogicalPosition;
use wry::{Rect, WebContext, WebViewBuilder, dpi::LogicalSize};

mod input;
mod layout;
mod providers;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .build(&event_loop)?;

    let data_dir = if let Some(proj_dirs) = ProjectDirs::from("", "yuxqiu", "ai") {
        proj_dirs.config_dir().to_owned()
    } else {
        Path::new("./data").to_path_buf()
    };
    let mut context = WebContext::new(Some(data_dir));

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
        // need to use `build_as_child` to make with_bounds work
        let webview = builder.build_as_child(&window)?;

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

    let input_height = 100;
    let mut size = window.inner_size().to_logical::<u32>(window.scale_factor());
    size.height -= input_height;

    // specify any newly added providers here
    const N: usize = 2;
    let providers: [&dyn ModelProvider; N] = [&ChatGPT, &Grok];

    let layout = compute_optimal_window_layout(size, providers.len());
    let bounds = compute_bounds::<N>(size, layout);

    let webviews = array_util::try_from_fn::<_, N, _>(|i| {
        let wb = providers[i].setup(WebViewBuilder::with_web_context(&mut context));
        build_webview(wb.with_bounds(bounds[i]))
    })?
    .map(Mutex::new)
    .map(Arc::new);

    // Bottom input area (as a WebView)
    let webviews_for_input = webviews.clone();
    let input = Input::setup(WebViewBuilder::new(), move |message| {
        for (i, provider) in providers.iter().enumerate() {
            provider
                .call(
                    &webviews_for_input[i]
                        .lock()
                        .expect("failed to acquire the lock"),
                    message.body(),
                )
                .with_context(|| format!("failed to call provider {i}"))
                .unwrap()
        }
    })
    .with_bounds(Rect {
        position: LogicalPosition::new(0, size.height).into(),
        size: LogicalSize::new(size.width, input_height).into(),
    });
    let input = Input::new(build_webview(input)?);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // For some strange reason, the first screen is not drawn correctly,
            // but immediately after the window loses and regains focus, it draws the window correctly.
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let mut size = size.to_logical::<u32>(window.scale_factor());
                size.height -= input_height;
                let bounds = compute_bounds::<N>(size, layout);

                for (webview, bound) in webviews.iter().zip(bounds) {
                    webview
                        .lock()
                        .expect("failed to acquire the lock")
                        .set_bounds(bound)
                        .expect("resize failed: `set_bounds` failed");
                }

                {
                    input
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, size.height).into(),
                            size: LogicalSize::new(size.width, input_height).into(),
                        })
                        .unwrap();
                }
            }
            // This helps a bit, but the first screen is still not correct (until the mouse hovers through the window)
            //
            // Event::RedrawRequested(_) => {
            //     // Ensure WebView is resized during redraw
            //     let mut size = window.inner_size().to_logical::<u32>(window.scale_factor());
            //     size.height -= input_height;
            //     let bounds = compute_bounds::<N>(size, layout);

            //     for (webview, bound) in webviews.iter().zip(bounds) {
            //         webview
            //             .lock()
            //             .expect("failed to acquire the lock")
            //             .set_bounds(bound)
            //             .expect("resize failed: `set_bounds` failed");
            //     }

            //     {
            //         input
            //             .set_bounds(Rect {
            //                 position: LogicalPosition::new(0, size.height).into(),
            //                 size: LogicalSize::new(size.width, input_height).into(),
            //             })
            //             .unwrap();
            //     }
            // }
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
