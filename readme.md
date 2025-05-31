# Implementation of the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

The program runs with a set of default values. </br>
The command line arguments are: </br>
<code>
Usage: prime_sieve [OPTIONS]<br>
</br>
Options:</br>
-s, --seconds <SECONDS> number of seconds to run for [default: 5]</br>
-l, --limit <LIMIT> maximum number of primes to look for [default: 10000]</br>
-b, --bypass-check</br>
-h, --help Print help</br>
-V, --version Print version</br>
</code>

There is room for a small optimization. At the moment it checks for and removes multiples of 2. You could implement the vector without multiples of 2. This would halve the vector size and remove the initial divide by 2 calculation.