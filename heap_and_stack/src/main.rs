fn main() {
    let s = "Hello"; // This is a str literal (Stack; pre-determined size and immutable)

    let mut _str = String::from(s); // This is a String (Heap; variable size)

    println!("{s}");
    println!("{}", type_of(&s));

    println!("{_str}");
    println!("{}", type_of(&_str));

    _str += " Potato";

    println!("{_str}");

    println!("{}", &_str); // & = retrieve from pointer (Borrow)


    // let __str = _str; // Moves the pointer to __str, cannot have two pointers to heap variable
    
    let __str = _str.clone(); // Copies the heap data and creates another copy of it within the heap

    println!("{__str}");

    println!("{_str}"); 

    let x = 5;

    let y = x;

    println!("{}", x); // Int are in stack
    println!("{}", type_of(&x));
    println!("{}", y); // So both remain valid without being copied, as rust copies behind the scenes
                        // Without performance loss!
    println!("{}", type_of(&y));

    let new_str = variable_stealer(_str);

    // println!("{}", _str) _str no longer exists if variable stealer takes it!!

    println!("{new_str} no longer the same place in memory!");
    not_variable_stealer(&new_str); // But we can borrow

    println!("{new_str} is still safely here");

}

fn variable_stealer(some_string: String) -> String {
    println!("{} variable stolen!!!", some_string);
    some_string
}

fn not_variable_stealer(some_string: &String) {
    println!("{} variable safe!", some_string)
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}