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

mod areas_volumes;
pub use crate::areas_volumes::*;

type GS = GeometricalShapes;
type GV = GeometricalVolumes;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let target_area = x * y;
    let object_area = times * match objects {
        GS::Square => square_area(a),
        GS::Circle => circle_area(a).ceil() as usize,
        GS::Rectangle => rectangle_area(a, b),
        GS::Triangle => triangle_area(a, b).ceil() as usize,
    };
    object_area <= target_area
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
let target_volume = x * y * z;
let object_volume = times * match objects {
    GV::Cube => cube_volume(a),
    GV::Sphere => sphere_volume(a).ceil() as usize,
    GV::Cone => cone_volume(a, b).ceil() as usize,
    GV::Pyramid => triangular_pyramid_volume(a as f64, b).ceil() as usize,
    GV::Parallelepiped => parallelepiped_volume(a, b, c),
};
object_volume <= target_volume
}
