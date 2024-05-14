// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// use std::ops::{Index, IndexMut};
pub trait Scalar: Sized{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0
    }
    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.
    }
    fn one() -> Self::Item {
        1.
    }
}
impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.
    }
    fn one() -> Self::Item {
        1.
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
    // La méthode 'new' crée une nouvelle matrice de taille 1x1 initialisé avec zéro.
    pub fn new() -> Matrix<T> {
        // T::zero() crée un élément zéro de type T
        // vec![T::zero()] crée un vecteur avec un seul élément qui est zéro.
        // vec![vec![T::zero()]] crée un vecteur de vecteurs (une matrice 1x1) avec un seul élément qui est zéro.
        Matrix(vec![vec![T::zero()]])
    }

    // La méthode 'zero' crée une nouvelle matrice de taille 'row' x 'col' remplie de zéros.
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        // vec![T::zero(); col] crée un vecteur de longueur 'col', où chaque élément est un zéro de type T.
        // vec![*; row] crée un vecteur de longueur 'row', où chaque élément est le vecteur précédemment créé.
        // C'est une matrice 'row' x 'col', remplie de zéros.
        Matrix(vec![vec![T::zero(); col]; row])
    }

    // La méthode 'identity' crée une matrice identité de taille 'n' x 'n'.
    pub fn identity(n: usize) -> Matrix<T> {
        // Créer une matrice 'n' x 'n', remplie de zéros.
        let mut mat = vec![vec![T::zero(); n]; n];
        
        // Pour chaque indice 'i' de 0 à 'n',
        for i in 0..n {
            // Rend l'élément à la position [i][i] égal à un.
            mat[i][i] = T::one();
        }
        
        // Retourne la matrice créée.
        Matrix(mat)
    }
}
