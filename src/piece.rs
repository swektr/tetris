pub type Piece = &'static[&'static[&'static[u8]]];
pub type PieceState = &'static[&'static[u8]];

pub trait PieceImpl {
    fn state(self, rotation: usize) -> PieceState;
}

impl PieceImpl for Piece {
    fn state(self, state: usize) -> PieceState {
        self[state]
    }
}

pub trait PieceStateImpl {
    /// Get the width of the piece in its current state
    fn width(self) -> usize;
    /// Get the height of the piece in its current state
    fn height(self) -> usize;
    /// Get the piece's inner (iterable) data structure
    fn inner(self) -> &'static[&'static[u8]];
    /// Get the value of the peiece at it's (y,x) cord
    fn at(self, y: usize, x: usize) -> u8;
}

impl PieceStateImpl for PieceState {
    fn width(self) -> usize {
        self[0].len()
    }
    
    fn height(self) -> usize {
        self.len()
    }
    
    fn at(self, y: usize, x: usize) -> u8 {
        self[y][x]
    }

    fn inner(self) -> &'static[&'static [u8]] {
        // Right now PieceState is just an alias so this
        // is just for semantics perhaps .
        self
    }
    
}

pub const I_PIECE: Piece = &[ 
    &[&[1,1,1,1]],
    &[&[1],&[1],&[1],&[1]]];

pub const J_PIECE: Piece = &[
    &[&[2,0,0], &[2,2,2]],
    &[&[2,2]  , &[2,0]   , &[2,0]],
    &[&[2,2,2], &[0,0,2]],
    &[&[0,2]  , &[0,2]   , &[2,2]]];

pub const L_PIECE: Piece = &[
    &[&[0,0,3], &[3,3,3]],
    &[&[3,0]  , &[3,0]   , &[3,3]],
    &[&[3,3,3], &[3,0,0]],
    &[&[3,3]  , &[0,3]   , &[0,3]]];

pub const O_PIECE: Piece = &[
    &[&[4,4],   &[4,4]]];

pub const S_PIECE: Piece = &[
    &[&[0,5,5], &[5,5,0]],
    &[&[5,0]  , &[5,5]   , &[0,5]]];

pub const Z_PIECE: Piece = &[
    &[&[6,6,0], &[0,6,6]],
    &[&[0,6]  , &[6,6]   , &[6,0]]];

pub const T_PIECE: Piece = &[
    &[&[0,7,0], &[7,7,7]],
    &[&[7,0]  , &[7,7]   , &[7,0]],
    &[&[7,7,7], &[0,7,0]],
    &[&[0,7]  , &[7,7]   , &[0,7]]];

pub const PIECES: &[Piece] = &[I_PIECE,J_PIECE,L_PIECE,O_PIECE,S_PIECE,Z_PIECE,T_PIECE];

pub fn gen_piece() -> Piece {
    PIECES[rand::random::<usize>()%PIECES.len()]
}