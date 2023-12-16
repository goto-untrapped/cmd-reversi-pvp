
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

    pub fn turn_over_stones(&mut self) {
        /* 
        inspect 8 directions.

        in one direction: 
        if I'am black, I search white and put it to vec
        if I find black, I will put vec to another vec to turn over.
        if I cannot find black, I do nothing.
        */

        // define new vec to record all stones pos to turn over
        let mut pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        // direction: to top
        // loop self board by decrease y pos to 0
        let mut one_line_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        // for (x, line) in self.board.iter().enumerate() {
        for x in 1..=BOARD_SIZE {
            let mut one_column_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
            // TODO: can not enter loop
            for y in x-1..=0 {
            // for (y, _column) in line.iter().enumerate() {
                // if found white, get position and put to vec
                if self.board[x][y] == WHITE_STONE {
                    one_column_pos_vec_to_turn_over.push((x, y));
                }
                // if found black, break
                if self.board[x][y] == BLACK_STONE {
                    break;
                }
                // if found space, empty tmp vec and break
                if self.board[x][y] == NO_STONE {
                    one_column_pos_vec_to_turn_over.clear();
                    break;
                }
            }
            // put one_column_pos_vec_to_turn_over to one_line_pos_vec_to_turn_over
            one_line_pos_vec_to_turn_over.append(&mut one_column_pos_vec_to_turn_over);
        }
        //   define new tmp vec to record stones pos to turn over 
        // if tmp vec is not empty, put tmp vec to record vec
        pos_vec_to_turn_over.append(&mut one_line_pos_vec_to_turn_over);


        // call another method to update stone colors
        Self::update_stones_color(self, &pos_vec_to_turn_over);
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

    fn added_stones_pos_to_turn_over(&mut self, x: &usize, y: &usize) {

    }

    fn update_stones_color(&mut self, pos_vec_to_turn_over: &Vec<(usize, usize)>) {
        // when I'm BLACK
        for (x, y) in pos_vec_to_turn_over.into_iter() {
            self.board[*x][*y] = BLACK_STONE;
        }
    }
}