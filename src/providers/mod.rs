use wry::{WebView, WebViewBuilder};

/// Implement different AI model providers
///
/// TODO:
/// - some of them might be able to purely defined by a config file
pub mod chatgpt;
pub mod grok;

pub trait ModelProvider {
    fn setup<'a>(&self, webview_builder: WebViewBuilder<'a>) -> WebViewBuilder<'a>;
    fn call(&self, webview: &WebView, input: &str) -> anyhow::Result<()>;
}
