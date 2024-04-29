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

pub struct One { first_layer: Option<Two> }
pub struct Two { second_layer: Option<Three> }
pub struct Three { third_layer: Option<Four> }
pub struct Four { fourth_layer: Option<u16> }

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref()?.second_layer.as_ref()?.third_layer.as_ref()?.fourth_layer
    }
}