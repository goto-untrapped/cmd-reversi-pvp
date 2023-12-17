use std::collections::HashSet;

pub const BOARD_SIZE: usize = 8;

#[derive(PartialEq, Eq)]
pub enum StoneType {
    BlackStone,
    WhiteStone,
    NoStone,
}

impl StoneType {
    fn as_str(&self) -> &'static str {
        match self {
            StoneType::BlackStone => "●",
            StoneType::WhiteStone => "○",
            StoneType::NoStone => "-",
        }
    }
}

pub struct Board<'a> {
    pub board: [[&'a str;BOARD_SIZE];BOARD_SIZE],
    pub candidate_board: [[&'a str;BOARD_SIZE];BOARD_SIZE],
    pub turn_over_stones_vec: Vec<(usize, usize)>,
}

impl<'a> Board<'a> {
    pub fn created() -> Board<'a> {
        let mut board = Board { 
            board: [[StoneType::NoStone.as_str();BOARD_SIZE];BOARD_SIZE],
            // for debug
            // board: [
            //     ["-", "-", "-", "o", "-", "x", "x", "-"],
            //     ["-", "-", "-", "-", "o", "x", "-", "-"],
            //     ["-", "-", "o", "x", "x", "o", "-", "-"],
            //     ["-", "-", "x", "x", "x", "-", "-", "-"],
            //     ["-", "-", "-", "x", "x", "x", "x", "-"],
            //     ["-", "-", "-", "-", "-", "-", "o", "-"],
            //     ["-", "-", "-", "-", "-", "-", "-", "-"],
            //     ["-", "-", "-", "-", "-", "-", "-", "-"],
            // ]
            candidate_board: [[StoneType::NoStone.as_str();BOARD_SIZE];BOARD_SIZE],
            turn_over_stones_vec: Vec::new(),
        };
        board.init_pos();

        board
    }

    pub fn updated_candidate_stone_pos(&mut self, my_stone_type: &StoneType) {
        // copy board to calculate candidate positions
        self.candidate_board = self.board.clone();
        // get all positions surrounded by can turon over stones
        self.got_candidate_pos_vec(&my_stone_type);
        for positions in &self.turn_over_stones_vec {
            self.candidate_board[positions.0][positions.1] = "*";
        }
    }

    pub fn is_no_stone_can_turn_over(&mut self) -> bool {
        if self.turn_over_stones_vec.len() == 0 {
            return true;
        }
        false
    }

    fn got_candidate_pos_vec(&mut self, my_stone_type: &StoneType) {
        let to_turn_over_stone_type = Self::get_to_turn_over_stone_type(&my_stone_type);
        let mut candidate_pos_vec: Vec<(usize, usize)> = Vec::new();
        // get positions that surrounded by put stones
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.board[x][y] != StoneType::NoStone.as_str() {
                    // ↑
                    if 0 < x {
                        if !self.is_pos_has_stone_already(&(x-1), &y) {
                            candidate_pos_vec.append(&mut vec![(x-1, y)]);
                        }
                    }
                    // ↓
                    if x + 1 < BOARD_SIZE {
                        if !self.is_pos_has_stone_already(&(x+1), &y) {
                            candidate_pos_vec.append(&mut vec![(x+1, y)]);
                        }
                    }
                    // ←
                    if 0 < y {
                        if !self.is_pos_has_stone_already(&x, &(y-1)) {
                            candidate_pos_vec.append(&mut vec![(x, y-1)]);
                        }
                    }
                    // →
                    if y + 1 < BOARD_SIZE {
                        if !self.is_pos_has_stone_already(&x, &(y+1)) {
                            candidate_pos_vec.append(&mut vec![(x, y+1)]);
                        }
                    }
                    // ↖
                    if 0 < x && 0 < y {
                        if !self.is_pos_has_stone_already(&(x-1), &(y-1)) {
                            candidate_pos_vec.append(&mut vec![(x-1, y-1)]);
                        }
                    }
                    // ↙
                    if x + 1 < BOARD_SIZE && 0 < y {
                        if !self.is_pos_has_stone_already(&(x+1), &(y-1)) {
                            candidate_pos_vec.append(&mut vec![(x+1, y-1)]);
                        }
                    }
                    // ↗
                    if 0 < x && y + 1 < BOARD_SIZE {
                        if !self.is_pos_has_stone_already(&(x-1), &(y+1)) {
                            candidate_pos_vec.append(&mut vec![(x-1, y+1)]);
                        }
                    }
                    // ↘
                    if x + 1 < BOARD_SIZE && y + 1 < BOARD_SIZE {
                        if !self.is_pos_has_stone_already(&(x+1), &(y+1)) {
                            candidate_pos_vec.append(&mut vec![(x+1, y+1)]);
                        }
                    }
                }
            }
        }

        // remove duplicate
        let candidate_pos_set: HashSet<(usize, usize)> = candidate_pos_vec.into_iter().collect();
        let mut candidate_pos_sorted_vec: Vec<&(usize, usize)> = candidate_pos_set.iter().collect();
        candidate_pos_sorted_vec.sort_by(
            |first, second| first.0.cmp(&second.0).then(first.1.cmp(&second.1))
        );
        
        // check each position and set to another vec if can turn over stones
        self.turn_over_stones_vec = Vec::new();
        for (x, y) in candidate_pos_sorted_vec {
            let current_turn_over_pos_vec = self.got_turn_over_stones_vec(*x, *y, &to_turn_over_stone_type, my_stone_type);
            if 1 <= current_turn_over_pos_vec.len() {
                self.turn_over_stones_vec.push((*x,*y));
            }
        }
    }

    pub fn is_pos_number_valid(&mut self, pos_number: &usize) -> bool {
        // screen index is start from 1
        if self.turn_over_stones_vec.len() < *pos_number {
            return true;
        }
        false
    }

    pub fn add_input_stone_pos(&mut self, pos_number: &usize, my_stone_type: &StoneType) -> bool {
        // screen index is start from 1
        let (x_added, y_added) = self.turn_over_stones_vec[pos_number-1];

        // add input stone pos to board
        self.board[x_added][y_added] = my_stone_type.as_str();
        // test updated board has count of turn over stones
        let to_turn_over_stone_type = Self::get_to_turn_over_stone_type(&my_stone_type);
        // if have not, clear input pos from board and return false
        let pos_vec_to_turn_over = self.got_turn_over_stones_vec(x_added, y_added, &to_turn_over_stone_type, my_stone_type);
        if pos_vec_to_turn_over.len() == 0 {
            self.board[x_added][y_added] = StoneType::NoStone.as_str();
            return false;
        }
        // if have, update board and return true
        Self::update_stones_color(self, &pos_vec_to_turn_over, &to_turn_over_stone_type);
        true
    }

    pub fn got_turn_over_stones_vec(&mut self, x_added: usize, y_added: usize, to_turn_over_stone_type: &StoneType, my_stone_type: &StoneType) -> Vec<(usize, usize)> {
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

        pos_vec_to_turn_over
    }

    pub fn is_no_pos_to_put_stones(&mut self) -> bool {
        // if board is already full of stones
        for line in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                if self.board[line][column] == StoneType::NoStone.as_str() {
                    return false;
                }
            }
        }

        true
    }

    pub fn game_result(&mut self) -> (i32, i32) {
        let mut count_black_stones: i32 = 0;
        let mut count_white_stones: i32 = 0;

        for line in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                if self.board[line][column] == StoneType::BlackStone.as_str() {
                    count_black_stones += 1;
                }
                if self.board[line][column] == StoneType::WhiteStone.as_str() {
                    count_white_stones += 1;
                }
            }
        }

        (count_black_stones, count_white_stones)
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

    fn get_to_turn_over_stone_type(my_stone_type: &StoneType) -> StoneType {
        if my_stone_type == &StoneType::BlackStone {
            return StoneType::WhiteStone
        }
        StoneType::BlackStone
    }

    fn is_pos_has_stone_already(&mut self, x: &usize, y: &usize) -> bool {
        if self.board[*x][*y] != StoneType::NoStone.as_str() {
            return true;
        }
        false
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
        let mut is_end_stone_not_exist = true;
        
        for offset in 1..=BOARD_SIZE {
            if x_added < offset || y_added < offset {
                break;
            }
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
        let mut is_end_stone_not_exist = true;

        for offset in 1..=BOARD_SIZE {
            if BOARD_SIZE <= x_added + offset || y_added < offset {
                break;
            }
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
        let mut is_end_stone_not_exist = true;

        for offset in 1..=BOARD_SIZE {
            if x_added < offset || BOARD_SIZE <= y_added + offset {
                break;
            }
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
        let mut is_end_stone_not_exist = true;

        for offset in 1..=BOARD_SIZE {
            if BOARD_SIZE <= x_added + offset  || BOARD_SIZE <= y_added + offset {
                break;
            }
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