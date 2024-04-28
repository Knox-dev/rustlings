// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

/// lorsque la valeur de x doit évoluer, il faut la déclarer comme mutable

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
