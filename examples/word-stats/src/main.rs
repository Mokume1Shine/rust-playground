// src/main.rs
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 1) 入力を全部読む
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    // 2) 文字数（改行抜き）
    let chars = count_chars_without_newlines(&text);

    // 3) 単語を抽出（英数字と _ の連続、lowercase）
    let words = tokenize(&text);

    // 4) まずは HashSet<&str> に挑戦してみる（わざと）
    // let mut set: HashSet<&str> = HashSet::new();
    // for w in &words { /* ここで &str をどう入れる？ */ }

    // 次に HashSet<String> に変更して通す
    // let mut set: HashSet<String> = HashSet::new();
    // for w in ??? { /* 所有権ごと入れる */ }

    let set: HashSet<String> = words.clone().into_iter().collect();

    // 5) 出力
    println!("chars: {}", chars);
    println!("words: {:?}", words);
    println!("unique_words: {}", set.len());

    Ok(())
}

// --- ここから下は小さな関数に分ける（テストしやすくする） ---

fn count_chars_without_newlines(s: &str) -> usize {
    // 改行以外をカウント
    s.chars().filter(|&c| c != '\n' && c != '\r').count()
}

fn tokenize(s: &str) -> Vec<String> {
    // 正規表現を使わずに、英数字/アンダースコアのみ残す形で分割
    // 例: "Apple, pie!!" -> ["apple", "pie"]

    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in s.chars() {
        if c.is_alphanumeric() || c == '_' {
            current.push(c.to_ascii_lowercase());
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*; // 同じファイル内の関数を使えるようにする

    #[test]
    fn count_ignores_newlines() {
        assert_eq!(count_chars_without_newlines("Hello\nWorld"), 10);
        assert_eq!(count_chars_without_newlines("a\r\nb"), 2);
    }

    #[test]
    fn tokenize_basic() {
        assert_eq!(tokenize("Apple, pie!!"), vec!["apple", "pie"]);
        assert_eq!(tokenize("a_b c-d e2f"), vec!["a_b", "c", "d", "e2f"]);
        assert_eq!(tokenize("Rust, rust; RUST!"), vec!["rust", "rust", "rust"]);
    }
}
