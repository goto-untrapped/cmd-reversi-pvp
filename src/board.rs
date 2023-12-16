
pub const BOARD_SIZE: usize = 6;

pub const BLACK_STONE: &str = "x";
pub const WHITE_STONE: &str = "o";
pub const NO_STONE: &str = "-";

pub struct Board<'a> {
    pub board: [[&'a str;BOARD_SIZE];BOARD_SIZE],
}

impl<'a> Board<'a> {
    pub fn created() -> Board<'a> {
        let mut board = Board { 
            board: [[NO_STONE;BOARD_SIZE];BOARD_SIZE],
        };
        board.init_pos();

        board
    }

    pub fn add_black_pos(&mut self, x_added: &usize, y_added: &usize) {
        self.board[*x_added][*y_added] = BLACK_STONE;
    }

    pub fn add_white_pos(&mut self, x_added: &usize, y_added: &usize) {
        self.board[*x_added][*y_added] = WHITE_STONE;
    }

    pub fn turn_over_white_stones(&mut self, x_added: usize, y_added: usize) {
        /* 
        inspect 8 directions.

        in one direction: 
        if I'am black, I search white and put it to vec
        if I find black, I will put vec to another vec to turn over.
        if I cannot find black, I do nothing.
        */

        // define new vec to record all stones pos to turn over
        let mut pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();

        // ↑
        let mut to_top_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        for x in (0..=x_added-1).rev() {
            // if found white, record to turn over
            if self.board[x][y_added] == WHITE_STONE {
                to_top_pos_vec_to_turn_over.push((x, y_added));
                continue;
            }
            // if found black, break
            if self.board[x][y_added] == BLACK_STONE {
                break;
            }
            // if found space, empty tmp vec and break
            if self.board[x][y_added] == NO_STONE {
                to_top_pos_vec_to_turn_over.clear();
                break;
            }
        }
        pos_vec_to_turn_over.append(&mut to_top_pos_vec_to_turn_over);

        // ↓

        // ←
        let mut to_left_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        for y in (0..=y_added-1).rev() {
            // if found white, record to turn over
            if self.board[x_added][y] == WHITE_STONE {
                to_left_pos_vec_to_turn_over.push((x_added, y));
                continue;
            }
            // if found black, break
            if self.board[x_added][y] == BLACK_STONE {
                break;
            }
            // if found space, empty tmp vec and break
            if self.board[x_added][y_added] == NO_STONE {
                to_left_pos_vec_to_turn_over.clear();
                break;
            }
        }
        pos_vec_to_turn_over.append(&mut to_left_pos_vec_to_turn_over);

        // →

        // ↖
        let mut to_top_left_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        for offset in 1..=x_added {
            let x_offset = x_added - offset;
            let y_offset = y_added - offset;
            // if found white, record to turn over
            if self.board[x_offset][y_offset] == WHITE_STONE {
                to_top_left_pos_vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found black, break
            if self.board[x_offset][y_offset] == BLACK_STONE {
                break;
            }
            // if found space, empty tmp vec and break
            if self.board[x_offset][y_offset] == NO_STONE {
                to_top_left_pos_vec_to_turn_over.clear();
                break;
            }
        }
        pos_vec_to_turn_over.append(&mut to_top_left_pos_vec_to_turn_over);

        // ↙

        // ↗

        // ↘
        
        // call another method to update stone colors
        Self::update_stones_color(self, &pos_vec_to_turn_over);
    }

    fn added_stones_pos_to_turn_over(&mut self, x: &usize, y: &usize) {

    }

    fn init_pos(&mut self) {
        // top left of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2 - 1] = WHITE_STONE;
        // top right of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2] = BLACK_STONE;
        // bottom left of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2 - 1] = BLACK_STONE;
        // bottom right of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2] = WHITE_STONE;
    }

    fn update_stones_color(&mut self, pos_vec_to_turn_over: &Vec<(usize, usize)>) {
        // when I'm BLACK
        for (x, y) in pos_vec_to_turn_over.into_iter() {
            self.board[*x][*y] = BLACK_STONE;
        }
    }
}