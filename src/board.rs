
pub const BOARD_SIZE: usize = 6;

// pub const StoneType::BlackStone.as_str(): &str = "x";
// pub const StoneType::WhiteStone.as_str(): &str = "o";
// pub const NO_STONE: &str = "-";

pub enum StoneType {
    BlackStone,
    WhiteStone,
    NoStone,
}

impl StoneType {
    fn as_str(&self) -> &'static str {
        match self {
            StoneType::BlackStone => "x",
            StoneType::WhiteStone => "o",
            StoneType::NoStone => "-",
        }
    }
}

pub struct Board<'a> {
    pub board: [[&'a str;BOARD_SIZE];BOARD_SIZE],
}

impl<'a> Board<'a> {
    pub fn created() -> Board<'a> {
        let mut board = Board { 
            board: [[StoneType::NoStone.as_str();BOARD_SIZE];BOARD_SIZE],
        };
        board.init_pos();

        board
    }

    pub fn add_black_pos(&mut self, x_added: &usize, y_added: &usize) {
        self.board[*x_added][*y_added] = StoneType::BlackStone.as_str();
    }

    pub fn add_white_pos(&mut self, x_added: &usize, y_added: &usize) {
        self.board[*x_added][*y_added] = StoneType::WhiteStone.as_str();
    }

    pub fn turn_over_stones(&mut self, x_added: usize, y_added: usize, stone_type: StoneType) {
        // define new vec to record all stones pos to turn over
        let mut pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();

        // ↑
        Self::append_to_top_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &stone_type);

        // ↓

        // ←
        let mut to_left_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        for y in (0..=y_added-1).rev() {
            // if found white, record to turn over
            if self.board[x_added][y] == StoneType::WhiteStone.as_str() {
                to_left_pos_vec_to_turn_over.push((x_added, y));
                continue;
            }
            // if found black, break
            if self.board[x_added][y] == StoneType::BlackStone.as_str() {
                break;
            }
            // if found space, empty tmp vec and break
            if self.board[x_added][y_added] == StoneType::NoStone.as_str() {
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
            if self.board[x_offset][y_offset] == StoneType::WhiteStone.as_str() {
                to_top_left_pos_vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found black, break
            if self.board[x_offset][y_offset] == StoneType::BlackStone.as_str() {
                break;
            }
            // if found space, empty tmp vec and break
            if self.board[x_offset][y_offset] == StoneType::NoStone.as_str() {
                to_top_left_pos_vec_to_turn_over.clear();
                break;
            }
        }
        pos_vec_to_turn_over.append(&mut to_top_left_pos_vec_to_turn_over);

        // ↙

        // ↗

        // ↘
        
        // call another method to update stone colors
        Self::update_stones_color(self, &pos_vec_to_turn_over, &stone_type);
    }



    fn append_to_top_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, stone_type: &StoneType) {
        let mut to_top_pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        match stone_type {
            StoneType::BlackStone => {
                for x in (0..=x_added-1).rev() {
                    // if found to turn over stone, record to turn over
                    if self.board[x][y_added] == StoneType::WhiteStone.as_str() {
                        to_top_pos_vec_to_turn_over.push((x, y_added));
                        continue;
                    }
                    // if found my stone, break
                    if self.board[x][y_added] == StoneType::BlackStone.as_str() {
                        break;
                    }
                    // if found space, empty to added vec and break
                    if self.board[x][y_added] == StoneType::NoStone.as_str() {
                        to_top_pos_vec_to_turn_over.clear();
                        break;
                    }
                }
            },
            StoneType::WhiteStone => {
                for x in (0..=x_added-1).rev() {
                    // if found to turn over stone, record to turn over
                    if self.board[x][y_added] == StoneType::BlackStone.as_str() {
                        to_top_pos_vec_to_turn_over.push((x, y_added));
                        continue;
                    }
                    // if found my stone, break
                    if self.board[x][y_added] == StoneType::WhiteStone.as_str() {
                        break;
                    }
                    // if found space, empty to added vec and break
                    if self.board[x][y_added] == StoneType::NoStone.as_str() {
                        to_top_pos_vec_to_turn_over.clear();
                        break;
                    }
                }
            },
            StoneType::NoStone => {},
        }

        pos_vec_to_turn_over.append(&mut to_top_pos_vec_to_turn_over);
    }

    
    fn init_pos(&mut self) {
        // top left of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2 - 1] = StoneType::WhiteStone.as_str();
        // top right of center
        self.board[BOARD_SIZE / 2 - 1][BOARD_SIZE / 2] = StoneType::BlackStone.as_str();
        // bottom left of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2 - 1] = StoneType::BlackStone.as_str();
        // bottom right of center
        self.board[BOARD_SIZE / 2][BOARD_SIZE / 2] = StoneType::WhiteStone.as_str();
    }

    fn update_stones_color(&mut self, pos_vec_to_turn_over: &Vec<(usize, usize)>, stone_type: &StoneType) {
        
        match stone_type {
            StoneType::BlackStone => {
                for (x, y) in pos_vec_to_turn_over.into_iter() {
                    self.board[*x][*y] = StoneType::BlackStone.as_str();
                }
            }
            StoneType::WhiteStone => {
                for (x, y) in pos_vec_to_turn_over.into_iter() {
                    self.board[*x][*y] = StoneType::WhiteStone.as_str();
                }
            },
            StoneType::NoStone => {},
        }
    }
}