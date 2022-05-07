/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * 
 * What is the largest prime factor of the number 600851475143 ?
 */
fn main() {
    println!("{:?}", factor(600_851_475_143));
}

fn factor(mut num: i64) ->i64 {
    let mut divider:i64 = 2;
    while num != 1 {
        while num % divider == 0 {
            num = num/divider;
        }
        divider += 1; 
    }
    divider - 1
}

//some tests for factorization
#[test]
fn name() {
    assert_eq!(factor(10),5);
    assert_eq!(factor(11),11);
    assert_eq!(factor(36),3);
    assert_eq!(factor(82),41);
    assert_eq!(factor(600_851_475_143),6857);
}
