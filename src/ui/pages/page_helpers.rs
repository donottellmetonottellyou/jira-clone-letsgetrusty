use pad::PadStr;
use unicode_segmentation::UnicodeSegmentation;

pub fn get_column_string(text: &str, width: usize) -> String {
    if text.len() <= width {
        return text.pad_to_width(width);
    }

    match width {
        0..=3 => {
            let mut column_string = "...".to_string();
            column_string.truncate(width);
            column_string
        }
        _ => {
            let mut column_string: String = text.graphemes(true).take(width - 3).collect();
            column_string.push_str("...");
            column_string
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
