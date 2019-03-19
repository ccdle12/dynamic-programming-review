fn main() {}

fn triple_step(input: Vec<i16>) -> i16 {
    let steps = vec![2, 3, 4];
    inner_triple_step(&steps, *input.last().unwrap())
}

// TODO: (ccdle12)
// * Analyze space + time complexity
// * Add cache to improve time complexity
// * Turn upside down for iterative solution
fn inner_triple_step(steps: &Vec<i16>, target: i16) -> i16 {
    if target == 1 {
        return 1;
    }

    if target <= 2 {
        return 0;
    }

    let mut result = 0;
    for step in steps {
        let new_target = target - step;
        println!(
            "target: {}, step: {}, new_target: {}",
            target, step, new_target
        );
        let current = inner_triple_step(&steps, new_target);
        result += current;
    }

    println!("result: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input = vec![1, 2];
        assert_eq!(2 as usize, input.len());

        let result = triple_step(input);
        assert_eq!(0, result);
    }

    #[test]
    fn simple_input_1() {
        let input = vec![1, 2, 3];

        let result = triple_step(input);
        assert_eq!(1, result);
    }

    #[test]
    fn simple_input_2() {
        let input = vec![1, 2, 3, 4];

        let result = triple_step(input);
        assert_eq!(1, result);
    }

    #[test]
    fn simple_input_3() {
        let input = vec![1, 2, 3, 4, 5];

        let result = triple_step(input);
        assert_eq!(2, result);
    }

    #[test]
    fn simple_input_4() {
        let input = vec![1, 2, 3, 4, 5, 6];

        let result = triple_step(input);
        assert_eq!(2, result);
    }

    #[test]
    fn simple_input_5() {
        let input = vec![1, 2, 3, 4, 5, 6, 7];

        let result = triple_step(input);
        assert_eq!(4, result);
    }

    #[test]
    fn medium_input_1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let result = triple_step(input);
        assert_eq!(5, result);
    }

    #[test]
    fn medium_input_2() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        let result = triple_step(input);
        assert_eq!(24, result);
    }
}
