fn main() {
    let mut num = 0;
    /**
     * since the biggest number is 20, we can jump by 20 to reduce computing load, 
     * or reduce it even more by by calculating the prime factors and 
     * multipliying the adding it to the number each time we fail to find a divisible number
     */
    while !divisible(num) {
        num += 20;
        //num += 2*3*5*7*11*13*17*19;
    }
    println!("{}",num);
}

fn divisible(num: u64) -> bool {
    if num == 0 {
        return false
    }
    for elem in 1..20 {
        match num % elem {
            0 => {}
            _ => {return false}
        }
    }
    true
}
