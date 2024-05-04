pub fn spell(n: u64) -> String {
    if n < 10 {
        return match n {
            0 => "zero".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            _ => unreachable!(),
        };
    } else if n < 20 {
        return match n {
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            _ => unreachable!(),
        };
    } else if n < 100 {
        let tens = match n / 10 {
            2 => "twenty".to_string(),
            3 => "thirty".to_string(),
            4 => "forty".to_string(),
            5 => "fifty".to_string(),
            6 => "sixty".to_string(),
            7 => "seventy".to_string(),
            8 => "eighty".to_string(),
            9 => "ninety".to_string(),
            _ => unreachable!(),
        };
        let ones = if n % 10 != 0 {
            format!("-{}", spell(n % 10))
        } else {
            String::new()
        };
        return format!("{}{}", tens, ones);
    } else {
        let big_units = ["", "thousand", "million"];
        let mut result = String::new();
        let mut i = 0;
        let mut n = n;
        while n > 0 {
            let group = n % 1000;
            if group > 0 {
                if !result.is_empty() {
                    result.insert_str(0, " ");
                }
                result.insert_str(0, &format!("{} {}", spell(group), big_units[i as usize]));
            }
            n /= 1000;
            i += 1;
        }
        return result;
    }
}
