// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

/// etape 1: déclarer un array, étape 2: utiliser la macro 'println!'

fn main() {
    let a = [1,2,5,9];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        println!("Array not big enough, more elements needed");
    }
}
