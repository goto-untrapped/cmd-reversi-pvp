use std::io;

use board::{Board, BOARD_SIZE};

mod board;

fn main() {
    let mut board: Board = board::Board::create();
    update_screen(&board);

    loop {
        // place black stone
        let (x, y) = get_input_pos();
        if is_input_invalid(&x, &y) {
            continue;
        }
        board.add_black_pos(&x, &y);
        board.update_stones_color();
        update_screen(&board);

        // place white stone
        let (x, y) = get_input_pos();
        if is_input_invalid(&x, &y) {
            continue;
        }
        board.add_white_pos(&x, &y);
        board.update_stones_color();
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

fn is_input_invalid(x: &usize, y: &usize) -> bool {
    // invalid: input position is outside of board
    if BOARD_SIZE <= *x || BOARD_SIZE <= *y {
        println!("Invalid input! Please input again.");
        return true;
    }

    false
}

fn update_screen(board: &Board) {
    // clear the screen
    print!("{}[2J", 27 as char);

    // decorate screen: show x position
    print!("  ");
    for x_index in 0..BOARD_SIZE {
        print!("{} ", x_index);
    }
    println!();

    // decorate screen: show y position and stones position
    let mut y_index = 0;
    for line in board.board {
        print!("{} ", y_index);
        y_index +=1;

        for mark in line {
            print!("{} ", mark);
        }
        println!();
    }
}