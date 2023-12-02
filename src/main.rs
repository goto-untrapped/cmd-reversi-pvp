use std::io;

fn main() {
    let mut board: [[&str;4];4] = [["-";4];4];
    board[1][1] = "x";
    board[1][2] = "o";
    board[2][1] = "o";
    board[2][2] = "x";
    for line in board.as_mut() {
        for mark in line.as_mut() {
            print!("{} ", mark);
        }
        println!();
    }

    loop {
        let (x, y) = get_input_pos();

        // limit pos range
        if 4 <= x || 4 <= y {
            println!("Invalid input! Please input again.");
            continue;
        }

        // update board by input
        board[x][y] = "x";
        for line in board.as_mut() {
            for mark in line.as_mut() {
                print!("{} ", mark);
            }
            println!();
        }
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
