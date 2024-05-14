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

pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

impl<'a> Vehicle for Truck<'a> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

impl<'a> Vehicle for Car<'a> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    list.into_iter().map(|v| v.model()).collect()
}

// fn main() {
//     let vehicles: Vec<&dyn Vehicle> = vec![
//         &Car {
//             plate_nbr: "A3D5C7",
//             model: "Model 3",
//             horse_power: 325,
//             year: 2010,
//         },
//         &Truck {
//             plate_nbr: "V3D5CT",
//             model: "Ranger",
//             horse_power: 325,
//             year: 2010,
//             load_tons: 40,
//         },
//     ];
//     println!("{:?}", all_models(vehicles));
// }