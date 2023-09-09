use std::time::Duration;

pub mod gfx;

pub const TICKS_PER_SECOND: i32 = 60;
pub const NCURSES_GETCH_TIMOUT: i32 = 1000/TICKS_PER_SECOND as i32;
pub const TICK_DUR_MILLIS: Duration = Duration::from_millis(NCURSES_GETCH_TIMOUT as u64);

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub const I_PIECE: &[&[&[u8]]] = &[ 
    &[&[1,1,1,1]],
    &[&[1],&[1],&[1],&[1]]];

pub const J_PIECE: &[&[&[u8]]] = &[
    &[&[2,0,0], &[2,2,2]],
    &[&[2,2]  , &[2,0]   , &[2,0]],
    &[&[2,2,2], &[0,0,2]],
    &[&[0,2]  , &[0,2]   , &[2,2]]];

pub const L_PIECE: &[&[&[u8]]] = &[
    &[&[0,0,3], &[3,3,3]],
    &[&[3,0]  , &[3,0]   , &[3,3]],
    &[&[3,3,3], &[3,0,0]],
    &[&[3,3]  , &[0,3]   , &[0,3]]];

pub const O_PIECE: &[&[&[u8]]] = &[
    &[&[4,4],   &[4,4]]];

pub const S_PIECE: &[&[&[u8]]] = &[
    &[&[0,5,5], &[5,5,0]],
    &[&[5,0]  , &[5,5]   , &[0,5]]];

pub const Z_PIECE: &[&[&[u8]]] = &[
    &[&[6,6,0], &[0,6,6]],
    &[&[0,6]  , &[6,6]   , &[6,0]]];

pub const T_PIECE: &[&[&[u8]]] = &[
    &[&[0,7,0], &[7,7,7]],
    &[&[7,0]  , &[7,7]   , &[7,0]],
    &[&[7,7,7], &[0,7,0]],
    &[&[0,7]  , &[7,7]   , &[0,7]]];

pub const PIECES: &[&[&[&[u8]]]] = &[I_PIECE,J_PIECE,L_PIECE,O_PIECE,S_PIECE,Z_PIECE,T_PIECE];


pub fn check_collision(board: &[[u8;BOARD_WIDTH];BOARD_HEIGHT], piece: &[&[u8]], y: i32, x: i32) -> bool {
    if is_out_of_bounds(piece, y, x) {
        return true;
    }
    let x = x as usize;
    let y = y as usize;
    ! piece.iter()
           .enumerate()
           .all(|(dy, row)| {
            row.iter()
               .enumerate()
               .filter(|(_, &cell)| cell > 0)
               .all(|(dx, _)| board[y+dy][x+dx] == 0)
        })
}

fn is_out_of_bounds(piece: &[&[u8]], y: i32, x: i32) -> bool {
    (y + piece.len() as i32) > (BOARD_HEIGHT as i32) ||
    (x + piece[0].len() as i32) > (BOARD_WIDTH as i32 ) ||
    x < 0 
}

pub fn clear_rows(board: &mut [[u8;BOARD_WIDTH];BOARD_HEIGHT]){
    for y in (1..board.len()).rev() {
        while board[y].iter().all(|cv| *cv > 0){
            shift_down(board, y);
        }
    }

}

fn shift_down(board: &mut [[u8;BOARD_WIDTH];BOARD_HEIGHT], into_row: usize) {
    for y in (1..=into_row).rev() {
        for x in 0..board[0].len() {
            board[y][x] = board[y-1][x];
        }
    }
    board[0].iter_mut().for_each(|cell| *cell = 0);
}

pub fn place_on_board(board: &mut [[u8;BOARD_WIDTH];BOARD_HEIGHT],  x: i32, y: i32, piece: &[&[u8]]){
    let x = x as usize;
    let y = y as usize;
    for (dy, row) in piece.iter().enumerate() {
        for (dx, &val) in row.iter().enumerate()
                                    .filter(|(_,val)| **val > 0) {
            board[y+dy][x+dx] = val;
        } 
    }
}