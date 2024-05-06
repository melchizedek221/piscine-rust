// fn main() {
//     println!("Hello, world!");
// }

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let numbers: Vec<u32> = s
        .split_whitespace()
        .map(|num_str| {
            if num_str.ends_with('k') {
                let without_k = &num_str[..num_str.len() - 1];
                without_k.parse::<f32>().unwrap() * 1000.0
            } else {
                num_str.parse::<f32>().unwrap()
            }
        } as u32)
        .collect();
    Box::new(numbers)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}                        


// fn main() {
    // let new_str = String::from("5.5k 8.9k 32");

    // creating a variable and we save it in the Heap
    // let a_h = transform_and_save_on_heap(new_str);
    // println!("Box value : {:?}", &a_h);
    // println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_h)));

    // let a_b_v = take_value_ownership(a_h);
    // println!("value : {:?}", &a_b_v);
    // println!("size occupied in the stack : {:?} bytes", (std::mem::size_of_val(&a_b_v)));
    // whenever the box, in this case "a_h", goes out of scope it will be deallocated, freed
// }


// $ cargo run
// Box value : [5500, 8900, 32]
// size occupied in the stack : 8 bytes
// value : [5500, 8900, 32]
// size occupied in the stack : 24 bytes
// $