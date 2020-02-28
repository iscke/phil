
use rand::Rng;

pub fn get_rand(sides: i32, count: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(count as usize);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let roll = rng.gen_range(1, sides + 1);
        res.push(roll);
    }
    res
}
pub fn join_rolls(rolls: &Vec<i32>) -> String {
    rolls.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}