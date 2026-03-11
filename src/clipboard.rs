use arboard::Clipboard;
use std::thread;
use std::time::Duration;

const CLIPBOARD_HOLD_MS: u64 = 300;

/// Copies `text` to the system clipboard.
///
/// The clipboard content is held for a short duration to ensure it is
/// flushed before the process exits. Returns an error message on failure.
pub fn clip_copy(text: &str) -> Result<(), String> {
    let text = text.to_string();

    let handle = thread::spawn(move || -> Result<(), String> {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("failed to access clipboard: {e}"))?;

        clipboard
            .set_text(text)
            .map_err(|e| format!("failed to copy text to clipboard: {e}"))?;

        thread::sleep(Duration::from_millis(CLIPBOARD_HOLD_MS));
        Ok(())
    });

    handle
        .join()
        .map_err(|_| "clipboard thread panicked".to_string())?
}
