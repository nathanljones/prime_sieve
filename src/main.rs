fn main() {
    println!("Hello, world!");
    prime_sieve(200);
}

fn prime_sieve(max_no: u32) {
    let mut primes = vec![true; max_no.try_into().unwrap()];
    let limit = max_no;
    /*let limit = max_no as f32;
    let limit = limit.sqrt();
    let limit = limit as u32;*/

    for n in 1..=primes.len() {
        if n % 2 == 0 {
            primes[n - 1] = false;
        }
    }

    for n in (3..=limit).step_by(2) {
        for j in n * n..=max_no {
            if j % n == 0 {
                primes[(j - 1) as usize] = false;
            }
        }
    }
    for n in 1..=primes.len() {
        if primes[n - 1] {
            println!("{}", n);
        }
    }
}
