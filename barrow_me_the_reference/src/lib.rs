pub fn delete_and_backspace(s: &mut String) {
    while let Some(i) = s.find('-') {
        if i > 0 {
            s.replace_range(i - 1..=i, "");
        } else {
            s.replace_range(i..=i, ""); 
        }
    }

    while let Some(i) = s.rfind('+') {
        if i < s.len() - 1 {
            s.replace_range(i..=i + 1, "");
        } else {
            s.replace_range(i..=i, ""); 
        }
    }
}

pub fn do_operations(v: &mut Vec<String>) {
    for i in 0..v.len() {
        let equation = &v[i];
        if equation.contains('+') {
            let operands: Vec<&str> = equation.split('+').collect();
            let result = operands[0].parse::<i32>().unwrap() + operands[1].parse::<i32>().unwrap();
            v[i] = result.to_string();
        } else if equation.contains('-') {
            let operands: Vec<&str> = equation.split('-').collect();
            let result = operands[0].parse::<i32>().unwrap() - operands[1].parse::<i32>().unwrap();
            v[i] = result.to_string();
        }
    }
}

// fn main() {
// 	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
//     let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"]
//         .iter()
//         .map(|s| s.to_string())
//         .collect();

//     delete_and_backspace(&mut a);
//     do_operations(&mut b);

//     println!("{:?}", (a, b));
// }
