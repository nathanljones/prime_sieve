fn main() {
    println!("Hello, world!");
    prime_sieve(200);
}

fn prime_sieve(max_no: u32) {
    let mut primes = vec![true; max_no.try_into().unwrap()];
    let limit = max_no;

    // remove all the 2 ones first - treat this as a special case
    for n in (4..=primes.len()).step_by(2) {
        if n % 2 == 0 {
            primes[n - 1] = false;
        }
    }

    for n in (3..=limit).step_by(2) {
        // if we encountered a false value then we can skip as this is
        // just a multiple of one which has been found
        if !primes[(n - 1) as usize] {
            break;
        }
        // main sieve check
        for j in n * n..=max_no {
            if j % n == 0 {
                primes[(j - 1) as usize] = false;
            }
        }
    }
    // now write out all our prime numbers
    for n in 1..=primes.len() {
        if primes[n - 1] {
            println!("{}", n);
        }
    }
}
