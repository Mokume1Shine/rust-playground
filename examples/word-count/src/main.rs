use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    text.make_ascii_lowercase();
    println!("text: {}", text);

    let mut word_map = HashMap::new();

    for raw in text.split_whitespace() {
        let word = raw.trim_matches(|c: char| !c.is_alphanumeric());
        if word.is_empty() {
            continue;
        }
        *word_map.entry(word).or_insert(0) += 1;
    }

    let mut items: Vec<_> = word_map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (key, value) in items {
        println!("{}: {}", key, value);
    }

    Ok(())
}
