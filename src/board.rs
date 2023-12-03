
pub const BOARD_SIZE: usize = 6;
pub const BLACK_STONE: &str = "x";
pub const WHITE_STONE: &str = "○";

pub struct Board<'a> {
    pub board: [[&'a str;BOARD_SIZE];BOARD_SIZE],
}

impl<'a> Board<'a> {
    pub fn create() -> Board<'a> {
        let mut board = Board { 
            board: [["-";BOARD_SIZE];BOARD_SIZE],
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

    pub fn update_stones_color(&mut self) {

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
}