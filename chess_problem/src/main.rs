use std::vec;


enum PieceType
{
    None = 0,

    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,

    Queen = 5,
    King = 7
}

struct Piece
{
    x: u16,
    y: u16,
    kind: PieceType
}

#[derive(Debug)]
struct Board
{
    board: Vec<i32>
}

fn main()
{
    // let b = Board::new();

    // println!("{:?}", b);
}