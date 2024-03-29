pub mod errors;
pub mod tui;

use core::fmt;
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::{max, min};

type Move = u8;
type Strategy = fn(&State) -> Move;

// #[derive(Copy)]
pub struct Player {
    name: String,
    strategy: Strategy,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            strategy: self.strategy.clone(),
        }
    }
}

#[allow(dead_code)]
fn random_strategy(state: &State) -> Move {
    let mut rng = rand::thread_rng();
    let actions = state.actions();
    let m = rng.gen_range(0..actions.len());
    let name = if state.moves_player1.len() == state.moves_player2.len() {
        &state.player1.name
    } else {
        &state.player2.name
    };
    println!("{} added to column {}", name, 1 + actions[m] / 6);
    actions[m]
}

#[allow(dead_code)]
fn manual_strategy(state: &State) -> Move {
    let mut line = String::new();
    let mut number = 0;
    while number < 1 || number > 7 {
        let name = if state.moves_player1.len() == state.moves_player2.len() {
            &state.player1.name
        } else {
            &state.player2.name
        };
        println!("{}, enter column number (1-7) and press Enter:", name);
        std::io::stdin().read_line(&mut line).unwrap();
        number = match line.trim().parse() {
            Ok(number) => number,
            Err(_e) => 0,
        };
    }
    (number as u8 - 1) * 6 + state.height[number - 1]
}

#[allow(dead_code)]
fn minimax_strategy1(state: &State) -> Move {
    let max_depth: usize = 2;
    let actions = state.actions();
    let values: Vec<_> = actions
        .iter()
        .map(|m| {
            let state_new = state.result(*m);
            min_value(&state_new, 0, max_depth)
        })
        .collect();
    actions[values.iter().position_max().unwrap()]
}

#[allow(dead_code)]
fn minimax_strategy(state: &State) -> Move {
    let mut rng = rand::thread_rng();
    let max_depth: usize = 6;
    let actions = state.actions();
    let values: Vec<_> = actions
        .par_iter()
        .map(|m| {
            let state_new = state.result(*m);
            min_value(&state_new, 0, max_depth)
        })
        .collect();
    // While you should pick the max value from the [values] vector,
    // there may be several choices that have the same value.  So which one
    // should you pick?  Pick one randomly!  This makes the algo less predictable.
    let max_value = *values.iter().max().unwrap();
    let candidates: Vec<_> = values
        .iter()
        .enumerate()
        .filter(|(_, &e)| e == max_value)
        .map(|(index, _)| index)
        .collect();
    let i = rng.gen_range(0..candidates.len());
    println!("Actions are {:?}", actions);
    println!("Values are {:?}", values);
    println!("Max value is {}", max_value);
    println!("Candidates are {:?}", candidates);
    println!("Selected index i {}", i);
    actions[candidates[i]]
}

/// Not quite working yet!
// fn minimax_strategy_level(level: usize) -> impl Fn(&State) -> Move {
//     move |state: &State| {
//         let max_depth: usize = level;
//         let actions = state.actions();
//         let values: Vec<_> = actions
//             .iter()
//             .map(|m| {
//                 let state_new = state.result(*m);
//                 min_value(&state_new, 0, max_depth)
//             })
//             .collect();
//         actions[values.iter().position_max().unwrap()]
//     }
// }

fn max_value(state: &State, depth: usize, max_depth: usize) -> i32 {
    let depth = depth + 1;
    if state.kind == StateKind::Final {
        return -1001 + depth as i32;
    }
    let mut value = -999;
    if depth > max_depth {
        return value;
    }
    for m in state.actions() {
        value = max(value, min_value(&state.result(m), depth, max_depth))
    }
    value
}
fn min_value(state: &State, depth: usize, max_depth: usize) -> i32 {
    let depth = depth + 1;
    if state.kind == StateKind::Final {
        return 1001 - depth as i32;
    }
    let mut value = 999;
    if depth > max_depth {
        return value;
    }
    for m in state.actions() {
        value = min(value, max_value(&state.result(m), depth, max_depth));
    }
    value
}

#[derive(PartialEq, Clone)]
enum StateKind {
    Final,
    Live,
}

fn play(current: State) -> State {
    let current_state = &current;
    let current_player = if current_state.moves_player1.len() == current_state.moves_player2.len() {
        &current.player1
    } else {
        &current.player2
    };

    let m = (current_player.strategy)(current_state);
    // println!("{} moves to {}", current_player.name, m);
    current_state.result(m)
}

#[derive(Clone)]
pub struct State {
    kind: StateKind,
    player1: Player,
    player2: Player,
    moves_player1: Vec<Move>,
    moves_player2: Vec<Move>,
    height: [u8; 7],
}

