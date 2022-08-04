use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub starting_point: Point,
}

impl MapBuilder {
    fn new() -> Self {
        Self {
            map: Map::new(),
            rooms: vec![Rect::default(); NUM_ROOMS],
            starting_point: Point::new(0, 0),
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, &mut rng: RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, (SCREEN_WIDTH as f32 * 0.9) as i32),
                rng.range(1, (SCREEN_HEIGHT as f32 * 0.9) as i32),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;

            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x >= 0
                        && p.x < SCREEN_WIDTH
                        && p.y > 0
                        && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                })
            }

            self.rooms.push(room);
        }
    }

    fn carve_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};

        for y in min(y1, y2)..max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn carve_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};

        for x in min(x1, x2)..max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        // sort rooms by center's x so that tunnels are likely to connect adjacent rooms and
        // not ones randomly across the map.
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous_center = rooms[i - 1].center();
            let current_center = rooms[i].center();

            if rng.range(0, 2) == 1 {
                self.carve_horizontal_tunnel(
                    previous_center.x, current_center.x, previous_center.y);
            }
        }
    }
}

