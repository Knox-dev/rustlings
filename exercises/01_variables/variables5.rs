// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

/// En Rust, on ne peut pas changer le type de la variable 
/// une foi créée, il faut la recréer

fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
