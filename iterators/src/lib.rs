// Structure Collatz
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    // Constructeur pour Collatz
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }

    // Renvoie la prochaine valeur dans la séquence de Collatz
    fn next_val(&mut self) {
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }
    }
}

// Implémentation de l'itérateur pour Collatz
impl Iterator for Collatz {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            None
        } else {
            self.next_val();
            Some(*self)
        }
    }
}

// Fonction qui compte le nombre de pas nécessaires pour atteindre 1
pub fn collatz(n: u64) -> usize {
    let mut collatz_iter = Collatz::new(n);
    let mut counter = 0;

    while let Some(_) = collatz_iter.next() {
        counter += 1;
    }

    counter
}
