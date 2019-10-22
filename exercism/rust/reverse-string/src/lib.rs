use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut g_str = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut result = String::with_capacity(g_str.capacity());
    g_str.reverse();
    for gc in g_str {
        result.push_str(gc);
    }
    result
}
