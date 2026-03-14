use nannou::prelude::*;

const GRID: usize = 80;
const CELL: f32 = 10.0;

struct Model {
    grid: Vec<Vec<bool>>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size((GRID as f32 * CELL) as u32, (GRID as f32 * CELL) as u32)
        .view(view)
        .build()
        .unwrap();

    let mut grid = vec![vec![false; GRID]; GRID];

    // 随机初始化
    for x in 0..GRID {
        for y in 0..GRID {
            grid[x][y] = random::<f32>() > 0.7;
        }
    }

    Model { grid }
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: isize, y: isize) -> i32 {
    let mut count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {

            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < GRID as isize &&
               ny >= 0 && ny < GRID as isize &&
               grid[nx as usize][ny as usize]
            {
                count += 1;
            }
        }
    }

    count
}

fn step(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {

    let mut next = vec![vec![false; GRID]; GRID];

    for x in 0..GRID {
        for y in 0..GRID {

            let n = count_neighbors(grid, x as isize, y as isize);

            next[x][y] = match (grid[x][y], n) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false
            };
        }
    }

    next
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid = step(&model.grid);
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    draw.background().color(BLACK);

    let win = app.window_rect();

    for x in 0..GRID {
        for y in 0..GRID {

            if model.grid[x][y] {

                let px = win.left() + x as f32 * CELL;
                let py = win.bottom() + y as f32 * CELL;

                draw.rect()
                    .x_y(px, py)
                    .w_h(CELL, CELL)
                    .color(WHITE);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
