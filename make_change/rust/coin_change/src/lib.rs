use std::i16::MAX;

#[allow(dead_code)]
pub fn min_change(target: i16, coins: Vec<i16>) -> i16 {
    let mut cache = vec![0];
    for _i in 1..target + 1 {
        cache.push(-1);
    }

    calc_change(target, &mut cache, &coins)
}

#[allow(dead_code)]
fn calc_change(target: i16, cache: &mut Vec<i16>, coins: &Vec<i16>) -> i16 {
    if cache[target as usize] >= 0 {
        return cache[target as usize];
    }

    let mut min_coins = MAX;

    for coin in coins {
        if target - coin >= 0 {
            println!(
                "Calling recursively, target: {}, coin: {}, new target: {}",
                target,
                coin,
                target - coin
            );
            let current_min = calc_change(target - coin, cache, coins);
            if current_min < min_coins {
                min_coins = current_min;
            }
        }
    }

    cache[target as usize] = min_coins + 1;
    cache[target as usize]
}

#[allow(dead_code)]
fn bottom_up(target: i16, cache: &mut Vec<i16>, coins: &Vec<i16>) -> i16 {
    for i in 1..(target + 1) {
        let mut min_coins = MAX;

        for coin in coins {
            println!("i-coins: i: {} coin: {}, result: {}", i, coin, i - coin);
            if i - coin >= 0 {
                let current_min = cache[(i - coin) as usize] + 1;
                println!(
                    "value at current_min index: {}, index: {}",
                    current_min,
                    i - coin
                );
                println!("current min: {}", current_min);
                println!("min coins: {}", min_coins);
                if current_min < min_coins {
                    min_coins = current_min;
                }
            }
            println!("index i: {}, assignin min_coins: {}", i, min_coins);
            cache[i as usize] = min_coins;
        }
    }

    println!("returned: {}", cache[target as usize]);
    cache[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        let change = min_change(1, vec![10, 6, 1]);

        assert_eq!(1, change);
    }

    #[test]
    fn simple_input_2() {
        let change = min_change(17, vec![10, 6, 1]);
        assert_eq!(3, change);
    }

    #[test]
    fn test_bottom_up() {
        let target = 17;

        let mut cache = vec![0];
        for _i in 1..target + 1 {
            cache.push(-1);
        }

        let change = bottom_up(target, &mut cache, &vec![10, 6, 1]);
        assert_eq!(3, change);
    }
}
