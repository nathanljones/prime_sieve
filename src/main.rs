use clap::Parser;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Parser, Default, Debug)]
#[command(version, about = "A Prime Number Sieve Implementation")]
struct ProgramParameters {
    #[arg(default_value_t = 5, short, long)]
    /// number of seconds to run for
    seconds: u32,
    #[arg(default_value_t = 10000, short, long)]
    /// maximum number of primes to look for   
    limit: u64,
    #[clap(long, short, action)]
    bypass_check: bool,
}
fn main() {
    let program_parameters = ProgramParameters::parse();

    let now = Instant::now();
    let mut no_passes: u64 = 1;
    let mut duration: u128 = 0;
    let mut first_pass: bool = true;
    while now.elapsed().as_secs() < u64::from(program_parameters.seconds) {
        let start_of_pass: Instant = Instant::now();
        // only on the first pass run the prime number checker if required
        // otherwise it will print out on every pass
        if first_pass {
            prime_sieve(program_parameters.limit, program_parameters.bypass_check);
            first_pass = false;
        } else {
            prime_sieve(program_parameters.limit, true);
        }
        no_passes += 1;
        duration += start_of_pass.elapsed().as_millis();
    }

    // NB this code will panic if using really large numbers
    // but large numbers isn't the point of this exercise
    let avg_passes: f64 = duration as f64 / no_passes as f64;
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
        (100_000, 9592),
        (1_000_000, 78_498),
        (10_000_000, 664_579),
        (100_000_000, 5_761_455),
        (1_000_000_000, 50_847_534),
        (10_000_000_000, 455_052_511),
    ]);

    let prime_sieve_length: u64 = prime_sieve.len() as u64;
    if prime_check.contains_key(&prime_sieve_length) {
        let prime_count = prime_sieve.iter().filter(|val| **val).count();
        // need to take away 1 as I've included 1 as a prime - the above table does not
        let prime_count = prime_count - 1;
        println!("FOUND {prime_count} PRIMES");
        prime_count == prime_check[&prime_sieve_length]
    } else {
        false
    }
}
