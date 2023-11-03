use rand::Rng;

/// pick the single or highest first
///
/// this strategy consists of picking the highest number available
/// the highest is either the single one, or the one paired with the lowest
/// it's then easy to pick since the way the choices are built, we just have to pick the first
pub fn highest_first(_: &Vec<(u8,u8)>) -> usize {
    return 0;
}

/// Random strategy
///
/// pick a random choice each time
pub fn random(choices: &Vec<(u8,u8)>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..choices.len())
}

/// Pick the double first with the lowest digit
///
/// this strategy picks a double first but with the lowest in it
/// The way the choices are built makes it easy since we have to pick the last choice
pub fn double_lowest(choices: &Vec<(u8,u8)>) -> usize {
    return choices.len() -1;
}

/// Pick the double first with the highest digit
///
/// this strategy picks a double first but with the highest in it
/// The way the choices are built is either to pick the first if it's a double or the second
/// if there is other choice than the single
pub fn double_highest(choices: &Vec<(u8,u8)>) -> usize {
    if choices.len() == 1 {
        0
    }else{
        let (a, _) = choices[0];
        if a > 0 {
            0
        }else{
            1
        }
    }
}

/// Pick the highest even first
///
/// This strategy consists in picking the highest even number first
pub fn even_highest(choices: &Vec<(u8,u8)>) -> usize{
    let mut pos_for_even_in_low = 0;
    for i in 0..choices.len(){
        let (a,b) = choices[i];
        if b%2 == 0 {
            return i;
        }else if a%2==0{
            pos_for_even_in_low = i;
        }
    }
    return pos_for_even_in_low;
}

/// Pick the highest odd first
///
/// This strategy consists in picking the highest odd number first
pub fn odd_highest(choices: &Vec<(u8,u8)>) -> usize{
    let mut pos_for_odd_in_low = 0;
    for i in 0..choices.len(){
        let (a,b) = choices[i];
        if b%2 == 1 {
            return i;
        }else if a%2==1{
            pos_for_odd_in_low = i;
        }
    }
    return pos_for_odd_in_low;
}
