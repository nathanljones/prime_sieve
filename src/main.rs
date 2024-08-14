use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut no_passes: u128 = 1;
    let mut duration: u128 = 0;
    while now.elapsed().as_secs() < 5 {
        let start_of_pass: Instant = Instant::now();
        prime_sieve(1000000, true);
        no_passes += 1;
        duration += start_of_pass.elapsed().as_millis();
    }

    let avg_passes: f64 = (duration / no_passes) as f64;
    println!("No Passes {no_passes}");
    println!("Average duration {avg_passes} miliseconds");
    println!("Finished");
}

fn prime_sieve(max_no: u64, bypass_check: bool) {
    let mut primes = vec![true; (max_no).try_into().unwrap()];
    let limit = max_no;

    // remove all the even ones first - treat this as a special case
    for n in (4..=primes.len()).step_by(2) {
        if n % 2 == 0 {
            primes[n - 1] = false;
        }
    }

    for n in (3..=limit).step_by(2) {
        // if we encountered a false value then we can skip as this is
        // just a multiple of one which has been found
        if !primes[(n - 1) as usize] {
            continue;
        }

        // starting number needs to be the square of the number as smaller ones are either prime
        // or marked already e.g. 5, start at 25 as prior multiples of 5 have been already marked
        let starting_number = n * n;
        if starting_number > max_no {
            break;
        }

        // increment by odd mulitples only
        // e.g. 5 x 1 = 5
        //      5 x 2 = 10 - even number so no point in checking
        //      5 x 3 = 15
        //      5 x 4 = 20 - even number so no point in checking
        for j in (starting_number..=max_no).step_by((n * 2) as usize) {
            if j % n == 0 {
                primes[(j - 1) as usize] = false;
            }
        }
    }
    if !bypass_check {
        if has_correct_no_primes(&primes) {
            println!("PRIME NO CHECK: PASSED");
        } else {
            println!("PRIME NO CHECK: FAILED");
        }
    }
    // now write out all our prime numbers
    /*for n in 1..=primes.len() {
        if primes[n - 1] {
            println!("{}", n);
        }
    }*/
}

fn has_correct_no_primes(prime_sieve: &[bool]) -> bool {
    let prime_check: HashMap<u64, usize> = HashMap::from([
        (10, 4),
        (100, 25),
        (1000, 168),
        (10000, 1229),
        (100000, 9592),
        (1000000, 78498),
        (10000000, 664579),
        (100000000, 5761455),
        (1000000000, 50847534),
        (10000000000, 455052511),
    ]);

    let prime_sieve_length: u64 = prime_sieve.len() as u64;
    if prime_check.contains_key(&prime_sieve_length) {
        let prime_count = prime_sieve.iter().filter(|val| **val).count();
        // need to take away 1 as I've included 1 as a prime - the above table does not
        let prime_count = prime_count - 1;
        println!("FOUND {0} PRIMES", prime_count);
        prime_count == prime_check[&prime_sieve_length]
    } else {
        false
    }
}
