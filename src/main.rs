use std::{thread, process::exit};
use tetris::*;
use board::*;
use gfx;

fn main() {
    let mut board = Board::new();
    gfx::init_ncurses();
    ctrlc::set_handler(|| {
        ncurses::endwin();
        exit(0)
    }).expect("could not set Ctrl-C handler");

    let game_window  = gfx::newwin(BOARD_HEIGHT+2, 2*BOARD_WIDTH+2, 0, 0);
    let next_window  = gfx::newwin(6, 10,0, 2*BOARD_WIDTH+2);
    let score_window = gfx::newwin(6, 13, 8, 2*BOARD_WIDTH+2);
    
    let mut curr_piece = gen_piece(PIECES);
    let mut next_piece = gen_piece(PIECES);

    // Piece position and rotation
    let mut rot: usize = 0;
    let mut y : i32 = 0;
    let mut x = 10/2 - curr_piece[rot][0].len() as i32 / 2;
    let mut dx: i32 = 0;

    let mut n_ticks: usize = 0;
    let tpa: usize = 40; // Ticks per action
    let mut speed_mult = 1;
    let min_tpa = 4; 
    let mut gs: GameState = GameState {
        score: 0,
        rows_cleared: 0,
        level: 0,
    };

    loop {
        gfx::draw_next(next_window, next_piece[0]);
        gfx::draw_board(game_window, &board);
        gfx::draw_score_win(score_window, &gs);
        if dx != 0 && ! board.check_collision(curr_piece[rot], y, x+dx){
            x = x+dx;
        }
        
        let tick_threshold = usize::max(
            min_tpa,
            tpa.saturating_sub(tpa/20 * gs.level as usize) / speed_mult
        );

        if n_ticks >= tick_threshold {
            n_ticks = 0;
            if ! board.check_collision(curr_piece[rot], y+1, x){
                y += 1;
            } else {
                board.place_piece(curr_piece[rot], y as usize, x as usize);
                curr_piece = next_piece;
                next_piece = gen_piece(PIECES);
                let cleared = board.clear_rows();
                if cleared > 0 {
                    gs.score += (5*gs.level+100)<<(cleared-1);
                    gs.rows_cleared += cleared;
                    if gs.rows_cleared >= 8 * (gs.level+1) {
                        gs.level += 1;
                    }
                }
                rot = 0;
                y   = 0;
                x   = 10/2 - curr_piece[rot][0].len() as i32 / 2;
                speed_mult = 1;
                // Check for overflow
                if board.check_collision(curr_piece[rot], y, x) {
                    break;
                }
            }
        }
        gfx::draw_piece(game_window, curr_piece[rot], y, x);
        gfx::doupdate();
        
        let tick = thread::spawn(|| {
            thread::sleep(TICK_DUR_MILLIS)
        });
        dx = 0;
        match ncurses::getch() {
            ncurses::KEY_LEFT  => dx = -1,
            ncurses::KEY_RIGHT => dx =  1,
            ncurses::KEY_UP    => {
                let temp_rot = (rot + 1) % curr_piece.len();
                if ! board.check_collision(curr_piece[temp_rot], y, x) {
                    rot = temp_rot;
                }
            },
            ncurses::KEY_DOWN  => {
                speed_mult = if speed_mult > 1 {
                    1
                } else {
                    16
                }
            },
            /* (p)ause */ 112 => {
                gfx::draw_pause_screen(game_window);
                gfx::doupdate();
                if pause_capture_quit() { break }
            },
            /* (q)uit  */ 113 => break,
            _ => (),
        };
        tick.join().unwrap();
        n_ticks = n_ticks.wrapping_add(1);
    }
    ncurses::endwin();
}