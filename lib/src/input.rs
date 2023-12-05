#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";

#[cfg(not(windows))]
const EMPTY_LINE: &'static str = "\n\n";

pub fn empty_line_chunks<'a>(input: &'a str) -> std::str::Split<'a, &'a str> {
    input.split(EMPTY_LINE)
}

#[test]
fn test_empty_line_chunks() {
    let text = &format!("first{}second{}third", EMPTY_LINE, EMPTY_LINE);

    let chunks: Vec<_> = empty_line_chunks(text).collect();
    assert_eq!(vec!["first", "second", "third"], chunks);
}