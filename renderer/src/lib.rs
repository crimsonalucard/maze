pub mod renderer {
    use nannou::prelude::*;
    use maze::maze::{Coord, Maze};

    struct Model {
        window: window::Id,
        maze: Option<Maze>,
    }

    fn model(app: &App) -> Model {
        let window = app.new_window().view(view).build().unwrap();
        let width = 100;
        let height = 100;
        let start: Coord = (0, 0);
        let maze = Maze::new(width, height, start);
        Model { window, maze: Some(maze) }
    }

    fn update(_app: &App, model: &mut Model, _update: Update) {}

    fn view(app: &App, model: &Model, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);
        // draw.ellipse().x_y(-512.0, 384.0).color(STEELBLUE);
        if let Some(maze) = &model.maze {
            cost_drawer(maze, &draw, app.window(model.window).unwrap());
            maze_drawer(maze, &draw, app.window(model.window).unwrap(), 1.0);
            draw.to_frame(app, &frame).unwrap();
        }
    }

    fn draw_grid_point(col: usize,
                       row: usize,
                       color: Srgb<u8>,
                       draw: &Draw,
                       cell_width: f32,
                       cell_height: f32,
                       width_offset: f32,
                       height_offset: f32) {
        draw.rect().width(cell_width).height(cell_height).color(color).x_y(
            (col as f32 * cell_width) - width_offset + (cell_width / 2.0),
            -(row as f32 * cell_height) + height_offset - (cell_height / 2.0));
    }

    fn maze_drawer(maze: &Maze, draw: &Draw, window: std::cell::Ref<Window>, thickness: f32) {
        let (width, height) = window.inner_size_pixels();
        let (cell_width, cell_height) = (width as f32 / maze.vertical_walls.len() as f32, height as f32 / maze.horizontal_walls.len() as f32);
        let (width_offset, height_offset) = (width as f32 / 2.0, height as f32 / 2.0);
        let (row_start, col_start) = maze.start.clone();
        let (row_end, col_end) = maze.end.clone();
        draw_grid_point(col_start, row_start, GREEN, draw, cell_width, cell_height, width_offset, height_offset);
        draw_grid_point(col_end, row_end, RED, draw, cell_width, cell_height, width_offset, height_offset);


        maze.horizontal_walls.iter().enumerate().for_each(|(row_index, walls)| {
            walls.iter()
                .enumerate()
                .filter(|(_, &wall)| wall)
                .map(|(col_index, _)| col_index)
                .for_each(|col_index| {
                    // print!("<{},{}>", row_index, col_index);

                    let start_point = pt2(((col_index as f32 * cell_width) - width_offset) as f32,
                                          (height_offset - (row_index as f32 * cell_height) + thickness) as f32);
                    let end_point = pt2(((col_index as f32 * cell_width) - width_offset) as f32,
                                        (height_offset - (row_index as f32 * cell_height) - cell_height - thickness) as f32);

                    // println!("{:?} -> {:?} ({},{}); ", start_point, end_point, row_index, col_index);
                    draw.line()
                        .start(start_point)
                        .end(end_point)
                        .weight(thickness * 2.0)
                        .color(YELLOW);
                });
        });
        maze.vertical_walls.iter().enumerate().for_each(|(row_index, walls)| {
            walls.iter()
                .enumerate()
                .filter(|(_, &wall)| wall)
                .map(|(col_index, _)| col_index)
                .for_each(|col_index| {
                    // print!("<{},{}>", row_index, col_index);
                    let start_point = pt2(((row_index as f32 * cell_width) - width_offset - thickness) as f32,
                                          (height_offset - (col_index as f32 * cell_height)) as f32);
                    let end_point = pt2(((row_index as f32 * cell_width) - width_offset + cell_width + thickness) as f32,
                                        (height_offset - (col_index as f32 * cell_height)) as f32);
                    // println!("{:?} -> {:?} ({},{}); ", start_point, end_point, row_index, col_index);
                    draw.line()
                        .start(start_point)
                        .end(end_point)
                        .weight(thickness * 2.0)
                        .color(YELLOWGREEN);
                });
        });
    }

    fn scale_to_u8(value: usize, max_value: usize) -> u8 {
        ((value as f32 / max_value as f32) * 255.0) as u8
    }

    fn cost_drawer(maze: &Maze, draw: &Draw, window: std::cell::Ref<Window>) {
        let (width, height) = window.inner_size_pixels();
        let (cell_width, cell_height) = (width as f32 / maze.vertical_walls.len() as f32, height as f32 / maze.horizontal_walls.len() as f32);
        let (width_offset, height_offset) = (width as f32 / 2.0, height as f32 / 2.0);
        // println!("{:?}", maze.cost);
        let max_value = match maze.cost.iter().map(|x| {
            x.iter().filter(|x| x.is_some()).map(|x| x.unwrap())
        }).flatten().max() {
            None => 0,
            Some(value) => value
        };

        let height = maze.cost.len();
        let width = maze.cost[0].len();
        (0..height).map(|row| {
            (0..width).map(|col| {
                (row.clone(), col.clone())
            }).collect::<Vec<(usize, usize)>>()
        }).collect::<Vec<Vec<(usize, usize)>>>().iter().flatten().for_each(|(row, col)| {
            if let Some(cost) = maze.cost[*row][*col] {
                let grad = scale_to_u8(cost, max_value);
                draw_grid_point(*col, *row, srgb(grad, grad, grad), draw, cell_width, cell_height, width_offset, height_offset);
            }
        });
    }

    pub fn render() {
        // let maze = Maze::new(30, 30);
        nannou::app(model).update(update).run();
    }
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
