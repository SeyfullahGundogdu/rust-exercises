/*
If we list all the natural numbers below 10 that are multiples of 3 or 5,
we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn main() {
    let sum1: u32 = euler1_v1(1000).iter().sum();
    let sum2: u32 = euler1_v2(1000).iter().sum();
    println!("{}-{}", sum1, sum2);
}

fn euler1_v1(x: u32) -> Vec<u32> {
    (1..x).filter(|n| n%3 == 0 || n%5 == 0).collect()
}

fn euler1_v2(x:u32) -> Vec<u32>{
    let mut vec: Vec<u32> = Vec::new();
    for n in 1..x {
        match n % 3 == 0 || n % 5 == 0{
            true => {vec.push(n);},
            _ => {},
        }
    }
    return vec;
}

#[cfg(test)]
    #[test]
    fn v1() {
        assert_eq!(euler1_v1(10), [3,5,6,9]);
    }
    #[test]
    fn v2(){
        assert_eq!(euler1_v2(10), [3,5,6,9]);
    }