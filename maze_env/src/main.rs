#![allow(warnings)]

use raylib::prelude::*;
use std::thread::sleep;
use std::time::Duration;

enum Actions {
    Left,
    Down,
    Right,
    Up
}

struct MazeMatrix {
    matrix: Vec<Vec<bool>>,
    current_pos: (usize, usize),
    cols: usize,
    rows: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl MazeMatrix {
    fn init(
        cols: usize,
        rows: usize,    
        start_pos: (usize, usize), 
        end_pos: (usize, usize),
        obstacles: Vec<(usize, usize)>
    ) -> MazeMatrix {
        let mut maze = MazeMatrix {
            matrix: vec![vec![true; cols]; rows],
            current_pos: start_pos,
            cols: cols,
            rows: rows,
            start: start_pos,
            end: end_pos,
        };

        for (row, col) in obstacles {
            maze.matrix[row][col] = false;
        }
        return maze;
    }
    
    fn _get_xy(&self, direction: Actions) -> (usize, usize) {
        let &(row, col) = &self.current_pos;
        let (next_row, next_col) = match direction {
            Actions::Left => (row, col.saturating_sub(1)),
            Actions::Down => (row.saturating_add(1), col),
            Actions::Right => (row, col.saturating_add(1)),
            Actions::Up => (row.saturating_sub(1), col),
        };
        return (
            next_row.clamp(0, self.rows - 1),
            next_col.clamp(0, self.cols - 1)
        );
    }

    fn move_agent(&mut self, direction: Actions) {
        let (next_row, next_col) = self._get_xy(direction);
        if self.matrix[next_row][next_col] {
            self.current_pos = (next_row, next_col);
        }
    }

    fn render_current(&self, d: &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let color = if self.matrix[row][col] {
                    Color::LIGHTGRAY
                } else {
                    Color::DARKGRAY
                };
                d.draw_rectangle(
                    (col * 100) as i32,
                    (row * 100) as i32,
                    100,
                    100,
                    color,
                );
                d.draw_rectangle_lines(
                    (col * 100) as i32,
                    (row * 100) as i32,
                    100,
                    100,
                    Color::BLACK,
                );
            }
        }
        let (agent_row, agent_col) = self.current_pos;
        d.draw_rectangle(
            (agent_col * 100) as i32,
            (agent_row * 100) as i32,
            100,
            100,
            Color::RED,
        );
    }

    fn render_step(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, action: Actions) {
        self.move_agent(action);
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::WHITE);
        self.render_current(&mut d);
        sleep(Duration::from_millis(500));
    }
}

struct GymnasiumWrapper {
    maze: MazeMatrix,
}

impl GymnasiumWrapper {
    fn new(
        cols: usize,
        rows: usize,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
        obstacles: Vec<(usize, usize)>
    ) -> GymnasiumWrapper {
        let maze = MazeMatrix::init(cols, rows, start_pos, end_pos, obstacles);
        return GymnasiumWrapper { maze }
    }

    fn reset(&mut self) {
        self.maze.current_pos = self.maze.start;
    }

    fn step(&mut self, action: Actions) {
        self.maze.move_agent(action);
    }

    fn render_step(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, action: Actions) {
        self.maze.render_step(rl, thread, action);
    }
}

fn main() {
    let mut env = GymnasiumWrapper::new(
        9,
        6,
        (2, 0),
        (8, 0),
        vec![(1, 2), (2, 2), (3, 2), (4, 5), (0, 7), (1, 7), (2, 7)]
    );

    let (mut rl, thread) = raylib::init()
        .size((env.maze.cols * 100) as i32, (env.maze.rows * 100) as i32)
        .title("Maze-v0 Environment")
        .vsync()
        .build();
    rl.set_target_fps(60);

    // Example actions from an agent
    let actions = vec![
        Actions::Down,
        Actions::Down,
        Actions::Down, // 3 downs
        Actions::Right,
        Actions::Right,
        Actions::Right,
        Actions::Right,
        Actions::Right,
        Actions::Right,
        Actions::Right,
        Actions::Right, // 8 rights
        Actions::Up,
        Actions::Up,
        Actions::Up,
        Actions::Up,
        Actions::Up, // 5 Ups
    ];

    for action in actions {
        if rl.window_should_close() {
            break;
        }
        env.render_step(&mut rl, &thread, action);
    }
}

// CHAT GPT ON HOW TO WRAP THIS TO USE IN PYTHON

// use pyo3::prelude::*;

// #[pyclass]
// struct MazeEnv {
//     wrapper: GymnasiumWrapper,
// }

// #[pymethods]
// impl MazeEnv {
//     #[new]
//     fn new() -> Self {
//         Self {
//             wrapper: GymnasiumWrapper::new(9, 6, (2, 0), (8, 0), vec![(1, 2), (2, 2), (3, 2), (4, 5), (0, 7), (1, 7), (2, 7)])
//         }
//     }

//     fn reset(&mut self) -> PyResult<(Vec<i32>, HashMap<String, Any>)> {
//         self.wrapper.reset();
//         // Return observation and info
//         Ok((vec![], HashMap::new()))
//     }

//     fn step(&mut self, action: i32) -> PyResult<(Vec<i32>, f64, bool, bool, HashMap<String, Any>)> {
//         // Convert action and return (obs, reward, terminated, truncated, info)
//         Ok((vec![], 0.0, false, false, HashMap::new()))
//     }
// }

// import gymnasium as gym
// from gymnasium import spaces
// from . import rust_maze

// class MazeEnv(gym.Env):
//     def __init__(self):
//         self.rust_env = rust_maze.MazeEnv()
//         self.action_space = spaces.Discrete(4)
//         self.observation_space = spaces.Box(low=0, high=1, shape=(6, 9))

//     def reset(self, seed=None):
//         return self.rust_env.reset()

//     def step(self, action):
//         return self.rust_env.step(action)