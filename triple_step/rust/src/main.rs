fn main() {}

fn triple_step(target: i16) -> i16 {
    let steps = vec![2, 3, 4];

    let mut cache = vec![0, 1];
    for _i in 2..target + 1 {
        cache.push(0);
    }

    inner_triple_step(&mut cache, &steps, target)
}

// TODO: (ccdle12)
// * Analyze space + time complexity
// O(c^log n)? The greatest recursion depth will be subject to -2 each time.
// E.g.
//      log n = A target of 8 has a largest recursion depth of 4.
//              A target of 5 has a largest recursion depth of 2.
//
//      c = number of coins, number of branches at each level of the recusrion
//          tree.
//
// * Add cache to improve time complexity
//   Each level has the same number of possibilities for steps, we can return
//   this value after seeing the step instead of repeating the same steps of
//   recursion
// E.g. 5 has 2 possible steps -> {2, 2}, {4} = 1
//   If we encounter 5 again through the recursion stack, it can simply just
//   return 2, instead of peforming recursive calls.
// * Space complexity increases to O(n) -> n being the target as we need to
//   cache each result for each index (step).
//
// * Turn upside down for iterative solution
fn inner_triple_step(cache: &mut Vec<i16>, steps: &Vec<i16>, target: i16) -> i16 {
    if target == 1 {
        return cache[1];
    }

    if target <= 2 {
        return cache[0];
    }

    if cache[target as usize] != 0 {
        return cache[target as usize];
    }

    for step in steps {
        let new_target = target - step;
        let current = inner_triple_step(cache, steps, new_target);
        cache[target as usize] += current;
    }

    cache[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input = 2;

        let result = triple_step(input);
        assert_eq!(0, result);
    }

    #[test]
    fn simple_input_1() {
        let input = 3;

        let result = triple_step(input);
        assert_eq!(1, result);
    }

    #[test]
    fn simple_input_2() {
        let input = 4;

        let result = triple_step(input);
        assert_eq!(1, result);
    }

    #[test]
    fn simple_input_3() {
        let input = 5;

        let result = triple_step(input);
        assert_eq!(2, result);
    }

    #[test]
    fn simple_input_4() {
        let input = 6;

        let result = triple_step(input);
        assert_eq!(2, result);
    }

    #[test]
    fn simple_input_5() {
        let input = 7;

        let result = triple_step(input);
        assert_eq!(4, result);
    }

    #[test]
    fn medium_input_1() {
        let input = 8;

        let result = triple_step(input);
        assert_eq!(5, result);
    }

    #[test]
    fn medium_input_2() {
        let input = 12;

        let result = triple_step(input);
        assert_eq!(24, result);
    }
}
