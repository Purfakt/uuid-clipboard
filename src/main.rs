use arboard::Clipboard;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4().to_string();
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(uuid).unwrap();
}