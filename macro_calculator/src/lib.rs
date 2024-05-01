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


pub struct Food {
    name: String,
    calories: Vec<String>,
    fats: f64,
    carbs: f64,
    proteins: f64,
    nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> String {
    let mut total_calories = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let calories_kj = food.calories[0].parse::<f64>().unwrap();
        let calories_kcal = food.calories[1].parse::<f64>().unwrap();
        let calories_per_portion = (calories_kj + calories_kcal) / food.nbr_of_portions;

        total_calories += calories_per_portion;
        total_carbs += food.carbs;
        total_proteins += food.proteins;
        total_fats += food.fats;
    }

    let mut json_string = String::new();
    json_string.push_str("{\n");

    let macros = vec["cals", "carbs", "proteins", "fats"];
    let mut first = true;

    for macro_name in macros {
        if first {
            first = false;
        } else {
            json_string.push_str(",\n");
        }

        json_string.push_str("\"");
        json_string.push_str(macro_name);
        json_string.push_str("\": ");
        json_string.push_str(&format!("{:.1}", match macro_name {
            "cals" => total_calories,
            "carbs" => total_carbs,
            "proteins" => total_proteins,
            "fats" => total_fats,
            _ => 0.0,
        }));
    }

    json_string.push_str("\n}");

    json_string
}
