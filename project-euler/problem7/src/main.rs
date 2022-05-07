fn main() {
    let mut num = 1;
    let mut place = 10001;
    loop {
        if isprime(num) {
            place -= 1;
            match place {
                0 => {break;}
                _  => {}
            }
        }
        num += 2;
    }
    println!("{}", num);
}

fn isprime(num:u64) -> bool {
    if num == 1 {
        return false;
    }
    for elem in 2..num/2+1 {
        if num % elem == 0{
        return false
        }     
    }
    true
}