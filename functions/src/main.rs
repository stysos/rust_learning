fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labelled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + five()
    };


    println!("The value of y is {y}");
}


fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}
// there is 
// no multi-line comments
// sad