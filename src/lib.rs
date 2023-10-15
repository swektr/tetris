use std::time::Duration;

pub mod gfx;
pub mod board;
pub mod piece;
pub const TICKS_PER_SECOND: i32 = 60;
pub const NCURSES_GETCH_TIMOUT: i32 = 1000/TICKS_PER_SECOND as i32;
pub const TICK_DUR_MILLIS: Duration = Duration::from_millis(NCURSES_GETCH_TIMOUT as u64);

/*
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
*/

pub struct GameState {
    pub score: i32,
    pub rows_cleared: i32,
    pub level: i32,
}

/*
pub fn gen_piece<'a>(pieces: &'a[&'a[&'a[&'a[u8]]]]) -> &'a[&'a[&'a[u8]]] {
    pieces[rand::random::<usize>()%PIECES.len()]
}
*/

pub fn pause_capture_quit() -> bool {
    ncurses::timeout(-1);
    let propagate_quit = loop {
        match ncurses::getch() {
            113 => break true,
            112 => break false,
            _   => (),
        }
    };
    ncurses::timeout(NCURSES_GETCH_TIMOUT);
    propagate_quit
}