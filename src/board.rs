
pub const BOARD_SIZE: usize = 6;

#[derive(PartialEq, Eq)]
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

    pub fn turn_over_stones(&mut self, x_added: usize, y_added: usize, my_stone_type: StoneType) {
        let to_turn_over_stone_type = Self::get_to_turn_over_stone_type(&my_stone_type);

        // define new vec to record all stones pos to turn over
        let mut pos_vec_to_turn_over: Vec<(usize, usize)> = Vec::new();

        // ↑
        Self::append_to_top_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ↓
        Self::append_to_bottom_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ←
        Self::append_to_left_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // →
        Self::append_to_right_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ↖
        Self::append_to_top_left_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ↙
        Self::append_to_bottom_left_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ↗
        Self::append_to_top_right_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);
        // ↘
        Self::append_to_bottom_right_pos_to_turn_over(self, &mut pos_vec_to_turn_over, x_added, y_added, &to_turn_over_stone_type, &my_stone_type);

        Self::update_stones_color(self, &pos_vec_to_turn_over, &to_turn_over_stone_type);
    }

    fn get_to_turn_over_stone_type(my_stone_type: &StoneType) -> StoneType {
        if my_stone_type == &StoneType::BlackStone {
            return StoneType::WhiteStone
        }
        StoneType::BlackStone
    }

    fn append_to_top_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        if x_added == 0 {
            return;
        }
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let mut is_end_stone_not_exist = true;

        for x in (0..=x_added-1).rev() {
            // if found to turn over stone, record
            if self.board[x][y_added] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x, y_added));
                continue;
            }
            // if found my stone, break
            if self.board[x][y_added] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x][y_added] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_bottom_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let mut is_end_stone_not_exist = true;

        for x in x_added+1..=BOARD_SIZE-1 {
            // if found to turn over stone, record
            if self.board[x][y_added] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x, y_added));
                continue;
            }
            // if found my stone, break
            if self.board[x][y_added] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x][y_added] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_left_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        if y_added == 0 {
            return;
        }
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let mut is_end_stone_not_exist = true;

        for y in (0..=y_added-1).rev() {
            // if found to turn over stone, record
            if self.board[x_added][y] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_added, y));
                continue;
            }
            // if found my stone, break
            if self.board[x_added][y] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_added][y] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_right_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let mut is_end_stone_not_exist = true;

        for y in y_added+1..BOARD_SIZE-1 {
            // if found to turn over stone, record
            if self.board[x_added][y] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_added, y));
                continue;
            }
            // if found my stone, break
            if self.board[x_added][y] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_added][y] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_top_left_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let min_offset = Self::get_min_offset(&x_added, &y_added);
        let mut is_end_stone_not_exist = true;
        
        for offset in 1..=min_offset {
            let x_offset = x_added - offset;
            let y_offset = y_added - offset;
            // if found to turn over stone, record
            if self.board[x_offset][y_offset] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found my stone, break
            if self.board[x_offset][y_offset] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_offset][y_offset] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_bottom_left_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        if y_added == 0 {
            return
        }
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let max_offset = Self::get_max_offset(&x_added, &y_added);
        let mut is_end_stone_not_exist = true;

        for offset in 1..=BOARD_SIZE-max_offset-1 {
            let x_offset = x_added + offset;
            let y_offset = y_added - offset;
            // if found to turn over stone, record
            if self.board[x_offset][y_offset] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found my stone, break
            if self.board[x_offset][y_offset] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_offset][y_offset] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_top_right_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let min_offset = Self::get_min_offset(&x_added, &y_added);
        let mut is_end_stone_not_exist = true;

        for offset in 1..=min_offset {
            let x_offset = x_added - offset;
            let y_offset = y_added + offset;
            // if found to turn over stone, record
            if self.board[x_offset][y_offset] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found my stone, break
            if self.board[x_offset][y_offset] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_offset][y_offset] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn append_to_bottom_right_pos_to_turn_over(&mut self, pos_vec_to_turn_over: &mut Vec<(usize, usize)>, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) {
        let mut vec_to_turn_over: Vec<(usize, usize)> = Vec::new();
        let max_offset = Self::get_max_offset(&x_added, &y_added);
        let mut is_end_stone_not_exist = true;

        for offset in 1..=BOARD_SIZE-max_offset-1 {
            let x_offset = x_added + offset;
            let y_offset = y_added + offset;
            // if found to turn over stone, record
            if self.board[x_offset][y_offset] == to_turn_over_stone_type.as_str() {
                vec_to_turn_over.push((x_offset, y_offset));
                continue;
            }
            // if found my stone, break
            if self.board[x_offset][y_offset] == my_stone_type.as_str() {
                is_end_stone_not_exist = false;
                break;
            }
            // if found space, empty to added vec and break
            if self.board[x_offset][y_offset] == StoneType::NoStone.as_str() {
                vec_to_turn_over.clear();
                break;
            }
        }
        // if to turn over vec don't has end my stone, don't turn over stones
        if is_end_stone_not_exist {
            vec_to_turn_over.clear();
        }

        pos_vec_to_turn_over.append(&mut vec_to_turn_over);
    }

    fn get_min_offset(x_added: &usize, y_added: &usize) -> usize {
        if x_added < y_added {
            return *x_added
        }
        *y_added
    }

    fn get_max_offset(x_added: &usize, y_added: &usize) -> usize {
        if y_added < x_added {
            return *x_added
        }
        *y_added
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
                    self.board[*x][*y] = StoneType::WhiteStone.as_str();
                }
            }
            StoneType::WhiteStone => {
                for (x, y) in pos_vec_to_turn_over.into_iter() {
                    self.board[*x][*y] = StoneType::BlackStone.as_str();
                }
            },
            StoneType::NoStone => {},
        }
    }
}