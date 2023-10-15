use crate::piece::{PieceState, PieceStateImpl};

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub struct Board {
    pub data: [[u8;BOARD_WIDTH];BOARD_HEIGHT],
    width: usize,
    height: usize,
}
impl Board {
    pub fn new() -> Board {
        Board {
            data: [[0 as u8; BOARD_WIDTH]; BOARD_HEIGHT],
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn check_collision(&self, piece: PieceState, y: i32, x: i32) -> bool {
        fn check_row(board: &Board, row: &[u8], y:i32 , x:i32 , py: usize) -> bool {
            row.iter()
               .enumerate()
               .filter(|(_,&cell)| cell > 0 ) // only non-zero cells can collide
               .any(|(px,_)| !board.at(y+py as i32, x+px as i32)
                                   .is_some_and(|cell| cell == 0))
        }
        piece.inner()
             .iter()
             .enumerate()
             .any(|(py, row)| check_row(self, row, y, x, py))
    }

    pub fn at(&self, y: i32, x: i32) -> Option<u8> {
        match 0 <= y && y < self.height as i32 && 0 <= x && x < self.width as i32 {
            true  => Some(self.data[y as usize][x as usize]),
            false => None,
        }
    }

    pub fn clear_rows(&mut self) -> i32{
        let mut n_cleared = 0; 
        for y in (1..self.data.len()).rev() {
            while self.data[y].iter().all(|cv| *cv > 0){
                self.shift_down(y);
                n_cleared += 1;
            }
        }
        n_cleared
    }
    
    fn shift_down(&mut self, into_row: usize) {
        for y in (1..=into_row).rev() {
            for x in 0..self.data[0].len() {
                self.data[y][x] = self.data[y-1][x];
            }
        }
        self.data[0].iter_mut().for_each(|cell| *cell = 0);
    }

    pub fn place_piece(&mut self, piece: PieceState, y: usize, x: usize) {
        for (dy, row) in piece.inner().iter().enumerate() {
            for (dx, &val) in row.iter().enumerate()
                                        .filter(|(_,val)| **val > 0) {
                self.data[y+dy][x+dx] = val;
            } 
        }
    }
}