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

// use chrono::{Datelike, Weekday};

// pub fn middle_day(year: i32) -> Option<Weekday> {
//     let is_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

//     let days_in_year = if is_leap_year { 366 } else { 365 };

//     if days_in_year % 2 == 0 {
//         return None;
//     }

//     let middle_day = days_in_year / 2;

//     let middle_date = chrono::NaiveDate::from_ymd(year, 1, 1)
//         + chrono::Duration::days(middle_day as i64);

//     Some(middle_date.weekday())
// }

// fn middle_day(year: i32) -> Option<Weekday> {

//     let start_date = NaiveDate::from_ymd(year, 1, 1);
//     let end_date = NaiveDate::from_ymd(year, 12, 31);
//     let total_days = (end_date - start_date).num_days() + 1;

//     if total_days % 2 == 0 {
//         return None;
//     }

//     let middle_day = start_date + chrono::Duration::days(total_days / 2);

//     Some(middle_day.weekday())
// }

// fn main() {
//     println!("{:?}", middle_day(1022).unwrap());
// }


extern crate chrono;
use chrono::{Utc, Datelike, TimeZone, LocalResult};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    // Vérifier si l'année est bissextile
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    // Si l'année est bissextile et a 366 jours ou non bissextile et a 365 jours, retourner None
    if is_leap && 366 % 2 == 0 || !is_leap && 365 % 2 == 0 {
        return None;
    }

    // Calculer le jour du milieu de l'année
    let middle_day = if is_leap {
        366 / 2 + 1
    } else {
        365 / 2 + 1
    };

    // Créer une date à partir de l'année spécifiée
    let date = match Utc.ymd(year, 1, 1) {
        LocalResult::Single(dt) => dt,
        _ => return None,
    };

    // Ajouter le nombre de jours nécessaires pour atteindre le jour du milieu
    let new_date = date + chrono::Duration::days(middle_day as i64 - 1);
    Some(new_date.weekday())
}
