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

    pub fn at_yx(&self, y: i32, x: i32) -> Option<u8> {
        match 0 <= y && y < self.height as i32 && 0 <= x && x < self.width as i32 {
            true  => Some(self.data[y as usize][x as usize]),
            false => None,
        }
    }

    pub fn check_collision(&self, piece: &[&[u8]], y: i32, x: i32) -> bool {
        // This is confusing and stupid. technically its "purely functional" 
        !piece.iter()
              .enumerate()
              .all(|(dy, row)| row.iter() 
                                  .enumerate()
                                  .filter(|(_, &cell)| cell > 0)
                                  .all(|(dx, _)| self.at_yx(y+dy as i32, x+dx as i32)
                                                     .is_some_and(|val| val == 0)))
    }
    pub fn clear_rows(&mut self){
        for y in (1..self.data.len()).rev() {
            while self.data[y].iter().all(|cv| *cv > 0){
                self.shift_down(y);
            }
        }
    }
    
    fn shift_down(&mut self, into_row: usize) {
        for y in (1..=into_row).rev() {
            for x in 0..self.data[0].len() {
                self.data[y][x] = self.data[y-1][x];
            }
        }
        self.data[0].iter_mut().for_each(|cell| *cell = 0);
    }

    pub fn place_piece(&mut self, piece: &[&[u8]], y: usize, x: usize) {
        for (dy, row) in piece.iter().enumerate() {
            for (dx, &val) in row.iter().enumerate()
                                        .filter(|(_,val)| **val > 0) {
                self.data[y+dy][x+dx] = val;
            } 
        }
    }
}