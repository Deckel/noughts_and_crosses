use std::io;
use std::collections::HashMap;

fn main() {
    
    let mut won: bool = false;
    let mut player: u32 = 1;

    // create a mutable game state vector
    let mut game_state = Vec::new();
    game_state.resize(9, 0u32);

    // create a mutable hash map of allowd moves
    let mut moves = HashMap::new();
    moves = create_map();

    while !won {    
        
        // build the board
        build_board(game_state.clone());
        
        // check if win condition is true
        check_win(game_state.clone());

        // get user input for a move
        let input_move = get_user_input(&moves);

        // update game state
        game_state = update_game_state(&moves, &input_move, game_state, player);

        // update avaliable moves
        moves = update_moves(moves, &input_move);

        // change player
        if player == 1 {
            player = 2;
        } else if player == 2 {
            player = 1;
        }
    }
}

fn get_user_input() -> String {
    // prompt user
    println!("Select a move:");
    // get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // trim out error
    let input = input.trim();
    // convert from pointer to object
    input
}

fn create_map() -> HashMap<String, u32> {
    // why do we need to create another mutable object here called moves and at the same time create it in
    // the outer scope as well, if this returns a mutable hashmap

    // create the hash map
    let mut moves = HashMap::new();

    // insert all possible moves
    moves.insert("a1".to_string(), 0);
    moves.insert("b1".to_string(), 1);
    moves.insert("c1".to_string(), 2);
    moves.insert("a2".to_string(), 2);
    moves.insert("b2".to_string(), 4);
    moves.insert("c2".to_string(), 5);
    moves.insert("a3".to_string(), 6);
    moves.insert("b3".to_string(), 7);
    moves.insert("c3".to_string(), 8);
    
    // return the hash map
    moves
}

fn update_moves(mut moves: HashMap<String, u32>, input_move: &String) -> HashMap<String, u32> {
    moves.remove(input_move);
    // return the hash map
    moves
}

fn update_game_state( moves: &HashMap<String, u32>, input_move: &String, mut game_state: Vec<u32>, player: u32) -> Vec <u32> {
    
    let index: usize = *moves.get(input_move).unwrap() as usize;

    if player == 1 {
        game_state[index] = 1;
    } else if player == 2 {
        game_state[index] = 2;
    }
    
    game_state
}

fn build_board(game_state: Vec<u32>) {

    let mut symbol_set = HashMap::new();

    symbol_set.insert(0, ".".to_string());
    symbol_set.insert(1, "x".to_string());
    symbol_set.insert(2, "o".to_string());

    println!("  a b c \n\
       1 {} {} {} \n\
       2 {} {} {} \n\
       3 {} {} {} \n",
       symbol_set.get(&game_state[0]).unwrap(),
       symbol_set.get(&game_state[1]).unwrap(),
       symbol_set.get(&game_state[2]).unwrap(),
       symbol_set.get(&game_state[3]).unwrap(),
       symbol_set.get(&game_state[4]).unwrap(),
       symbol_set.get(&game_state[5]).unwrap(),
       symbol_set.get(&game_state[6]).unwrap(),
       symbol_set.get(&game_state[7]).unwrap(),
       symbol_set.get(&game_state[8]).unwrap()
    )
}

fn check_win(game_state: Vec<u32>) {

}
