use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{
    collections::BinaryHeap,
    fmt::{self, Display, Formatter},
};

type ScoreType = i64;
const INF: ScoreType = ScoreType::MAX;

const H: usize = 3;
const W: usize = 4;
const END_TURN: usize = 4;
const DX: [i64; 4] = [1, -1, 0, 0];
const DY: [i64; 4] = [0, 0, 1, -1];

#[derive(PartialEq, Eq, Clone, Debug)]
struct Coord {
    y: i64,
    x: i64,
}

impl Default for Coord {
    fn default() -> Self {
        Self { y: 0, x: 0 }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct MazeState {
    points: Vec<Vec<ScoreType>>,
    turn: usize,
    character: Coord,
    game_score: ScoreType,
    evaluated_score: ScoreType,
    first_action: Option<usize>,
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
            first_action: None,
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

    #[allow(dead_code)]
    fn random_action(&self) -> usize {
        let legal_actions = self.legal_actions();
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        legal_actions[rng.gen_range(0..legal_actions.len())]
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    fn greedy_action(&self) -> usize {
        let legal_actions = self.legal_actions();
        let mut best_score = -INF;
        let mut best_action = None;
        for action in legal_actions {
            let mut now_state = self.clone();
            now_state.advance(action);
            now_state.evaluate_score();
            if now_state.evaluated_score > best_score {
                best_score = now_state.evaluated_score;
                best_action = Some(action);
            }
        }
        best_action.unwrap()
    }

    fn beam_search_action(&self, beam_width: usize, beam_depth: usize) -> usize {
        let mut now_beam = BinaryHeap::new();
        let mut best_state = None;

        now_beam.push(self.clone());
        for t in 0..beam_depth {
            let mut next_beam = BinaryHeap::new();
            for _ in 0..beam_width {
                if let Some(now_state) = now_beam.pop() {
                    let legal_actions = now_state.legal_actions();
                    for action in legal_actions {
                        let mut next_state = now_state.clone();
                        next_state.advance(action);
                        next_state.evaluate_score();
                        if t == 0 {
                            next_state.first_action = Some(action);
                        }
                        next_beam.push(next_state);
                    }
                }
            }

            now_beam = next_beam;
            best_state = now_beam.peek();

            if best_state.unwrap().is_done() {
                break;
            }
        }
        best_state.unwrap().first_action.unwrap()
    }
}

impl PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MazeState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
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

#[allow(dead_code)]
fn play_game(seed: u64) {
    let mut state = MazeState::from_seed(seed);
    while !state.is_done() {
        // state.advance(state.random_action()); // ランダム行動
        state.advance(state.greedy_action()); // 貪欲法
        println!("{}", state);
    }
}

fn test_ai_score(game_number: usize) -> f64 {
    let mut total_score = 0;
    for i in 0..game_number {
        let mut state = MazeState::from_seed(i as u64);
        while !state.is_done() {
            // state.advance(state.random_action()); // ランダム行動
            // state.advance(state.greedy_action()); // 貪欲法
            state.advance(state.beam_search_action(2, END_TURN)); // ビームサーチ
        }
        total_score += state.game_score;
    }
    total_score as f64 / game_number as f64
}

fn main() {
    // play_game(121321);
    let score = test_ai_score(100);
    println!("{}", score);
}
