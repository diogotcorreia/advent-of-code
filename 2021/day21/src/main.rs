#[derive(Debug)]
struct Player {
    pos: i32,
    score: i32,
}

impl Player {
    fn advance_pos(&mut self, pos: i32) {
        self.pos = ((self.pos + pos - 1) % 10) + 1
    }

    fn increase_score(&mut self, score: i32) {
        self.score += score;
    }
}

fn main() {
    let player1_start_pos = get_player_start_position_from_input();
    let player2_start_pos = get_player_start_position_from_input();

    println!(
        "Part 1: {}",
        play_with_deterministic_dice(player1_start_pos, player2_start_pos)
    );

    println!(
        "Part 2: {}",
        play_with_dirac_dice(player1_start_pos, player2_start_pos)
    );
}

fn get_player_start_position_from_input() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn play_with_deterministic_dice(player1_start_pos: i32, player2_start_pos: i32) -> i32 {
    let mut player1 = Player {
        pos: player1_start_pos,
        score: 0,
    };
    let mut player2 = Player {
        pos: player2_start_pos,
        score: 0,
    };
    let mut deterministic_dice = 0;

    let mut dice_rolled = 0;

    while player1.score < 1000 && player2.score < 1000 {
        let play = (0..3)
            .map(|_| roll_deterministic_dice(&mut deterministic_dice))
            .sum();
        dice_rolled += 3;

        player1.advance_pos(play);
        player1.increase_score(player1.pos);
        if player1.score >= 1000 {
            break;
        }
        let play = (0..3)
            .map(|_| roll_deterministic_dice(&mut deterministic_dice))
            .sum();
        dice_rolled += 3;

        player2.advance_pos(play);
        player2.increase_score(player2.pos);
    }

    player1.score.min(player2.score) * dice_rolled
}

fn roll_deterministic_dice(dice: &mut i32) -> i32 {
    *dice = (*dice % 100) + 1;
    *dice
}

fn play_with_dirac_dice(player1_start_pos: i32, player2_start_pos: i32) -> i128 {
    let mut caching_table = [[[[0 as i128; 22]; 22]; 10]; 10];
    let mut playing_now = false;

    let options_list = calculate_dice_options_list();

    caching_table[player1_start_pos as usize - 1][player2_start_pos as usize - 1][0][0] = 1;

    while !has_game_ended(&caching_table) {
        let mut new_caching_table = [[[[0 as i128; 22]; 22]; 10]; 10];

        for p1_pos in 0..10 {
            for p2_pos in 0..10 {
                for p1_score in 0..=21 {
                    for p2_score in 0..=21 {
                        if p1_score == 21 || p2_score == 21 {
                            new_caching_table[p1_pos][p2_pos][p1_score][p2_score] +=
                                caching_table[p1_pos][p2_pos][p1_score][p2_score];
                            continue;
                        }
                        for (dice, count) in options_list.iter().enumerate() {
                            if !playing_now {
                                let p1_new_pos = (p1_pos + dice + 1) % 10;
                                let p1_new_score = (p1_score + p1_new_pos + 1).min(21);
                                new_caching_table[p1_new_pos][p2_pos][p1_new_score][p2_score] +=
                                    caching_table[p1_pos][p2_pos][p1_score][p2_score]
                                        * *count as i128;
                            } else {
                                let p2_new_pos = (p2_pos + dice + 1) % 10;
                                let p2_new_score = (p2_score + p2_new_pos + 1).min(21);
                                new_caching_table[p1_pos][p2_new_pos][p1_score][p2_new_score] +=
                                    caching_table[p1_pos][p2_pos][p1_score][p2_score]
                                        * *count as i128;
                            }
                        }
                    }
                }
            }
        }

        playing_now = !playing_now;
        caching_table = new_caching_table;
    }

    let wins = sum_winning_universes(&caching_table);

    wins.0.max(wins.1)
}

fn calculate_dice_options_list() -> [i32; 9] {
    // there are 3 * 3 * 3 = 27 options for scores
    let mut list = [0; 9]; // max sum is 9=3+3+3

    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                list[a + b + c - 1] += 1;
            }
        }
    }
    list
}

fn has_game_ended(cache: &[[[[i128; 22]; 22]; 10]; 10]) -> bool {
    for p1_pos in 0..10 {
        for p2_pos in 0..10 {
            for p1_score in 0..=20 {
                for p2_score in 0..=20 {
                    if cache[p1_pos][p2_pos][p1_score][p2_score] != 0 {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn sum_winning_universes(cache: &[[[[i128; 22]; 22]; 10]; 10]) -> (i128, i128) {
    let (mut p1_wins, mut p2_wins) = (0, 0);

    for p1_pos in 0..10 {
        for p2_pos in 0..10 {
            for p1_score in 0..=21 {
                p2_wins += cache[p1_pos][p2_pos][p1_score][21];
            }
            for p2_score in 0..=21 {
                p1_wins += cache[p1_pos][p2_pos][21][p2_score];
            }
        }
    }

    (p1_wins, p2_wins)
}
