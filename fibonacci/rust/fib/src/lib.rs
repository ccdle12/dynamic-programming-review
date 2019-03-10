fn fib(num: i32) -> i32 {
    let mut cache = vec![];
    for i in 0..num + 1 {
        cache.push(-1);
    }

    private_fib(num, &mut cache)
}

fn private_fib(num: i32, cache: &mut Vec<i32>) -> i32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    let result = private_fib(num - 1, cache) + private_fib(num - 2, cache);
    cache.insert(num as usize, result);

    cache[num as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        let res = fib(0);
        assert_eq!(res, 0);

        let res = fib(1);
        assert_eq!(res, 1);
    }

    #[test]
    fn using_a_cache() {
        let res = fib(2);
        assert_eq!(res, 1);

        let res = fib(4);
        assert_eq!(res, 3);

        let res = fib(5);
        assert_eq!(res, 5);

        let res = fib(8);
        assert_eq!(res, 21);
    }
}
