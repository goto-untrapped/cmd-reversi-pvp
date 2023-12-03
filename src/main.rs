use std::io;

use board::{Board, BOARD_SIZE};

mod board;

fn main() {
    let mut board: Board = board::Board::create();
    update_screen(&board);

    loop {
        let (x, y) = get_input_pos();

        // limit pos range
        if BOARD_SIZE <= x || BOARD_SIZE <= y {
            println!("Invalid input! Please input again.");
            continue;
        }

        // update board by input
        board.board[x][y] = "x";

        update_screen(&board);
    }
}

fn get_input_pos() -> (usize, usize) {
    let mut input_xy = String::new();
    io::stdin()
    .read_line(&mut input_xy)
    .expect("failed to read from stdin");
    let mut input_xy_iter = input_xy.split_whitespace();

    let x:usize = input_xy_iter.next().unwrap().parse::<usize>().unwrap();
    let y:usize = input_xy_iter.next().unwrap().parse::<usize>().unwrap();
    
    (x, y)
}

fn update_screen(board: &Board) {
    print!("{}[2J", 27 as char);
    for line in board.board {
        for mark in line {
            print!("{} ", mark);
        }
        println!();
    }
}