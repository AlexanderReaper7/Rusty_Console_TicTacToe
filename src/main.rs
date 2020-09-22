use std::io;
use std::str::FromStr;
use std::error::Error;
use std::io::*;


const N : usize = 3;
const DEFAULT_CHAR : char = '*';
const PLAYER1 : char = 'X';
const PLAYER2 : char = 'O';


fn main() {
    // Init
    let mut grid = Grid::default();
    // print instructions and wait for user 'ok'
    print_instructions();
    print!("Ready? : ");
    stdout().flush();
    {let _input : String = read_line().unwrap_or_default();}
    let mut current_player : char = PLAYER1;
    let mut marks_count : usize = 0;
    // run
    loop {
        // show grid
        grid.print();
        // get user input
        let coords = get_coords(&current_player);
        // try to claim coords
        if grid.claim(coords, current_player) {
            // check win
            match grid.check_win(coords, current_player, marks_count) {
                Some(val) => if val {
                    // Win for current player
                }else{
                    // Draw
                },
                _ => ()
            };
        }
        else {
            println!("Invalid input, try again.");
            continue
        }
        // start next turn
        marks_count += 1;
        current_player = match current_player {
            PLAYER1 => PLAYER2,
            PLAYER2 => PLAYER1,
            _ => panic!()
        };
    }
}

fn get_coords(current_player: &char) -> [usize;2] {
    loop {
        // get input
        print!("input coord for player \"{}\" ", current_player);
        stdout().flush();
        let input : String = match read_line() {
            Ok(val) => val,
            Err(_error) => {println!("Invalid input, try again."); continue}
        };
        // split
        let input_arr : Vec<&str> = input.split(' ').collect();
        if input_arr.len() != 2 {println!("Invalid input, try again."); continue}
        let mut output: [usize;2] = [0;2]; // TODO: potentially bad code!
        // parse
        for (index, value) in input_arr.iter().enumerate() {
            match value.parse::<usize>() {
                Ok(n) => output[index] = n - 1,
                Err(_e) => {println!("Input not a valid number, try again."); continue}
              }
        }
        return output
    }
}

fn print_instructions() {
    println!("    1. The game is played on a grid that's {N} by {N} squares.
    2. You are \'{PLAYER1}\', your friend is \'{PLAYER2}\'. Players take turns putting their marks in empty squares.
    3. Empty squares are shown as '{DEFAULT_CHAR}'.
    4. Mark a square by entering an empty square´s coordinates separated by a space in the format \"X Y\".
    5. The first player to get {N} of their marks in a row (vertically, horizontally or diagonally) is the winner.
    6. When all {N2} squares are full, the game is over. If no player has {N} marks in a row, the game ends as a draw."
    , N = N, PLAYER1=PLAYER1,PLAYER2=PLAYER2,DEFAULT_CHAR=DEFAULT_CHAR,N2=N*N)
}


fn read_line<T: FromStr>() -> Result<T>
where <T as FromStr>::Err: Error + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    stdout().flush();
    Ok(input.trim().parse().unwrap())
}


struct Grid {
    squares: [[char;N];N]
}

impl Default for Grid {
    fn default() -> Self { 
        Self{squares : [[DEFAULT_CHAR;N];N]}
     }
}

impl Grid {
    fn new(empty_char: char) -> Self {
        Self{squares : [[empty_char;N];N]}
    }

    fn claim(&mut self, coords: [usize;2], value: char) -> bool {
        for elem in coords.iter() {
            if elem >= &N {
                return false
            }
        }
        // claim square if it is empty
        if self.squares[coords[0]][coords[1]] == DEFAULT_CHAR {
            self.squares[coords[0]][coords[1]] = value;
            return true
        }
        false
    }

    fn check_win(&self, coords: [usize;2], current_player: char, move_count: usize) -> Option<bool> {
        // check col
        for i in 0..N {
            if self.squares[coords[1]][i] != current_player {
                break;
            }
            if i == N-1{
                // report win
                return Some(true);
            }
        }
        // check row
        for i in 0..N {
            if self.squares[i][coords[0]] != current_player{
                break;
            }
            if i == N-1 {
                // report win
                return Some(true)
            }
        }
        // check diag
        if coords[0] == coords[1] {
            // we're on a diagonal
            for i in 0..N {
                if self.squares[i][i] != current_player{
                    break;
                }
                if i == N-1 {
                    // report win
                    return Some(true)
                }
            }
        }
        // check anti diag
        if coords[0] + coords[1] == N - 1 {
            for i in 0..N {
                if self.squares[i][(N-1)-i] != current_player{
                    break;
                }
                if i == N-1 {
                    // report win
                    return Some(true)
                }
            }
        }
        // check draw
        if move_count == (N*N - 1) {
            // report draw
            return Some(false)
        }
        return None
    }

    fn print(&self) {
        let mut output: String = "\n".to_string();
        for row in self.squares.iter() {
            for (i, sq) in row.iter().enumerate() {
                output.push(*sq);  // add this square´s character to output
                if (i + 1) % N == 0 { // if at the end of row, add new-line character
                    output.push('\n');
                }  
                else {  // else add space
                    output.push(' ');
                }
            }
        }
        println!("{}", output);
        stdout().flush();
    }
}