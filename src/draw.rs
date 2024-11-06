use crate::button;
use crate::constants;
use crate::tile;
use constants::{grid, buttons};
use macroquad::prelude::*;


pub struct VisualHandler {
    pub zoom_level : u16,
    pub grid : [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
    pub buttons : [button::Button; buttons::NUM_BUTTONS]
}


impl VisualHandler {
    pub fn new(zoom_level: u16, grid: [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES], buttons: [button::Button; buttons::NUM_BUTTONS]) -> Self {
        VisualHandler {
            zoom_level,
            grid,
            buttons,
        }
    }

    pub fn zoom_grid(&mut self) {
        let tile_size = grid::size() / self.zoom_level as f32;
        let center = grid::NUM_TILES / 2;
        let i = center.saturating_sub(self.zoom_level as usize / 2);
        let j = (center + self.zoom_level as usize / 2).min(grid::NUM_TILES);
    
        let mut y_pos = grid::y_pos() as f32;
        for row in i..j {
            let mut x_pos: f32 = grid::x_pos() as f32;
            for col in i..j {
                self.grid[row][col].x = x_pos;
                self.grid[row][col].y = y_pos;
                self.grid[row][col].size = tile_size;
                x_pos += tile_size;
            }
            y_pos += tile_size;
        }
    }

    pub fn draw_grid(&self, mode: Color) {
        draw_rectangle(
            grid::x_pos() - grid::border_padding(),
            grid::y_pos() - grid::border_padding(),
            grid::size() + grid::border_padding() * 2.0,
            grid::size() + grid::border_padding() * 2.0,
            mode,
        );
    
        let center = grid::NUM_TILES / 2;
        let i = center.saturating_sub(self.zoom_level as usize / 2);
        let j = (center + self.zoom_level as usize / 2).min(grid::NUM_TILES);
    
        for row in i..j {
            for col in i..j {
                let mut tile = self.grid[row][col];
                tile.draw();
            }
        }
    
        draw_rectangle_lines(grid::x_pos(), grid::y_pos(), grid::size(), grid::size(), 3.0, BLACK);

        for button in &self.buttons {
            button.draw(mode);
        }
    }
}