impl State {
    fn result(&self, m: Move) -> State {
        let mut moves1 = self.moves_player1.clone();
        let mut moves2 = self.moves_player2.clone();
        if moves1.len() == moves2.len() {
            moves1.push(m);
        } else {
            moves2.push(m);
        };
        let mut height = self.height.clone();
        height[(m / 6) as usize] += 1;
        // println!("Moves player1: {:?}", moves1);
        // println!("Moves player2: {:?}", moves2);
        // println!("Height: {:?}", height);

        if self.is_winning_move(m) {
            return State {
                kind: StateKind::Final,
                player1: Player {
                    name: self.player1.name.clone(),
                    strategy: self.player1.strategy,
                },
                player2: Player {
                    name: self.player2.name.clone(),
                    strategy: self.player2.strategy,
                },
                moves_player1: moves1,
                moves_player2: moves2,
                height: height,
            };
        } else {
            return State {
                kind: StateKind::Live,
                player1: Player {
                    name: self.player1.name.clone(),
                    strategy: self.player1.strategy,
                },
                player2: Player {
                    name: self.player2.name.clone(),
                    strategy: self.player2.strategy,
                },
                moves_player1: moves1,
                moves_player2: moves2,
                height: height,
            };
        }
    }

    /// Check if this move ends the game
    fn is_winning_move(&self, m: Move) -> bool {
        let moves = if self.moves_player1.len() == self.moves_player2.len() {
            self.moves_player1.clone()
        } else {
            self.moves_player2.clone()
        };
        if moves.len() < 3 {
            return false;
        }
        let combinations: Vec<_> = moves.into_iter().combinations(3).collect();
        for mut ms in combinations {
            ms.push(m);
            ms.sort();
            if is_winning_position(ms[0], ms[1], ms[2], ms[3]) {
                return true;
            }
        }
        false
    }

    /// Return the moves available for this state
    fn actions(&self) -> Vec<Move> {
        match self.kind {
            StateKind::Live => {
                let mut hs: Vec<Move> = Vec::new();
                for i in 0u8..7 {
                    if self.height[i as usize] < 6 {
                        hs.push(i * 6 + self.height[i as usize]);
                    }
                }
                hs
            }
            StateKind::Final => Vec::new(),
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows: Vec<String> = empty_board();
        for m in &self.moves_player1 {
            let i = (m % 6) as usize;
            let j = (m / 6) as usize;
            let row = rows[i].clone();
            let mut chars: Vec<char> = row.chars().collect();
            chars[j] = '🔴';
            rows[i] = chars.into_iter().collect();
        }
        for m in &self.moves_player2 {
            let i = (m % 6) as usize;
            let j = (m / 6) as usize;
            let row = rows[i].clone();
            let mut chars: Vec<char> = row.chars().collect();
            chars[j] = '🟡';
            rows[i] = chars.into_iter().collect();
        }
        rows.reverse();
        rows.push("\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}\u{2212}".to_string());
        rows.push("\u{2009}1 2 3 4 5 6 7".to_string());
        f.write_str(&rows.join("\n"))
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            kind: StateKind::Live,
            player1: Player {
                name: "Adrian".to_string(),
                strategy: manual_strategy,
                // strategy: random_strategy,
            },
            player2: Player {
                name: "Bottie".to_string(),
                // strategy: random_strategy,
                strategy: minimax_strategy,
            },
            moves_player1: Vec::new(),
            moves_player2: Vec::new(),
            height: [0; 7],
        }
    }
}

/// Get an empty board
fn empty_board() -> Vec<String> {
    vec![
        "⚪⚪⚪⚪⚪⚪⚪".into(),
        "⚪⚪⚪⚪⚪⚪⚪".into(),
        "⚪⚪⚪⚪⚪⚪⚪".into(),
        "⚪⚪⚪⚪⚪⚪⚪".into(),
        "⚪⚪⚪⚪⚪⚪⚪".into(),
        "⚪⚪⚪⚪⚪⚪⚪".into(),
    ]
}

fn is_winning_position(m0: Move, m1: Move, m2: Move, m3: Move) -> bool {
    let d10 = m1 - m0;
    let d21 = m2 - m1;
    let d32 = m3 - m2;
    if d10 == d21 && d21 == d32 {
        // horizontal
        if d10 == 6 {
            return true;
        }
        // vertical, you may have (21,22,23,24) which is not a winning position
        if d10 == 1 && m0 % 6 < 3 {
            return true;
        }
        // slope+1, (3,10,17,24) is not a winning position
        if d10 == 7 && m0 % 6 < 3 {
            return true;
        }
        // slope-1, (2,7,12,17) is not a winning position
        if d10 == 5 && m0 % 6 > 2 {
            return true;
        }
    }
    false
}
