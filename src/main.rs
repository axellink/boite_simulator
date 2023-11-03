use debug_print::debug_println;

mod game;
mod strategies;

fn run_test(strategy : fn(&Vec<(u8,u8)>) -> usize, strategy_name : &str) {
    let mut sum_of_score: u32 = 0;
    let mut sum_of_perfect: u32 = 0;
    let number_of_game = 1000;

    println!("Testing {} strategy", strategy_name);

    for _ in 0..number_of_game{
        debug_println!("NEW GAME");
        let (score, perfect) = game::game(strategy);
        debug_println!("SCORED {} WITH {} PERFECTS", score, perfect);
        debug_println!("");
        sum_of_score += score as u32;
        sum_of_perfect += perfect as u32;
    }

    println!("Average score : {}", sum_of_score as f32/number_of_game as f32);
    println!("Average number of perfect : {}", sum_of_perfect as f32/number_of_game as f32);
}

fn main(){
    run_test(strategies::random, "random");
    println!("");
    run_test(strategies::highest_first, "highest first");
    println!("");
    run_test(strategies::double_lowest, "double lowest");
    println!("");
    run_test(strategies::double_highest, "double highest");
    println!("");
    run_test(strategies::even_highest, "even highest");
    println!("");
    run_test(strategies::odd_highest, "odd highest");
}
