pub fn quotes_balanced(s: &str) -> bool {
    let dq = s
        .chars()
        .filter(|&c| c == '"')
        .count();
    let sq = s
        .chars()
        .filter(|&c| c == '\'')
        .count();
    dq % 2 == 0 && sq % 2 == 0
}
