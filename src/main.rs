use std::time::{Duration, Instant};

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_resizable: false, // Prevents user resizing
        fullscreen: false,
        window_width: 640,  // Set the window width to 640 pixels
        window_height: 640, // Set the window height to 640 pixels

        ..Default::default() // Keeps default settings for everything else
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let generations_per_sec = 10.0;
    let mut generation_time = Duration::from_secs_f64(1.0 / generations_per_sec);
    let pause_time = Duration::new(u64::MAX, 0);

    let mut grid_1 = [[false; 64]; 64]; // Create a grid of 64x64 cells initialized to 0
    let mut grid_2 = [[false; 64]; 64]; // Create a second grid for the next generation

    randomise_grid(&mut grid_1); // Randomly populate the first grid

    let mut do_sim; // Flag to control simulation state
    let mut last_frame = Instant::now(); // Reset the frame start time
    loop {
        let elapsed = last_frame.elapsed();
        do_sim = elapsed >= generation_time; // Check if we should simulate based on elapsed time
        if do_sim {
            clear_background(BLACK);
            simulate(&grid_1, &mut grid_2);
            std::mem::swap(&mut grid_1, &mut grid_2);
            last_frame = Instant::now(); // Reset the frame start time
        }

        render(&grid_1);

        if is_key_pressed(KeyCode::P) {
            if generation_time == pause_time {
                generation_time = Duration::from_secs_f64(1.0 / generations_per_sec); // Resume
            } else {
                generation_time = pause_time; // Pause
            }
        }

        if is_key_pressed(KeyCode::R) {
            randomise_grid(&mut grid_1);
        }
        if is_key_pressed(KeyCode::C) {
            grid_1 = [[false; 64]; 64]; // Clear the grid
        }
        next_frame().await;
    }
}

fn randomise_grid(grid: &mut [[bool; 64]; 64]) {
    for y in 0..64 {
        for x in 0..64 {
            grid[x][y] = rand::gen_range(0, 2) == 1;
        }
    }
}

fn simulate(current_grid: &[[bool; 64]; 64], next_grid: &mut [[bool; 64]; 64]) {
    for y in 0..64 {
        for x in 0..64 {
            let cell = current_grid[x][y];
            let alive_neighbors = count_alive_neighbors(&current_grid, x, y);

            // Apply the rules of Conway's Game of Life
            if cell {
                // Cell is alive
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    next_grid[x][y] = false; // Cell dies
                } else {
                    next_grid[x][y] = true; // Cell stays alive
                }
            } else {
                // Cell is dead
                if alive_neighbors == 3 {
                    next_grid[x][y] = true; // Cell becomes alive
                } else {
                    next_grid[x][y] = false; // Cell stays dead
                }
            }
        }
    }
}

fn count_alive_neighbors(grid: &[[bool; 64]; 64], x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the cell itself
            }
            let neighbor_x = x as isize + dx;
            let neighbor_y = y as isize + dy;
            if neighbor_x >= 0 && neighbor_x < 64 && neighbor_y >= 0 && neighbor_y < 64 {
                if grid[neighbor_x as usize][neighbor_y as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn render(grid: &[[bool; 64]; 64]) {
    let cell_size = 10.0; // Size of each cell in pixels
    for y in 0..64 {
        for x in 0..64 {
            let color = if grid[x][y] { WHITE } else { BLACK };
            draw_rectangle(
                x as f32 * cell_size,
                y as f32 * cell_size,
                cell_size,
                cell_size,
                color,
            );
        }
    }
}
