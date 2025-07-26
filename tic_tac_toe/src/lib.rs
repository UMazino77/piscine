pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O',table) || horizontal('O', table) || vertical('O',table) {
        return "player O won".to_string()
    } else if diagonals('X',table) || horizontal('X', table) || vertical('X',table) {
        return "player X won".to_string()
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][2] == player && table [1][1] == player && table[2][0] == player {
        return true ;
    }
    for i in 0..3 {
        if table[i][i] != player {
            return false
        }
    }
    true
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][i] == player && player == table[i][(i+2)%3] && player == table[i][(i+1)%3]{
            return true
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][i] == player && player == table[(i+2)%3][i] && player == table[(i+1)%3][i] {
            return true
        }
    }
    false
}