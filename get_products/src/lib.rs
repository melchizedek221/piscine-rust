pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return vec![1];
    }
    
    let product: usize = arr.iter().product();
    
    arr.into_iter().map(|n| product / n).collect::<Vec<usize>>()
}