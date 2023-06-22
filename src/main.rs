extern crate piston;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TILE_SIZE: u32 = 32;
const MAX_VISIBILITY_RANGE: f32 = 5.0;
const MIN_FOG_OPACITY: f32 = 0.2;
const SHADOW_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const EXPLORED_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const OBJECT_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

struct Game {
    player_position: (u32, u32),
    object_position: (u32, u32),
    map: Vec<Vec<bool>>,
    fog_of_war: Vec<Vec<(f32, bool)>>,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let player_position = (
            rng.gen_range(0..SCREEN_WIDTH / TILE_SIZE),
            rng.gen_range(0..SCREEN_HEIGHT / TILE_SIZE),
        );
        let object_position = (
            rng.gen_range(0..SCREEN_WIDTH / TILE_SIZE),
            rng.gen_range(0..SCREEN_HEIGHT / TILE_SIZE),
        );

        let map = vec![vec![false; (SCREEN_WIDTH / TILE_SIZE) as usize]; (SCREEN_HEIGHT / TILE_SIZE) as usize];
        let fog_of_war = vec![vec![(1.0, false); (SCREEN_WIDTH / TILE_SIZE) as usize]; (SCREEN_HEIGHT / TILE_SIZE) as usize];

        Self {
            player_position,
            object_position,
            map,
            fog_of_war,
        }
    }

    fn render(&mut self, e: &Event, window: &mut PistonWindow) {
        window.draw_2d(e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            for (y, row) in self.map.iter().enumerate() {
                for (x, &tile) in row.iter().enumerate() {
                    let color = if tile { EXPLORED_COLOR } else { [0.0, 1.0, 0.0, 1.0] };
                    let rect = [x as f64 * TILE_SIZE as f64, y as f64 * TILE_SIZE as f64, TILE_SIZE as f64, TILE_SIZE as f64];
                    rectangle(color, rect, c.transform, g);
                }
            }

            let visibility_range = MAX_VISIBILITY_RANGE;
            for (y, row) in self.fog_of_war.iter_mut().enumerate() {
                for (x, fog_tile) in row.iter_mut().enumerate() {
                    let (opacity, explored) = *fog_tile;
                    let dx = self.player_position.0 as i32 - x as i32;
                    let dy = self.player_position.1 as i32 - y as i32;
                    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

                    if distance <= visibility_range {
                        let alpha = MIN_FOG_OPACITY + (1.0 - MIN_FOG_OPACITY) * (1.0 - distance / visibility_range);
                        let new_opacity = if explored { opacity.min(alpha) } else { alpha };
                        *fog_tile = (new_opacity, true);
                    } else if explored {
                        *fog_tile = (opacity, false);
                    }

                    if !explored && (x as i32 != self.player_position.0 as i32 || y as i32 != self.player_position.1 as i32) {
                        let shadow_color = [SHADOW_COLOR[0], SHADOW_COLOR[1], SHADOW_COLOR[2], opacity];
                        let rect = [x as f64 * TILE_SIZE as f64, y as f64 * TILE_SIZE as f64, TILE_SIZE as f64, TILE_SIZE as f64];
                        rectangle(shadow_color, rect, c.transform, g);
                    }
                }
            }

            let player_rect = [
                self.player_position.0 as f64 * TILE_SIZE as f64,
                self.player_position.1 as f64 * TILE_SIZE as f64,
                TILE_SIZE as f64,
                TILE_SIZE as f64,
            ];
            rectangle([1.0, 0.0, 0.0, 1.0], player_rect, c.transform, g);

            if self.fog_of_war[self.object_position.1 as usize][self.object_position.0 as usize].1 {
                let object_rect = [
                    self.object_position.0 as f64 * TILE_SIZE as f64,
                    self.object_position.1 as f64 * TILE_SIZE as f64,
                    TILE_SIZE as f64,
                    TILE_SIZE as f64,
                ];
                rectangle(OBJECT_COLOR, object_rect, c.transform, g);
            }
        });
    }

    fn handle_input(&mut self, button: &Button) {
        let mut should_update_fog_of_war = false;

        match button {
            Button::Keyboard(Key::W) if self.player_position.1 > 0 => {
                self.player_position.1 -= 1;
                should_update_fog_of_war = true;
            }
            Button::Keyboard(Key::A) if self.player_position.0 > 0 => {
                self.player_position.0 -= 1;
                should_update_fog_of_war = true;
            }
            Button::Keyboard(Key::S) if self.player_position.1 < SCREEN_HEIGHT / TILE_SIZE - 1 => {
                self.player_position.1 += 1;
                should_update_fog_of_war = true;
            }
            Button::Keyboard(Key::Space) => {
                if self.player_position == self.object_position {
                    println!("Interacting with object at ({}, {})", self.player_position.0, self.player_position.1);
                }
            }
            Button::Keyboard(Key::D) if self.player_position.0 > 0 => {
                self.player_position.0 += 1;
                should_update_fog_of_war = true;
            }
            _ => {}
        }

        if should_update_fog_of_war {
            let visibility_range = MAX_VISIBILITY_RANGE;
            for (y, row) in self.fog_of_war.iter_mut().enumerate() {
                for (x, fog_tile) in row.iter_mut().enumerate() {
                    let (opacity, explored) = *fog_tile;
                    let dx = self.player_position.0 as i32 - x as i32;
                    let dy = self.player_position.1 as i32 - y as i32;
                    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

                    if distance <= visibility_range {
                        let alpha = MIN_FOG_OPACITY + (1.0 - MIN_FOG_OPACITY) * (1.0 - distance / visibility_range);
                        let new_opacity = if explored { opacity } else { alpha };
                        *fog_tile = (new_opacity, true);
                    } else if explored {
                        *fog_tile = (opacity, false);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fog of War Example", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            game.handle_input(&button);
        }

        game.render(&e, &mut window);
    }
}
