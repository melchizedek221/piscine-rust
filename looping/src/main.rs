use std::io;

fn main() {
    // println!("Vous avez saisi : {}", input);

    let ans: &str = "The letter e";
    let mut i: i32 = 0;

    loop {
        // println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?
        // ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        i += 1;

        if input.trim() == ans {
            println!("Number of trials: {} ", i);
            break;
        }
    }
}
