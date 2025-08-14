fn main() {
    let mut count: usize = 2;
    let mut prime_numbers = Vec::<usize>::new();
    loop {
        let sq = count.isqrt() + 1;
        let is_prime = true;
        for &pn in &prime_numbers {
            if pn > sq {
                break;
            }
            if count % pn == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_numbers.push(count);
            print!("{}, ", count);
        }
        if count == std::usize::MAX {
            break;
        }
        count += 1;
    }
}
