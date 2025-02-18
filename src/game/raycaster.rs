use std::f32::consts::PI;

use macroquad::{
    color::{BLUE, GREEN},
    math::{vec2, Vec2},
    shapes::draw_rectangle,
};

use super::map::{self, Map};

pub struct Raycaster();
impl Raycaster {
    pub fn raycast(map: &Map, pos: Vec2, angle: f32, length: f32) -> (f32, Vec2) {
        let mut reached_pos = Vec2::new(
            (pos.x / map.map_item_width).floor(),
            (pos.y / map.map_item_height).floor(),
        );
        let sin: f32 = angle.sin() * (map.map_item_width / map.map_item_height);
        let cos: f32 = angle.cos();
        let h_length_offset = map.map_item_width / sin.abs();
        let v_length_offset = map.map_item_height / cos.abs();

        let mut h_length = (if (angle >= 0.0 && angle < PI) || angle >= 2.0 * PI {
            reached_pos.y + 1.0
        } else {
            reached_pos.y
        } * map.map_item_height
            - pos.y)
            / sin;
        let mut v_length = (if angle <= PI / 2.0 || angle >= 3.0 * PI / 2.0 {
            reached_pos.x + 1.0
        } else {
            reached_pos.x
        } * map.map_item_width
            - pos.x)
            / cos;
        let mut is_h;
        loop {
            is_h = 0;
            if h_length >= length && v_length >= length {
                break;
            }
            if h_length <= v_length {
                reached_pos.y += angle.sin().signum();
                h_length += h_length_offset;
                is_h = 1;
            } else {
                reached_pos.x += angle.cos().signum();
                v_length += v_length_offset;
                is_h = 2;
            }
            if reached_pos.y >= 0.0
                && map.map_height > reached_pos.y as usize
                && reached_pos.x >= 0.0
                && map.map_width > reached_pos.x as usize
            {
                if map.map[reached_pos.y as usize][reached_pos.x as usize] != 0 {
                    break;
                }
            }
        }
        let max_len = if is_h == 0 {
            length
        } else if is_h == 1 {
            h_length - h_length_offset
        } else {
            v_length - v_length_offset
        };
        return (max_len, vec2(max_len * angle.cos(), max_len * angle.sin()));
    }
}
