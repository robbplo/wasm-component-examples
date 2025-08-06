use crate::exports::betty_blocks::concat_text::concat_text;

wit_bindgen::generate!({ generate_all });

struct ConcatText;

impl concat_text::Guest for ConcatText {
    fn concat(a: String, b: String) -> String {
        format!("{} {}", a, b)
    }
}

export! {ConcatText}
