use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::{COLOR_BLACK, COLOR_RED, COLOR_WHITE, COLOR_GREEN, COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_YELLOW};
use crate::{NCURSES_GETCH_TIMOUT, GameState};
use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
pub fn init_ncurses() {
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(ncurses::stdscr(),true);
    ncurses::curs_set(CURSOR_INVISIBLE);
    ncurses::start_color();
    // Colors to be used with blocks
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

pub fn draw_score_win(w: ncurses::WINDOW, gs: &GameState) {
    let score_str = &gs.score.to_string();
    let score_pos = 13 - score_str.len() as i32;
    let rc_str    = &gs.rows_cleared.to_string();
    let rc_pos    = 13 - rc_str.len() as i32;
    let lvl_str   = &gs.level.to_string();
    let lvl_pos   = 13 - lvl_str.len() as i32;
    ncurses::wmove(w, 0, 0);
    ncurses::waddstr(w, "Level:");
    ncurses::wmove(w, 0, lvl_pos);
    ncurses::waddstr(w, lvl_str);

    ncurses::wmove(w, 1, 0);
    ncurses::waddstr(w, "Score: ");
    ncurses::wmove(w, 1,score_pos);
    ncurses::waddstr(w, score_str);

    ncurses::wmove(w, 2,0);
    ncurses::waddstr(w, "Cleared: ");
    ncurses::wmove(w, 2,rc_pos);
    ncurses::waddstr(w, rc_str);
    
    ncurses::wnoutrefresh(w);
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

pub fn draw_pause_screen(game_window: ncurses::WINDOW) {
    ncurses::wmove(
        game_window,
        ((BOARD_HEIGHT)/2) as i32,
        ((2*BOARD_WIDTH+2)/2 - "PAUSED".len()/2) as i32);
    ncurses::waddstr(game_window, "PAUSED");
    ncurses::wnoutrefresh(game_window);
}

pub fn doupdate() {ncurses::doupdate();}
pub fn newwin(lines: usize, cols: usize, y: usize, x:usize) -> ncurses::WINDOW {
    ncurses::newwin(lines as i32, cols as i32, y as i32, x as i32)
}
