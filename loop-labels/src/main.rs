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

    let person = Person::new("Bob", 30);
    person.greet();

    if Person::is_adult(person.age) {
        println!("{} is an adult", person.name);
    } else {
        println!("{} is not an adult", person.name);
    }
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

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person { 
            name: name.to_string(),
            age
        }
    }

    fn greet(&self) {
        println!("Hello my name is {} and I am {} years old", self.name, self.age)
    }

    fn is_adult(age: u8) -> bool {
        age >= 18
    }
}