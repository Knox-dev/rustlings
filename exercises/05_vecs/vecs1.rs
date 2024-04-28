// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

/// convertir l'array en vecteur avant de le comparer à un autre vecteur!

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [4,45, 45, 56]; // a plain array
    let v: Vec<i32> = vec![4,45, 45, 56]; 


    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();

        let a_as_vec: Vec<i32> = a.to_vec();
        assert_eq!(a_as_vec, v);
    }
}
