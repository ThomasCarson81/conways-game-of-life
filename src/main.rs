use std::time::{Duration, Instant};

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_resizable: false, // Prevents user resizing
        fullscreen: false,
        window_width: 640,    // Set the window width to 640 pixels
        window_height: 640,   // Set the window height to 640 pixels
        ..Default::default()  // Keeps default settings for everything else
    }
}

const CELL_SIZE: f32 = 10.0; // Size of each cell in pixels
const GRID_WIDTH: usize = 64; // Number of cells in the grid width
const GRID_HEIGHT: usize = 64; // Number of cells in the grid height

type Grid = [u64; GRID_HEIGHT]; // 64x64 grid, bits in each u64 = x, each u64 = y

fn empty_grid() -> Grid {
    [0; 64] // Initialize a grid of 64 rows, each with 64 bits set to 0
}

fn get(grid: &Grid, x: usize, y: usize) -> bool {
    (grid[y] >> x) & 1 == 1
}

fn set(grid: &mut Grid, x: usize, y: usize, alive: bool) {
    if alive {
        grid[y] |= 1 << x;
    } else {
        grid[y] &= !(1 << x);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let generations_per_sec = 30.0;
    let mut generation_time = Duration::from_secs_f64(1.0 / generations_per_sec);
    let pause_time = Duration::new(u64::MAX, 0);

    let mut grid_1 = empty_grid(); // Create a grid of 64x64 cells initialized to 0
    let mut grid_2 = empty_grid(); // Create a second grid for the next generation

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
            grid_1 = empty_grid(); // Clear the grid
        }
        let mouse_pos = mouse_position();
        let x = (mouse_pos.0 / CELL_SIZE) as usize;
        let y = (mouse_pos.1 / CELL_SIZE) as usize;
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            let current_state = get(&grid_1, x, y);
            set(
                &mut grid_1,
                x,
                y,
                if is_mouse_button_down(MouseButton::Left) {
                    true
                } else if is_mouse_button_down(MouseButton::Right) {
                    false
                } else {
                    current_state // Keep the current state if no mouse button is pressed
                },
            );
        }
        next_frame().await;
    }
}

fn randomise_grid(grid: &mut Grid) {
    for row in grid.iter_mut() {
        *row = ((rand::rand() as u64) << 32) | rand::rand() as u64;
    }
}

fn simulate(current_grid: &Grid, next_grid: &mut Grid) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cell = get(current_grid, x, y);
            let alive_neighbors = count_alive_neighbors(&current_grid, x, y);
            set(
                next_grid,
                x,
                y,
                matches!((cell, alive_neighbors), (true, 2 | 3) | (false, 3)),
            );
        }
    }
}

fn count_alive_neighbors(grid: &Grid, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the cell itself
            }
            let neighbor_x = x as isize + dx;
            let neighbor_y = y as isize + dy;
            if neighbor_x >= 0
                && neighbor_x < GRID_WIDTH as isize
                && neighbor_y >= 0
                && neighbor_y < GRID_HEIGHT as isize
            {
                if get(grid, neighbor_x as usize, neighbor_y as usize) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn render(grid: &Grid) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let color = if get(grid, x, y) { WHITE } else { BLACK };
            draw_rectangle(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }
}
