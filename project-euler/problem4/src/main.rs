fn main() {
    //start by 999*999
    let (mut a, mut b) = (999, 999);
    let mut mul = a * b;
    let mut rev = 0;
    while a > 99 && b > 99 {
        //check if the product is a palindrome 
        if reversenum(mul) == mul && mul > rev {
            //save the value if greater then the previous one
            rev = mul;
        }
        //decrement b by one, when reached to 99( 2 digits) start over but decrease a by 1;
        b -= 1;
        match b {
            99 => {
                a -= 1;
                b = a;
            },
            _ => {},
        }
        mul = a * b;
    }
    println!("{}", rev);
}
/**
 * return the inverse of a number
 */
fn reversenum(mut number: u64) -> u64 {
    let mut digit;
    let mut invnum = 0;
    while number != 0 {
        digit = number % 10;
        invnum = invnum * 10 + digit;
        number /= 10;
    }
    invnum
}
