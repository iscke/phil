
use regex::Regex;
use crate::common::{get_rand, join_rolls};

enum Advantage {
    Advantage,
    Disadvantage,
}

pub fn try_roll(dice: &String) -> Option<String> {
    lazy_static! {
        static ref DICE: Regex = Regex::new(r"^([+-])?(\d+)d(\d+)$").unwrap();
    }
    if let Some(cap) = DICE.captures(dice.trim()) {
        let adv: Option<Advantage> = cap.get(1).map(|c| if c.as_str() == "+" {Advantage::Advantage} else {Advantage::Disadvantage});
        let count: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let sides: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        Some(roll_dice(adv, count, sides))
    } else {
        None
    }
}

fn roll_dice(adv: Option<Advantage>, count: i32, sides: i32) -> String {
    let dice = get_rand(sides, count);
    let rolls_str = join_rolls(&dice);

    match adv {
        None => {
            format!("{}d{} - {}\n~> **{}**", count, sides, rolls_str, dice.iter().sum::<i32>())
        },
        Some(Advantage::Advantage) => {
            format!("{}d{} taking highest - {}\n~> **{}**", count, sides, rolls_str, dice.iter().max().unwrap())
        },
        Some(Advantage::Disadvantage) => {
            format!("{}d{} taking lowest - {}\n~> **{}**", count, sides, rolls_str, dice.iter().min().unwrap())
        },
    }
}
