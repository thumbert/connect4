
use clap::Parser;
use connect4::*; 

/// Play Connect4 from the terminal line
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Level of the computer strength (1-5)
    #[arg(short, long, default_value="3")]
    level: usize,
    /// Name of the human player
    #[arg(short, long, default_value="Anonymous")]
    name: String,
}


fn main() {
    let args = Args::parse();
    println!("Game level: {}", args.level); 
    let strategy = match args.level {
        1 => minimax_strategy_level::<1>,
        2 => minimax_strategy_level::<2>,
        3 => minimax_strategy_level::<3>,
        4 => minimax_strategy_level::<4>,
        5 => minimax_strategy_level::<5>,
        _ => panic!("Unsupported level {}", args.level),
    };

    let mut state = State {
        kind: StateKind::Live,
        player1: Player {
            name: args.name,
            strategy: manual_strategy,
        },
        player2: Player {
            name: "Botty".to_string(),
            // strategy: random_strategy,
            strategy,
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

