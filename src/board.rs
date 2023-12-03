
pub const BOARD_SIZE: usize = 6;

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

    pub fn update_pos(&mut self, x_added: &usize, y_added: &usize) {
        self.board[*x_added][*y_added] = "x";
    }

    fn init_pos(&mut self) {
        // top left of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2 - 1] = "x";
        // top right of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2] = "o";
        // bottom left of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2 - 1] = "o";
        // bottom right of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2] = "x";
    }
}