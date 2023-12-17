use std::{io, process::exit};

use board::{Board, BOARD_SIZE, StoneType};

mod board;

fn main() {
    let mut board: Board = board::Board::created();
    let mut put_stone_count = 1;

    loop {
        let my_stone_type = got_my_turn_stone_color(&mut put_stone_count);
        board.updated_candidate_stone_pos(&my_stone_type);
        if board.is_no_stone_can_turn_over() {
            put_stone_count += 1;
            continue;
        }
        update_screen(&board.candidate_board);
        // put black stone
        let pos_number: usize;
        match got_input_pos() {
            Some(input_pos_number) => {
                pos_number = input_pos_number
            },
            None => {
                print_input_error_message();
                continue;
            },
        };

        if board.is_pos_number_valid( &pos_number) {
            print_input_error_message();
            continue;
        }
        // if input stone position can't turn over any stones, need again
        if !&board.add_input_stone_pos(&pos_number, &my_stone_type) {
            println!("can't turn over any stones, choose another position");
            continue;
        }
        put_stone_count += 1;

        // game set
        if board.is_no_pos_to_put_stones() {
            let (count_black_stones, count_white_stones) = board.game_result();
            print_game_result(count_black_stones, count_white_stones);
            exit(0);
        }
    }
}

fn got_input_pos() -> Option<usize> {
    let mut input_xy = String::new();
    io::stdin()
        .read_line(&mut input_xy)
        .expect("failed to read from stdin");
    let mut input_xy_iter = input_xy.split_whitespace();

    let pos_number_str = input_xy_iter.next()?;

    let pos_number: usize = pos_number_str.parse::<usize>().ok()?;

    Some(pos_number)
}

// black: odd count, white: even count
fn got_my_turn_stone_color(count: &mut i32) -> StoneType {
    if *count % 2 == 1 { return StoneType::BlackStone }
    StoneType::WhiteStone
}

fn print_input_error_message() {
    println!("Invalid input! Please input again.");
}

fn update_screen(board: &[[& str;BOARD_SIZE];BOARD_SIZE]) {
    println!();

    // decorate screen: show x position
    for _ in 0..BOARD_SIZE-1 {
        print!("* * ");
    }
    println!();

    let mut count: String = "1".to_owned();
    // decorate screen: show y position and stones position
    for line in board {
        print!("* ");

        for mark in line {
            if mark == &"*" {
                print!("{0:<3}", count);
                let mut count_i32: i32 = count.parse().unwrap();
                count_i32 += 1;
                count = count_i32.to_string();
            } else {
                print!("{}  ", mark);
            }
        }
        println!("* ");
    }

    for _ in 0..BOARD_SIZE-1 {
        print!("* * ");
    }
    println!();
}

fn print_game_result(count_black_stones: i32, count_white_stones: i32) {
    let winner: &str = {
        if count_white_stones < count_black_stones { "BLACK" }
        else if count_black_stones < count_white_stones { "WHITE" }
        else { "DRAW" }
    };

    println!("Winner is {}", winner);
    println!("black stones: {}", count_black_stones);
    println!("white stones: {}", count_white_stones);
}