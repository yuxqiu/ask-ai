use wry::{Rect, WebView, WebViewBuilder, http::Request};

pub struct Input {
    webview: WebView,
}

impl Input {
    pub fn new(webview: WebView) -> Self {
        Self { webview }
    }

    pub fn set_bounds(&self, bounds: Rect) -> wry::Result<()> {
        self.webview.set_bounds(bounds)
    }

    pub fn setup<F>(builder: WebViewBuilder, ipc_handler: F) -> WebViewBuilder
    where
        F: Fn(Request<String>) + 'static,
    {
        builder
            .with_html(
                r#"
<!DOCTYPE html>
<html>
<head>
  <style>
    body {
      margin: 0;
      padding: 0;
      background: #222;
      display: flex;
      justify-content: center;
      align-items: flex-end;
      min-height: 100vh;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    }
    .textarea-container {
      width: 100%;
      max-width: 800px;
      padding: 10px;
      box-sizing: border-box;
    }
    textarea {
      width: 100%;
      min-height: 40px;
      max-height: 200px;
      font-size: 16px;
      background: #333;
      color: white;
      border: 1px solid #444;
      border-radius: 8px;
      outline: none;
      padding: 10px;
      resize: none;
      overflow-y: auto;
      box-sizing: border-box;
      line-height: 1.5;
    }
    textarea::placeholder {
      color: #888;
    }
    textarea:focus {
      border-color: #555;
    }
  </style>
</head>
<body>
  <div class="textarea-container">
    <textarea
      id="userInput"
      placeholder="Type here and press Enter..."
      autofocus
    ></textarea>
  </div>

  <script>
    const textarea = document.getElementById('userInput');

    // Adjust textarea height dynamically
    function adjustTextareaHeight() {
      textarea.style.height = 'auto';
      textarea.style.height = `${Math.min(textarea.scrollHeight, 200)}px`;
    }

    // Handle keydown events
    textarea.addEventListener('input', adjustTextareaHeight);

    textarea.addEventListener('keydown', (event) => {
      if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        const message = textarea.value.trim();
        if (message) {
          // Send message (replace with your desired functionality)
          if (typeof window.ipc !== 'undefined') {
            window.ipc.postMessage(message); // For Electron-like environments
          } else {
            console.log('Message sent:', message); // Fallback for browser
          }
          textarea.value = ''; // Clear textarea
          adjustTextareaHeight();
        }
      }
    });

    // Initial height adjustment
    adjustTextareaHeight();
  </script>
</body>
</html>
"#,
            )
            .with_ipc_handler(ipc_handler)
    }
}
