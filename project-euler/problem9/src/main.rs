use core::f64;
/**
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a2 + b2 = c2
 * 
 * For example, 32 + 42 = 9 + 16 = 25 = 52.
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * 
 * Find the product abc.

 */
fn main() {
    println!("{:?}",pythagorean());

}

fn pythagorean() -> (u64,u64,u64) {
    let (mut a, mut b) = (1.0,1.0);
    let mut c = 0.0;
    let mut sum = 0.0;
    while sum != 1000.0 {
        b += 1.0;
        if b >= 1000.0 { //b > 1000;
            a += 1.0;
            b = a;
        }
        c= ((a*a+b*b) as f64).sqrt();
        sum = a + b + c;
    }
    (a as u64,b as u64,c as u64)
}