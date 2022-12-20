fn main() { /// Initialize an empty 3x3 board and set the first player to 'X'
    let mut board = [[' '; 3]; 3];
    let mut player = 'X';

    loop {
        /// Print the current board
        println!("Current board:");
        for row in &board {
            println!("{} | {} | {}", row[0], row[1], row[2]);
        }
         /// Prompt the current player to enter their move
        println!("Player {}'s turn. Enter row and column:", player);
         
        /// Read the player's move from standard input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut coord: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        let row = coord[0];
        let col = coord[1];
        
        /// Validate the player's move
        if row > 2 || col > 2 {
            println!("Invalid move. Try again.");
            continue;
        }

        if board[row][col] != ' ' {
            println!("Position already occupied. Try again.");
            continue;
        }

        board[row][col] = player;

        if check_win(&board, player) {
            println!("Player {} wins!", player);
            break;
        }

        if check_draw(&board) {
            println!("It's a draw!");
            break;
        }

        if player == 'X' {
            player = 'O';
        } else {
            player = 'X';
        }
    }
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool { /// checks if the player has won by checking for three in a row in any direction
    /// check rows
    for row in board {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }

    /// check columns
    for col in 0..3 {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }

    /// check diagonals
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

fn check_draw(board: &[[char; 3]; 3]) -> bool { /// checks if the board is full and no one has won.
    for row in board {
        for col in row {
            if *col == ' ' {
                return false;
            }
        }
    }

    true
}
