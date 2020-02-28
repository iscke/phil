
/**
 * Blades in the dark dice -
 * Roll n d6 taking highest
 * If n is 0, roll 2 and take lowest
 * If n isn't 0 and you roll two 6s, it's a crit
 */

use crate::common::{get_rand, join_rolls};

pub fn try_roll(dice: &String) -> Option<String> {
    match dice.trim().parse::<u8>() {
        Ok(count) if (count <= 4) => Some(roll_blades(count)),
        _ => None,
    }
}

fn roll_blades(count: u8) -> String {
    let mut num = count;
    if num == 0 {
        num = 2
    }

    let dice = get_rand(6, num as i32);
    match count {
        0 => format!("2d6 taking lowest - {}, {}\n~> **{}**", dice[0], dice[1], dice.iter().min().unwrap()),
        1 => format!("1d6 - **{}**", dice[0]),
        _ => {
            let rolls_str = join_rolls(&dice);

            let mut highest: [i32; 2] = [0, 0];
            for roll in dice {
                if roll > highest[0] {
                    highest[1] = highest[0];
                    highest[0] = roll;
                } else if roll > highest[1] {
                    highest[1] = roll;
                }
            }
            let crit = if highest[0] == 6 && highest[1] == 6 {
                " - crit!"
            } else {""};

            format!("{}d6 taking highest - {}\n~> **{}**{}", count, rolls_str, highest[0], crit)
        }
    }
}
