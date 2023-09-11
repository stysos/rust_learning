fn main() {
    const CANT_CHANGE_ME: u32 = 69420;
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x += 6;
    println!("{x} is x");
    println!("Can't change me {CANT_CHANGE_ME}");
    let x_string: String = x.to_string();
    let x_len = x_string.len();

    let parsley: u32 = x_string.parse().unwrap();
    println!("{parsley} is a number again ?? ");
    println!("A number of this length: {x_len}");
    // CANT_CHANGE_ME += 1;
}
