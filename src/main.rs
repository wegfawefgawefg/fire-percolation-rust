use raylib::prelude::*;
use rand;

// enum with three states, null, tree, and fire
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Null,
    Tree,
    Fire,
}

fn main() {
    let screen_dimensions = Vector2::new(800.0, 600.0);

    let (mut rl, thread) = raylib::init()
        .size(screen_dimensions.x as i32, screen_dimensions.y as i32)
        // .fullscreen()
        .title("Forest Fire")
        .build();
    
    
    // 2d grid of 32x32 cells
    let grid_dim = Vector2::new(128.0, 128.0) * 2.0;
    let mut grid = vec![vec![Cell::Null; grid_dim.x as usize]; grid_dim.y as usize];

    // randomly generate trees at 50% chance
    let mut chance = 0.5;
    for y in 0..grid_dim.y as usize {
        for x in 0..grid_dim.x as usize {
            if rand::random::<f32>() < chance {
                grid[y][x] = Cell::Tree;
            }
        }
    }
    // make left line all fire
    for y in 0..grid_dim.y as usize {
        grid[y][0] = Cell::Fire;
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // check if r is pressed
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            // randomly generate trees at 50% chance
            for y in 0..grid_dim.y as usize {
                for x in 0..grid_dim.x as usize {
                    if rand::random::<f32>() < chance {
                        grid[y][x] = Cell::Tree;
                    } 
                    else {
                        grid[y][x] = Cell::Null;
                    }
                }
            }
            // make left line all fire
            for y in 0..grid_dim.y as usize {
                grid[y][0] = Cell::Fire;
            }
        }

        // do up and down arrow to change chance by 0.1
        let chance_change = 0.01;
        if d.is_key_pressed(KeyboardKey::KEY_UP) {
            if chance < 1.0 {
                chance += chance_change;
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if chance > 0.0 {
                chance -= chance_change;
            }
        }

        d.clear_background(Color::WHITE);

        // draw the grid
        let grid_pos = Vector2::new(0.0, 0.0);
        let grid_size = Vector2::new(screen_dimensions.x, screen_dimensions.y);
        d.draw_rectangle(
            grid_pos.x as i32,
            grid_pos.y as i32,
            (grid_size.x) as i32,
            (grid_size.y) as i32,
            Color::BLACK,
        );
        
        let cell_size = Vector2::new(
            grid_size.x / grid_dim.x, 
            grid_size.y / grid_dim.y);
            
        // draw grid
        let cell_size_bias = 5;
        for y in 0..grid_dim.y as usize {
            for x in 0..grid_dim.x as usize {
                let cell_pos = Vector2::new(
                    grid_pos.x + (x as f32 * cell_size.x),
                    grid_pos.y + (y as f32 * cell_size.y),
                );
                let cell_color = match grid[y][x] {
                    Cell::Null => Color::BLACK,
                    Cell::Tree => Color::GREEN,
                    Cell::Fire => Color::RED,
                };
                d.draw_rectangle(
                    cell_pos.x as i32,
                    cell_pos.y as i32,
                    cell_size.x as i32 + cell_size_bias as i32,
                    cell_size.y as i32 + cell_size_bias as i32,
                    cell_color,
                );
            }
        }

        // update the grid
        let mut new_grid = grid.clone();
        for y in 0..grid_dim.y as usize {
            for x in 0..grid_dim.x as usize {
                if grid[y][x] == Cell::Fire {
                    // if cell is fire, set all neighbors to fire
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < grid_dim.x as i32 && ny >= 0 && ny < grid_dim.y as i32 {
                                if grid[ny as usize][nx as usize] == Cell::Tree {
                                    new_grid[ny as usize][nx as usize] = Cell::Fire;
                                }
                            }
                        }
                    }
                }
            }
        }
        grid = new_grid;

        // let itime: f64 = d.get_time() * 1000.0;
        // let time_str = format!("Time: {}", itime);
        let text_size = 20;
        let chance_str = format!("Chance: {}", chance);
        let mut text_cursor = Vector2::new(10.0, 10.0);

        // draw controls
        let text_color = Color::WHITE;
        d.draw_text("Press R to reset", 
            text_cursor.x as i32, text_cursor.y as i32, text_size, text_color);
        text_cursor.y += text_size as f32;
        d.draw_text(format!("Press UP and DOWN to change chance by {}", chance_change).as_str(),
            text_cursor.x as i32, text_cursor.y as i32, text_size, text_color);
        text_cursor.y += text_size as f32;
        text_cursor.y += text_size as f32;

        d.draw_text(&chance_str, 
            text_cursor.x as i32, text_cursor.y as i32, text_size, text_color);
    }
}