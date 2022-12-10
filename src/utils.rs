pub fn newline_end(str: &str) -> String {
    if str.ends_with('\n') {
        str.to_string()
    } else {
        str.to_string() + "\n"
    }
}
