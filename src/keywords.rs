// https://doc.rust-lang.org/stable/reference/keywords.html
const KEYWORDS: [&str; 52] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "union",
];

pub fn is_keyword(x: &str) -> bool {
    KEYWORDS.contains(&x.to_lowercase().as_str())
}

#[test]
fn value_is_a_keyword() {
    assert!(is_keyword("type"));
    assert!(is_keyword("TYPE"));
}

#[test]
fn value_is_not_a_keyword() {
    assert!(!is_keyword("rpms"));
}
