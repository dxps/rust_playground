use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for g_char in UnicodeSegmentation::graphemes(input, true) {
        result = [g_char, &result].join("");
    }
    result

    // After submitting my updated solution, I found others' extremely short (one-line) version:
    // input.graphemes(true).rev().collect()
}
