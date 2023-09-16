fn main() {

    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            println!("{x} {y}");
            if x == 2 {
                break 'outer;
            }
        }
    }

    conditional_loops();

    for_loop();
}

fn conditional_loops() { 
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;

    }

    println!("Lift off !!! ");
}

fn for_loop() {
    // Rip python

    let a: Vec<i32> = (10..60).step_by(10).rev().collect();
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

