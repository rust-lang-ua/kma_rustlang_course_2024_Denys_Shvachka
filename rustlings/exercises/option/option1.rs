// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM DONE

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));


}
