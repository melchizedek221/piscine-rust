#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64; // Le type de valeur qui est retournée à chaque itération.

    fn next(&mut self) -> Option<u64> {
        // Si la valeur est 1 ou 0, nous avons atteint la fin de la séquence.
        if self.v == 1 || self.v == 0 {
            None
        } else if self.v % 2 == 0 { // Si la valeur est pair.
            self.v /= 2;
            Some(self.v)
        } else { // Si la valeur est impair.
            self.v = 3*self.v + 1;
            Some(self.v)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let collatz_iter = Collatz::new(n);
    collatz_iter.count() // count() renvoie le nombre d'éléments dans l'itérateur
}