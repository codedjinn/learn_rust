
use std::fmt;

const SQUARE:i32 = 0;
const PAWN:i32 = 1;
const KNIGHT:i32 = 2;
const BISHOP:i32 = 3;
const ROOK:i32 = 4;
const QUEEN:i32 = 5;
const KING:i32 = 6;

#[derive(Copy, Clone, Debug)]
struct Piece
{
    x: i32,
    y: i32
}

impl fmt::Display for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "x {} y {}", self.x, self.y)
    }
}

fn main()
{
    let board_input = 
               "...K....\
                ........\
                .B......\
                ......P.\
                ........\
                ..N.....\
                ....R...\
                .....Q..";

    println!("{:?}", board_input);

    let mut board1d = vec![0; 64];

    let mut king = Piece { x: 0, y: 0 };
    let mut queen = Piece { x: 0, y: 0 };
    let mut rook = Piece { x: 0, y: 0 };
    let mut knight = Piece { x: 0, y: 0 };
    let mut bishop = Piece { x: 0, y: 0 };
    let mut pawn = Piece { x: 0, y: 0 };

    let mut x = 0;
    let mut y = 0;

    let mut index = 0;
    for ch in board_input.chars()
    {
        let val: i32 = match ch {
            '.' => SQUARE,
            'K' => {
                king.x = x; 
                king.y = y;

                println!("{:?}", king);

                KING
            },
            'Q' => {
                queen.x = x;
                queen.y = y;

                QUEEN
            },
            'R' => {
                rook.x = x;
                rook.y = y;             
                
                println!("{:?}", rook);

                ROOK
            }
            'B' => {
                bishop.x = x;
                bishop.y = y;
                BISHOP
            },
            'N' => {
                knight.x = x;
                knight.y = y;
                KNIGHT
            },
            'P' => 
            {
                pawn.x = x;
                pawn.y = y;
                PAWN
            },
             _ => -1
        };

        board1d[index] = val;

        index = index + 1;

        x = x + 1;
        if x > 7
        {
            x = 0;
            y = y + 1;
        }
    }

    let slice = &board1d[..];

    check_bishop(king, bishop, slice);

    let mut check = check_rook(king, rook, slice);

    if check
    {
        println!("KING IS IN CHECK!");
    }
    else
    {
        println!("KING IS NOT IN CHECK!");
    }
}

fn check_bishop(king: Piece, bishop: Piece, board: &[i32]) -> bool
{
    // use pyth to determine if we are in line
    let dx = king.x - bishop.x;
    let dy = king.y - bishop.y;
    
    println!("dx {0} dy {1}", dx, dy);
    
    let squared = (dx * dx) + (dy * dy);
    let r = (squared as f64).sqrt();

    println!("board {:?}", board);

    println!("r {}", r);

    return false;
}

fn check_rook(king: Piece, rook: Piece, board: &[i32]) -> bool
{
    println!("check_rook()");
    
    // we can first test if the align, then we can
    // test if the rook is blocked

    // same vertical
    if rook.x == king.x
    {
        println!("rook same vertical");

        // check if we are obstructed

        // scan up
        if rook.y > king.y
        {
            println!("scan up");

            let len = rook.y - king.y;

            for i in 1..len
            {
                let pos = 8 * (rook.y - i) + rook.x;
                let ind = &board[pos as usize];
                if *ind != 0
                {
                    return false;
                }
            }
            return true;
        }
        else
        {
            println!("scan down!");

            let len = king.y - rook.y;
            for i in 1..len
            {
                let pos = 8 * (rook.y + i) + rook.x;
                let ind = &board[pos as usize];
                if *ind != 0
                {
                    return false;
                }
            }
            return true;
        }
    }
    else if rook.y == king.y
    {
        println!("rook same horizontal");
    }

    return false;
}

// fn check_queen(king: Piece, queen: Piece) -> bool
// {
//     return false;
// }