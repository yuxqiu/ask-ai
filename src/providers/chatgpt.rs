use wry::{WebView, WebViewBuilder};

use super::ModelProvider;

pub struct ChatGPT;

impl ModelProvider for ChatGPT {
    fn call(&self, webview: &WebView, input: &str) -> anyhow::Result<()> {
        let script = format!(
            // Adapted from: https://github.com/bsorrentino/AI-MultiPrompt-Extension/tree/main/Chrome
            // - Enter event is replaced by clicking the button because Enter corresponds to a newline
            //   on small-screen devices.
            r#"
(function() {{
    const promptElem = document.querySelector("form #prompt-textarea");

    if (!promptElem) {{
        console.warn("prompt not found!");
    }} else {{
        const oldP = promptElem.querySelector("p");
        const text = document.createTextNode("{}");
        const newP = document.createElement('p');
        newP.appendChild(text);
        oldP.replaceWith(newP);

        const setFocus = ( onElem, timeout = 500 ) => {{
            onElem.dispatchEvent(new Event('input', {{ 'bubbles': true }}));
            return new Promise( (resolve, reject) => {{
                setTimeout( () => {{
                    onElem.focus();
                    if( document.activeElement === onElem  )
                        resolve(true)
                    else
                        //reject( )
                        resolve(false)
                }}, timeout )
            }})
        }}

        const retrySetFocusUntilSuccess = (onElem, retry) => {{
            console.debug('focus attempt remaining ', retry);
            if (retry === 0) {{
                return Promise.reject("prompt refuse the focus")
            }}

            return setFocus(onElem)
                .then(() => Promise.resolve())
                .catch(() => retrySetFocusUntilSuccess(onElem, retry - 1));
        }}

        retrySetFocusUntilSuccess(promptElem, 3)
            .then(() => {{
                // Find the submit button by ID and trigger a click event
                const submitButton = document.getElementById("composer-submit-button");
                if (submitButton) {{
                    submitButton.click();
                    console.log("Form submission triggered by clicking the submit button.");
                }} else {{
                    console.warn("Submit button not found!");
                }}
            }})
            .catch((e) => console.warn(e.message));
    }}
}})();
        "#,
            input
        );
        webview.evaluate_script(&script)?;
        Ok(())
    }

    fn setup<'a>(&self, webview_builder: WebViewBuilder<'a>) -> WebViewBuilder<'a> {
        webview_builder.with_url("https://chatgpt.com")
    }
}
