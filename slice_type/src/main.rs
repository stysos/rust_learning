fn main() {
    let mut s = String::from("hello world"); // String is in the stack

    let word = first_word(&s);

    println!("{}, {}", s, word);

    let hello = &s[0..first_word(&s)];
    println!("{hello}");
    
    s.clear(); // Empties the string (i.e., s = "")

    println!("{}, {}", s, word); // len saved still



    let word = first_word(&s);
    println!("{}, {}", s, word); // len cleared



    // let world = &s[6..11];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//fn second_word(s: &String) -> (usize, usize) {

//}

