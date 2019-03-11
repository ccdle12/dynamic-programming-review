// "FAST" - Strategy
// F - First Solution.
// A - Analyze Solution.
// S - Find the subproblems.
// T - Turn the solution around.

fn fib(num: i32) -> i32 {
    let mut cache = vec![];
    for _i in 0..num + 1 {
        cache.push(-1);
    }

    private_fib(num, &mut cache)
}

// 1. Start withe a brute force naive solution. "F"
// 2. Add a cache to be returned, this ensures we do not continue to make duplicate recursive
//    calls. "A"
// 3. Brute Force - Space Complexity: O(2^n), the recursive tree has n levels and each level calls
//    two recursive functions. "S"
fn private_fib(num: i32, cache: &mut Vec<i32>) -> i32 {
    if num < 2 {
        return num;
    }

    // Check if the result already exists, return if so.
    if cache[num as usize] != -1 {
        return cache[num as usize];
    }

    let result = private_fib(num - 1, cache) + private_fib(num - 2, cache);
    cache.insert(num as usize, result);

    cache[num as usize]
}

// 4. Approach the problem from the bottom up, take the recursive solution and use an interatrive
//    approach. - "T"
fn bottom_up(num: i32) -> i32 {
    // Deal with the base cases first.
    // Create a cache that catches 1-3.
    let mut cache = vec![0, 1, 1];

    // Fill the cache up to num.
    // Iteratively fill the cache until we have fib(num).
    for i in 2..(num + 1) {
        let index = i as usize;
        let result = cache[index - 1] + cache[index - 2];
        cache.insert(index, result);
    }

    cache[num as usize]
}

// We only need the last two numbers.
fn bottom_up_improved(num: i32) -> i32 {
    // Cover the first 3 base cases.
    if num < 2 {
        return num;
    };

    let mut n1 = 1;
    let mut n2 = 0;

    for i in 2..num {
        let current = n1 + n2;

        // Shift.
        n2 = n1;
        n1 = current;
    }

    n1 + n2
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

    #[test]
    fn large_number_range() {
        let expected = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811,
        ];

        for i in 0..expected.len() {
            let result = fib(i as i32);
            assert_eq!(expected[i], result);

            // Testing the bottom-up approach.
            let result = bottom_up(i as i32);
            assert_eq!(expected[i], result);

            // Testing improved bottom-up approach.
            let result = bottom_up_improved(i as i32);
            assert_eq!(expected[i], result)
        }
    }
}
