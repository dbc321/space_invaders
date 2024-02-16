use crate::aabb::AABB;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn intersects(obj_1: AABB, obj_2: AABB) -> bool {
    obj_1.x < obj_2.x + obj_2.width &&
    obj_1.x + obj_1.width > obj_2.x &&
    obj_1.y < obj_2.y + obj_2.height &&
    obj_1.y + obj_1.height > obj_2.y
}