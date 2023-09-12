fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition is true!");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was {number}");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");

    loop {
        println!("Again!!!");
        break
    
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}
