// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

/// il faut modifier la fonction afin qu'elle prenne le vecteur 
/// par référence

#[test]
fn test_fill_vec() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0); // Passer vec0 par référence à fill_vec

    assert_eq!(vec0, vec![22, 44, 66]); // Vérifier que vec0 n'a pas été modifié
    assert_eq!(vec1, vec![22, 44, 66, 88]); // Vérifier que vec1 contient les éléments attendus
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone(); // Cloner le vecteur pour pouvoir le modifier
    new_vec.push(88); // Ajouter l'élément 88 au vecteur cloné

    new_vec // Retourner le vecteur modifié
}
