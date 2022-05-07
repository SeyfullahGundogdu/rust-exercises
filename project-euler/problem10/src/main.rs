//The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//Find the sum of all the primes below two million.

fn main() {
    println!("{}",subprimes(10));
}

//https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
fn subprimes(num:u64) -> u64{
    let mut sum = 0;
    let mut sieve:Vec::<bool> = vec![true;num as usize];
    for i in 2..num {
        if sieve[i as usize] {
            sum+= i;
            for j in (i*i..num).step_by(i as usize) {
                sieve[j as usize] = false
            }
        }        
    }
    sum
}