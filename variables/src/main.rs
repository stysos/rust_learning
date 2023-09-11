use hex;

fn main() {
    const CANT_CHANGE_ME: u32 = 69420;
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x += 6*10_i32.pow(8);
    println!("{x} is x");
    println!("Can't change me {CANT_CHANGE_ME}");
    let x_string: String = x.to_string();
    let x_len = x_string.len();

    let parsley: u32 = x_string.parse().unwrap();
    println!("{parsley} is a number again ?? ");
    println!("A number of this length: {x_len}");

    let hexy: i32 = 0xff;

    println!("{hexy}");

    let mut password: String = "Hello".to_string();
    password = hex::encode(password);
    println!("{password}");
    // CANT_CHANGE_ME += 1;
    let _longstring: String = "haaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".repeat(127);

    println!("{_longstring}");

    let length_x: i8 = process_string(_longstring).try_into().expect("AHHH");
    
    println!("{length_x}");
}


fn process_string(string: String) -> usize {
    string.len()
}