use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_WHITE, COLOR_GREEN, COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_YELLOW};
use crate::{NCURSES_GETCH_TIMOUT};
use crate::board::Board;
pub fn init_ncurses() {
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(ncurses::stdscr(),true);
    ncurses::timeout(-1);
    ncurses::curs_set(CURSOR_INVISIBLE);
    ncurses::start_color();

    ncurses::init_pair(1, COLOR_BLACK, COLOR_WHITE);
    ncurses::init_pair(2, COLOR_BLACK, COLOR_RED);
    ncurses::init_pair(3, COLOR_BLACK, COLOR_GREEN);
    ncurses::init_pair(4, COLOR_BLACK, COLOR_CYAN);
    ncurses::init_pair(5, COLOR_BLACK, COLOR_BLUE);
    ncurses::init_pair(6, COLOR_BLACK, COLOR_MAGENTA);
    ncurses::init_pair(7, COLOR_BLACK, COLOR_YELLOW);
    
    ncurses::refresh();
    ncurses::timeout(NCURSES_GETCH_TIMOUT);
}
pub fn draw_block(w: ncurses::WINDOW, color: i16){
    ncurses::waddch(w, ' ' as u32 | ncurses::COLOR_PAIR(color));
    ncurses::waddch(w, ' ' as u32 | ncurses::COLOR_PAIR(color));
}

pub fn draw_piece(w: ncurses::WINDOW, piece: &[&[u8]], y: i32,  x: i32) {
    let height = piece.len() as i32;
    let width = piece[0].len() as i32 * 2; // Blocks are 2 chars wide
    // convert cords to ncurses window dimension; 
    let y = y+1;
    let x = 2*x+1;
    for y in (y..y+height).enumerate() {
        for x in (x..x+width).step_by(2).enumerate() { //Block = WxH = 2x1 
            let (piece_y, board_y) = y;
            let (piece_x, board_x) = x;
            ncurses::wmove(w, board_y, board_x);
            if piece[piece_y][piece_x] > 0 {
                draw_block(w, piece[piece_y][piece_x] as i16);
            }
        }
    }
    ncurses::wnoutrefresh(w);
}


pub fn draw_next(next_window: ncurses::WINDOW, next_piece: &[&[u8]]) {
    ncurses::werase(next_window);
    ncurses::box_(next_window, 0, 0);
    ncurses::waddstr(next_window, "next:" );
    ncurses::wnoutrefresh(next_window);
    draw_piece(next_window, next_piece,1, 0);
}

pub fn draw_board(w: ncurses::WINDOW, board: &Board){
    ncurses::werase(w);
    ncurses::box_(w, 0, 0);
    for i in 0..board.height() {
        ncurses::wmove(w, (i+1) as i32, 1);
        for j in 0..board.width() {
            if board.data[i][j] > 0 {
                draw_block(w, board.data[i][j] as i16);
                
            } else{
                ncurses::wmove(w,(i+1) as i32, 3+2*j as i32);
            }
        }
    }
    ncurses::wnoutrefresh(w);
}

pub fn doupdate() {ncurses::doupdate();}
pub fn newwin(lines: usize, cols: usize, y: usize, x:usize) -> ncurses::WINDOW {
    ncurses::newwin(lines as i32, cols as i32, y as i32, x as i32)
} 
