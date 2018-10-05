use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        // expect return v if from_str returns Ok(v)
        // expect raise err if from_str returns Err(e)
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argment"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest comon divisor of {:?} is {}",
             numbers, d);
}

// 別のファイルに書いてロードする方法がわからない
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// 別のファイルに書くと見つからない
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                   3 * 11);
}
