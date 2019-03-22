fn main() {}

fn factorial(target: u32) -> u32 {
    if target >= 1 {
        return target * factorial(target - 1);
    }

    1
}

fn iterative(target: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 2..target + 1 {
        result = result * i;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input_1() {
        let result = factorial(3);
        assert_eq!(result, 6)
    }

    #[test]
    fn simple_input_2() {
        let result = factorial(4);
        assert_eq!(result, 24);

        let result = iterative(4);
        assert_eq!(result, 24);
    }

    #[test]
    fn simple_input_3() {
        let result = factorial(8);
        assert_eq!(result, 40320);

        let result = iterative(8);
        assert_eq!(result, 40320);
    }
}
