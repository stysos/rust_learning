fn main() {

    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            println!("{x} {y}");
            if x == 2 {
                break 'outer;
            }
        }
    }
}