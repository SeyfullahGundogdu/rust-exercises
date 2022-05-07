fn main() {
    println!("{}",diff(100));
}

fn diff(num:u64) -> u64 {
    let mut sumsqr = 0;
    let mut sqrsum = 0;
    for elem in 1..=num {
        sumsqr += elem;
        sqrsum += elem * elem;
    }
    sumsqr*sumsqr -sqrsum
}
