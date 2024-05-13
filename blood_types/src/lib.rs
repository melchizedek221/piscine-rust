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

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

// use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid Antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid RhFactor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            other_order => other_order,
        }
    }
}


impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || s.len() > 3 {
            return Err(String::from("Invalid length for BloodType string"));
        }

        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = antigen_str.parse().map_err(|e| format!("Failed to parse antigen: {}", e))?;
        let rh_factor = rh_str.parse().map_err(|e| format!("Failed to parse RhFactor: {}", e))?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh_sign = match &self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", &self.antigen, rh_sign)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, donor: &BloodType) -> bool {
        let is_antigen_compatible = 
            (self.antigen == Antigen::A && (donor.antigen == Antigen::A || donor.antigen == Antigen::O)) ||
            (self.antigen == Antigen::B && (donor.antigen == Antigen::B || donor.antigen == Antigen::O)) ||
            (self.antigen == Antigen::AB) ||
            (self.antigen == Antigen::O && donor.antigen == Antigen::O);
    
        let is_rh_compatible =
            (self.rh_factor == RhFactor::Positive) ||
            (self.rh_factor == RhFactor::Negative && donor.rh_factor == RhFactor::Negative);
    
        is_antigen_compatible && is_rh_compatible
    }

    pub fn donors(&self) -> Vec<BloodType> {
        let mut donors = Vec::new();
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood_type in all_blood_types {
            if self.can_receive_from(&blood_type) {
                donors.push(blood_type);
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        let all_blood_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood_type in all_blood_types {
            if blood_type.can_receive_from(self) {
                recipients.push(blood_type);
            }
        }

        recipients
    }
}