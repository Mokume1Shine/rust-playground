use std::hint::black_box;
use std::time::{Duration, Instant};

fn split_ws(s: &str) -> usize {
    s.split_whitespace().map(black_box).count()
}
fn split_space(s: &str) -> usize {
    s.split(' ').map(black_box).count()
}

fn time_it<F: Fn(&str) -> usize>(label: &str, f: F, s: &str, iters: usize) -> Duration {
    // ウォームアップ
    for _ in 0..100 {
        let _ = f(black_box(s));
    }
    // 計測
    let start = Instant::now();
    let mut acc = 0usize;
    for _ in 0..iters {
        acc = acc.wrapping_add(f(black_box(s)));
    }
    let dur = start.elapsed();
    // 結果を使う（最適化で消されない対策）
    std::fs::write("/dev/null", acc.to_string()).ok();
    println!("{label}: {:?} (iters={iters})", dur);
    dur
}

fn main() {
    let inputs = [
        ("ascii_simple", "lorem ipsum dolor sit amet".to_string()),
        ("mixed_ws", "lorem\tipsum   dolor\nsit\r\namet".to_string()),
        (
            "unicode_ws",
            "foo\u{00A0}bar\u{2003}baz".to_string(), // NBSP / EM SPACE
        ),
        (
            "long",
            std::iter::repeat("alpha beta\tgamma\ndelta  ")
                .take(20_000)
                .collect::<String>(),
        ),
    ];

    let iters = 200;

    for (name, s) in inputs {
        println!("=== {name} ===");
        let _ = time_it("split_whitespace", split_ws, &s, iters);
        let _ = time_it("split(' ')", split_space, &s, iters);
        println!();
    }
}
