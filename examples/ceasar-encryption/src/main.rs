use std::env;
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    let text = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: caesar-encryption <text: String> <shift: int>");
            process::exit(1);
        }
    };

    let shift: i32 = match args.next().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => {
            eprintln!("Please provide a arg: <shift: int>, e.g 3 or -3");
            process::exit(1);
        }
    };

    println!("{}", caesar(&text, shift));
}

fn caesar(s: &str, n: i32) -> String {
    let k: u8 = (((n % 26) + 26) % 26) as u8;
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_lowercase() {
            out.push(caesar_char(ch, b'a', k));
        } else if ch.is_ascii_uppercase() {
            out.push(caesar_char(ch, b'A', k));
        } else {
            out.push(ch);
        }
    }
    out
}

fn caesar_char(ch: char, base: u8, k: u8) -> char {
    let off = ch as u8 - base;
    let shifted = (off + k) % 26;
    (shifted + base) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_basic() {
        assert_eq!(caesar("ABC", 3), "DEF");
        assert_eq!(caesar("xyz", 3), "abc");
    }

    #[test]
    fn test_caesar_negative() {
        assert_eq!(caesar("DEF", -3), "ABC");
        assert_eq!(caesar("abc", -3), "xyz");
    }
}
