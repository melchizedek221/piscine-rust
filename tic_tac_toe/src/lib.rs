// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if check_winner(&'O'.to_string(), &table) {
        "player O won".to_string()
    } else if check_winner(&'X'.to_string(), &table) {
        "player X won".to_string()
    } else {
        "tie".to_string()
    }
}

pub fn check_winner(player: &str, table: &Vec<Vec<&str>>) -> bool {
    diagonals(player, table) || horizontal(player, table) || vertical(player, table)
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (0..table.len()).all(|i| table[i][i] == player) ||
    (0..table.len()).all(|i| table[i][table.len() - 1 - i] == player)
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    table.iter().any(|row| row.iter().all(|&cell| cell == player))
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (0..table[0].len()).any(|col| (0..table.len()).all(|row| table[row][col] == player))
}