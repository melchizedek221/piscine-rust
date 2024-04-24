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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    if let Some(light) = lights
        .iter_mut()
        .find(|light| light.alias == alias) {
            light.brightness = value;
        }
}

// fn main() {
//     // Bedroom
//     let mut lights = vec![
//         Light::new("living_room"),
//         Light::new("bedroom"),
//         Light::new("rest_room"),
//     ];
//     println!("brightness = {}", lights[0].brightness);
//     change_brightness(&mut lights, "living_room", 200);
//     println!("new brightness = {}", lights[0].brightness);
// }
