// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

/// Dans la déclaration de variables au seon d'une fonction,
/// leurs type respectifs doit être également précisé

fn main() {
    call_me(3);
}

fn call_me(num: i32) { // déclaration du type dans la liste de paramètres
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
