use wry::{WebView, WebViewBuilder};

use super::ModelProvider;

pub struct ChatGPT;

// https://github.com/bsorrentino/AI-MultiPrompt-Extension/tree/main/Chrome
impl ModelProvider for ChatGPT {
    fn call(&self, webview: &WebView, input: &str) -> anyhow::Result<()> {
        let script = format!(
            r#"
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

    const pressEnter = (onElem) => {{
        // Create a new KeyboardEvent
        const enterEvent = new KeyboardEvent('keydown', {{
            bubbles: true, // Make sure the event bubbles up through the DOM
            cancelable: true, // Allow it to be canceled
            key: 'Enter', // Specify the key to be 'Enter'
            code: 'Enter', // Specify the code to be 'Enter' for newer browsers
            which: 13 // The keyCode for Enter key (legacy property)
        }});

        // Dispatch the event on the textarea element
        onElem.dispatchEvent(enterEvent);

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
        .then(() => pressEnter(promptElem))
        .catch((e) => console.warn(e.message));
}}
        "#,
            input
        );
        println!("{}", script);
        webview.evaluate_script(&script)?;
        Ok(())
    }

    fn setup<'a>(&self, webview_builder: WebViewBuilder<'a>) -> WebViewBuilder<'a> {
        webview_builder.with_url("https://chatgpt.com")
    }
}
