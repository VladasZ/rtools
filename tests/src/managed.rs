#![cfg(test)]

use rtools::{
    data_manager::{DataManager, ResourceLoader},
    managed,
};

struct Text {
    text: String,
}

impl ResourceLoader for Text {
    fn load_path(path: &std::path::Path) -> Self {
        Self {
            text: path.to_string_lossy().to_string(),
        }
    }

    fn load_data(data: &[u8], _name: impl ToString) -> Self {
        Self {
            text: String::from_utf8_lossy(data).to_string(),
        }
    }
}

managed!(Text);

#[test]
fn managed() {
    Text::set_root_path(&std::path::PathBuf::from("hello"));

    let text1 = Text::get("Sokol");
    let text2 = Text::get("Sokol");

    assert_eq!(text1.text, "hello/Sokol");
    assert_eq!(text2.text, "hello/Sokol");

    assert_eq!(text1.hash(), text2.hash());
}
