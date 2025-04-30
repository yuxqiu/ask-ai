use wry::{WebView, WebViewBuilder};

use super::ModelProvider;

pub struct Grok;

impl ModelProvider for Grok {
    fn call(&self, webview: &WebView, input: &str) -> anyhow::Result<()> {
        let script = format!(
            r#"
(function() {{
    const promptElem = document.querySelector("form textarea");

    if (!promptElem) {{
        console.warn("prompt not found!");
        return;
    }}

    const setValue = (elem, value) => {{
        // Use the property descriptor to update value properly
        const prototype = Object.getPrototypeOf(elem);
        const descriptor = Object.getOwnPropertyDescriptor(prototype, 'value');
        descriptor.set.call(elem, value);

        // Dispatch input event
        elem.dispatchEvent(new Event('input', {{ bubbles: true }}));
    }};

    const setFocus = (onElem, timeout = 500) => {{
        return new Promise((resolve) => {{
            setTimeout(() => {{
                onElem.focus();
                resolve(document.activeElement === onElem);
            }}, timeout);
        }});
    }};

    const simulateEnterPress = (onElem) => {{
        ['keydown', 'keypress', 'keyup'].forEach(type => {{
            const event = new KeyboardEvent(type, {{
                bubbles: true,
                cancelable: true,
                key: 'Enter',
                code: 'Enter',
                which: 13,
                keyCode: 13
            }});
            onElem.dispatchEvent(event);
        }});
    }};

    const retrySetFocusUntilSuccess = (onElem, retry) => {{
        console.debug('focus attempt remaining', retry);
        if (retry === 0) {{
            return Promise.reject(new Error("Prompt refused focus"));
        }}

        return setFocus(onElem)
            .then(success => {{
                if (success) return;
                return retrySetFocusUntilSuccess(onElem, retry - 1);
            }});
    }};

    setValue(promptElem, "{}");

    retrySetFocusUntilSuccess(promptElem, 3)
        .then(() => simulateEnterPress(promptElem))
        .catch(e => console.warn(e.message));
}})();
        "#,
            input
        );
        webview.evaluate_script(&script)?;
        Ok(())
    }

    fn setup<'a>(&self, webview_builder: WebViewBuilder<'a>) -> WebViewBuilder<'a> {
        webview_builder.with_url("https://grok.com")
    }
}
