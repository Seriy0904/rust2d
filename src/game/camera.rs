use std::{cmp::Ordering, f32::consts::PI, vec};

use macroquad::{
    color::Color,
    math::{clamp, Rect, Vec2},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{screen_height, screen_width},
};

use super::{entities::SpritedEntityData, map::Map, raycaster::Raycaster};

pub const VIEW_ANGLE_STEP: f32 = 0.1;
pub const VIEW_ANGLE: f32 = 90.0;
pub struct Camera {
    glance_len: f32,
    angle: f32,
    pos: Vec2,
    view_angle_step: f32,
    fov: f32,
    wall_dists: Vec<usize>,
}
impl Camera {
    pub fn new(glance_len: f32) -> Camera {
        return Camera {
            glance_len,
            angle: 0.0,
            pos: Vec2::ZERO,
            view_angle_step: VIEW_ANGLE_STEP,
            fov: VIEW_ANGLE,
            wall_dists: vec![],
        };
    }
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn draw_all_entites(&self, mut entities: Vec<&SpritedEntityData>) {
        entities.sort_by(|a, b| -> Ordering {
            if ((a.pos.x - self.pos.x).powf(2.0) + (a.pos.y - self.pos.y).powf(2.0)).sqrt()
                > ((b.pos.x - self.pos.x).powf(2.0) + (b.pos.y - self.pos.y).powf(2.0)).sqrt()
            {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
        for entity in entities {
            self.draw_entity(&entity);
        }
    }
    fn draw_entity(&self, entity: &SpritedEntityData) {
        let dif = entity.pos - self.pos;
        let mut dist = dif.length();

        let wall_width = screen_width() / (self.wall_dists.len()) as f32;

        if dist < 1.0 {
            dist = 1.0;
        }
        let perspective_size = entity.size * 40.0 / (dist);
        let x_pos = self.x_from_dif(dif);
        if x_pos + perspective_size.x / 2.0 >= 0.0
            && x_pos - perspective_size.x / 2.0 <= screen_width()
        {
            let start_wall = ((x_pos - perspective_size.x / 2.0) / wall_width).ceil() as i32;
            let last_wall = ((x_pos + perspective_size.x / 2.0) / wall_width).floor() as i32;
            for current_wall in (start_wall)..(last_wall) {
                if current_wall >= self.wall_dists.len() as i32 || current_wall <= 0 {
                    continue;
                }
                if self.wall_dists[current_wall as usize] as f32 > dist {
                    draw_texture_ex(
                        &(entity.texture.clone()),
                        current_wall as f32 * wall_width,
                        screen_height() / 2.0 - perspective_size.y / 2.0,
                        Color {
                            r: 40.0 / dist,
                            g: 40.0 / dist,
                            b: 40.0 / dist,
                            a: 1.0,
                        },
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(wall_width, perspective_size.y)),
                            source: Some(Rect::new(
                                // 0.0,
                                (current_wall - start_wall) as f32 * entity.texture.width()
                                    / ((last_wall - start_wall) as f32),
                                0.0,
                                entity.texture.width() / ((last_wall - start_wall) as f32),
                                entity.texture.height(),
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
    fn x_from_dif(&self, dif: Vec2) -> f32 {
        let mut start_angle = dif.y.atan2(dif.x);
        if start_angle < 0.0 {
            start_angle += 2.0 * PI;
        };
        let min_angle = self.angle - (VIEW_ANGLE / 2.0).to_radians();
        let max_angle = self.angle + (VIEW_ANGLE / 2.0).to_radians();
        let max_enemy_angle = if max_angle > PI && start_angle < PI {
            start_angle + 2.0 * PI
        } else {
            start_angle
        };
        let min_enemy_angle = if min_angle < PI && start_angle > PI {
            start_angle - 2.0 * PI
        } else {
            start_angle
        };
        let real_enemy_angle = if max_enemy_angle - max_angle < min_angle - min_enemy_angle {
            max_enemy_angle
        } else {
            min_enemy_angle
        };
        let x_pos = (real_enemy_angle - min_angle) * screen_width() / (VIEW_ANGLE.to_radians());
        return x_pos;
    }

    pub fn draw_map(&mut self, map: &Map) {
        self.wall_dists.clear();
        let mut angle: f32 = -self.fov / 2.0;
        while angle < self.fov / 2.0 {
            let (dist, _coords) = Raycaster::raycast(
                map,
                self.pos,
                self.angle + angle.to_radians(),
                self.glance_len as f32,
            );
            let corrected_dist = dist * (angle.to_radians()).cos();
            let y_offset = 2.0 * screen_height() / (corrected_dist);
            draw_rectangle(
                screen_width() * ((angle + self.fov / 2.0) / self.fov),
                screen_height() / 2.0 - y_offset,
                screen_width() / (self.fov / self.view_angle_step),
                y_offset * 2.0,
                // 2.0,
                Color {
                    r: clamp(30.0 / (corrected_dist), 0.0, 0.9),
                    g: clamp(30.0 / (corrected_dist), 0.0, 0.9),
                    b: 0.0, //clamp(30.0 / (wall_pos), 0.0, 0.9),
                    a: 1.0,
                },
            );
            self.wall_dists.push(dist as usize);
            angle += self.view_angle_step;
        }
    }
    // raycaster
}
