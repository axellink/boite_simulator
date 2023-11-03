use debug_print::debug_println;
use rand::distributions::{Distribution,Uniform};

/// Plays two dices and return the choices allowed by game state
///
/// Launches two dices and calculate their sum
/// Then tries to find every combination possible to match this sum
/// given the game state and return them
///
fn gambling(game_state: &Vec<u8>)  -> Vec<(u8,u8)> {
    let mut rng = rand::thread_rng();
    let dice = Uniform::from(1..7);
    let dices: u8 = dice.sample(&mut rng) + dice.sample(&mut rng);
    let mut res : Vec<(u8,u8)> = vec!();

    debug_println!("Rolled {}", dices);

    if game_state.contains(&dices) {
        res.push((0,dices))
    }

    for i in 1..(dices/2) {
        let complement = dices - i;
        if game_state.contains(&i) && game_state.contains(&complement){
            res.push((i,complement));
        }
    }

    return res;
}

/// plays a round of Boite and returns the sum
///
/// do the gambling over and over until no choices are left
/// then sum the remaining numbers and returns it
///
fn round(strategy: fn(&Vec<(u8,u8)>) -> usize) -> u8 {
    let mut game_state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
    let mut choices = gambling(&game_state);
    while !choices.is_empty(){
        let (a,b) = choices[strategy(&choices)];
        debug_println!("Choices are {}, {}", a,b);
        game_state.retain(|x| x!=&a && x!=&b);
        debug_println!("New game state is {:?}", game_state);
        choices = gambling(&game_state);
    }
    return game_state.into_iter().fold(0,|acc,e| acc+e)
}

/// plays the full game for a player
///
/// plays until the maximum score of 100 or over is reached
/// and returns the number of round played and the number of perfect rounds
pub fn game(strategy: fn(&Vec<(u8,u8)>) -> usize) -> (u8,u8) {
    let mut num_of_rnd: u8 = 0;
    let mut num_of_perfect: u8 = 0;
    let mut score: u8 = 0;

    while score < 100{
        debug_println!("NEW ROUND !!");
        num_of_rnd += 1;
        let round_score = round(strategy);
        if round_score == 0 {
            num_of_perfect += 1;
        }
        score += round_score;
    }

    return (num_of_rnd,num_of_perfect);
}