use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::{self, Display, Formatter};

type ScoreType = i64;
const INF: ScoreType = ScoreType::MAX;

const H: usize = 3;
const W: usize = 4;
const END_TURN: usize = 4;
const DX: [i64; 4] = [1, -1, 0, 0];
const DY: [i64; 4] = [0, 0, 1, -1];

#[derive(Clone, Debug)]
struct Coord {
    y: i64,
    x: i64,
}

impl Default for Coord {
    fn default() -> Self {
        Self { y: 0, x: 0 }
    }
}

#[derive(Clone, Debug)]
struct MazeState {
    points: Vec<Vec<ScoreType>>,
    turn: usize,
    character: Coord,
    game_score: ScoreType,
    evaluated_score: ScoreType,
}

impl MazeState {
    fn from_seed(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let y = rng.gen_range(0..H);
        let x = rng.gen_range(0..W);

        let mut points = vec![vec![0; W]; H];

        for j in 0..H {
            for i in 0..W {
                if j == y && i == x {
                    continue;
                }
                points[j][i] = rng.gen_range(0..10);
            }
        }

        Self {
            points,
            turn: 0,
            character: Coord {
                y: y as i64,
                x: x as i64,
            },
            game_score: 0,
            evaluated_score: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn advance(&mut self, action: usize) {
        self.character.y += DY[action];
        self.character.x += DX[action];
        let point = &mut self.points[self.character.y as usize][self.character.x as usize];
        if *point > 0 {
            self.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 {
            let y = self.character.y + DY[action];
            let x = self.character.x + DX[action];
            if y >= 0 && y < H as i64 && x >= 0 && x < W as i64 {
                actions.push(action);
            }
        }
        actions
    }

    fn action(&self) -> usize {
        let legal_actions = self.legal_actions();
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        legal_actions[rng.gen_range(0..legal_actions.len())]
    }

    // fn evaluate_score(&mut self) {
    //     self.evaluated_score = self.game_score;
    // }

    // fn greedy_action(&self) -> usize {
    //     let legal_actions = self.legal_actions();
    //     let mut best_score = -INF;
    //     let mut best_action = None;
    //     for action in legal_actions {
    //         let mut now_state = self.clone();
    //         now_state.advance(action);
    //         now_state.evaluate_score();
    //         if now_state.evaluated_score > best_score {
    //             best_score = now_state.evaluated_score;
    //             best_action = Some(action);
    //         }
    //     }
    //     best_action.unwrap()
    // }

    fn play_game(seed: u64) {
        let mut state = Self::from_seed(seed);
        while !state.is_done() {
            state.advance(state.action()); // ランダム行動
            println!("{}", state);
        }
    }
}

impl Display for MazeState {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "turn:{}", self.turn)?;
        writeln!(f, "score:{}", self.game_score)?;
        let (y, x) = (self.character.y as usize, self.character.x as usize);
        for j in 0..H {
            for i in 0..W {
                if j == y && i == x {
                    write!(f, "@")?;
                } else if self.points[j][i] > 0 {
                    write!(f, "{}", self.points[j][i])?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn main() {
    MazeState::play_game(121321);
}
