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

pub struct One { pub first_layer: Option<Two> }
pub struct Two { pub second_layer: Option<Three> }
pub struct Three { pub third_layer: Option<Four> }
pub struct Four { pub fourth_layer: Option<u16> }

// impl One {
//     pub fn get_fourth_layer(&self) -> Option<u16> {
//         match &self.first_layer {
//             Some(two) => match &two.second_layer {
//                 Some(three) => match &three.third_layer {
//                     Some(four) => four.fourth_layer,
//                     None => None,
//                 },
//                 None => None,
//             },
//             None => None,
//         }
//     }
// }

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref()?.second_layer.as_ref()?.third_layer.as_ref()?.fourth_layer
    }
}