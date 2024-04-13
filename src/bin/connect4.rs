use connect4::*; // everything from lib.rs

fn main() {
    let mut state = State {
        kind: StateKind::Live,
        player1: Player {
            name: "Adrian".to_string(),
            strategy: manual_strategy,
            // strategy: random_strategy,
        },
        player2: Player {
            name: "Bottie".to_string(),
            // strategy: random_strategy,
            // strategy: minimax_strategy3,
            strategy: minimax_strategy_level::<3>,
        },
        moves_player1: Vec::new(),
        moves_player2: Vec::new(),
        height: [0; 7],
    };

    while state.kind == StateKind::Live {
        println!("{:#?}", state);
        // println!("Moves player1: {:?}", state.moves_player1);
        // println!("Moves player2: {:?}", state.moves_player2);
        state = play(state);
    }
    let name = if state.moves_player1.len() == state.moves_player2.len() {
        &state.player2.name
    } else {
        &state.player1.name
    };
    println!("{:#?}", state);
    println!("GAME OVER!  {} won!", name);
}

