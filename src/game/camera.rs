use std::{cmp::Ordering, f32::consts::PI, vec};

use macroquad::{
    color::{Color, RED},
    input::mouse_delta_position,
    math::{clamp, Rect, Vec2},
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{screen_height, screen_width},
};

use super::{entities::SpritedEntityData, map::Map, raycaster::Raycaster};

pub const VIEW_ANGLE_STEP: f32 = 0.1;
pub const VIEW_ANGLE: f32 = 90.0;
pub struct Camera {
    glance_len: f32,
    angle: f32,
    horizont_line: f32,
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
            horizont_line: screen_height() / 2.0,
        };
    }
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn update(&mut self) {
        self.key_handling();
    }
    pub fn sort_by_camera(&self, entities: &mut Vec<&SpritedEntityData>) {
        entities.sort_by(|a, b| -> Ordering {
            if ((a.pos.x - self.pos.x).powf(2.0) + (a.pos.y - self.pos.y).powf(2.0)).sqrt()
                > ((b.pos.x - self.pos.x).powf(2.0) + (b.pos.y - self.pos.y).powf(2.0)).sqrt()
            {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
    }
    pub fn draw_all_entites(&self, entities: &Vec<&SpritedEntityData>) {
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
                        self.horizont_line - perspective_size.y / 2.0,
                        Color {
                            r: entity.color.r * (100.0 - dist) / 90.0,
                            g: entity.color.g * (100.0 - dist) / 90.0,
                            b: entity.color.b * (100.0 - dist) / 90.0,
                            a: entity.color.a,
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

    pub fn draw_walls(&mut self, map: &Map) {
        self.wall_dists.clear();
        let mut angle: f32 = -self.fov / 2.0;
        while angle < self.fov / 2.0 {
            let (dist, coords) = Raycaster::raycast(
                map,
                self.pos,
                self.angle + angle.to_radians(),
                self.glance_len as f32,
            );
            let corrected_dist = dist * (angle.to_radians()).cos();
            let y_offset = 2.0 * screen_height() / (corrected_dist);
            self.draw_fov(coords, angle);
            draw_rectangle(
                screen_width() * ((angle + self.fov / 2.0) / self.fov),
                self.horizont_line - y_offset,
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
    fn draw_fov(&self, fov_coords: Vec2, angle: f32) {
        if angle == -self.fov / 2.0 {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + fov_coords.x,
                self.pos.y + fov_coords.y,
                2.0,
                RED,
            );
        } else if angle + self.view_angle_step >= self.fov / 2.0 {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + fov_coords.x,
                self.pos.y + fov_coords.y,
                2.0,
                RED,
            );
        }
        draw_circle(
            self.pos.x + fov_coords.x,
            self.pos.y + fov_coords.y,
            2.0,
            RED,
        );
    }
    fn key_handling(&mut self) {
        self.horizont_line += mouse_delta_position().y * 400.0;
        if self.horizont_line > screen_height() {
            self.horizont_line = screen_height();
        } else if self.horizont_line < 0.0 {
            self.horizont_line = 0.0;
        }
    }
    pub fn draw_env(&self) {
        let start_y = self.horizont_line;
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            start_y,
            Color {
                r: 0.27,
                g: 0.52,
                b: 0.73,
                a: 1.0,
            },
        );
        draw_text(
            &format!("HorLine {}", mouse_delta_position().y),
            5.0,
            150.0,
            30.0,
            RED,
        );
        let y_offset = screen_height() / (2.0 * 100.0);
        let mut current_y = start_y;
        let ground_height = screen_height() - start_y;
        while current_y < screen_height() {
            draw_rectangle(
                0.0,
                current_y,
                screen_width(),
                y_offset,
                Color {
                    r: ((current_y + 30.0 - start_y) / ground_height) * 1.0 + 0.02,
                    g: ((current_y + 30.0 - start_y) / ground_height) * 1.0 + 0.02,
                    b: ((current_y + 30.0 - start_y) / ground_height) * 1.0 + 0.02,
                    a: 1.0,
                },
            );
            current_y += y_offset;
        }
    }
}
