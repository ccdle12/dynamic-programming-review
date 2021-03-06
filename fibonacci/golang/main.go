package main

func main() {}

func Fib(num int32) int32 {
	// Create cache for internal fib.
	cache := []int32{}
	for i := int32(0); i <= num; i++ {
		cache = append(cache, -1)
	}

	return fib(num, cache)
}

func fib(num int32, cache []int32) int32 {
	// Catch the base case 0 and 1.
	if num < 2 {
		return num
	}

	// Check if the num value already exists in cache.
	if cache[num] != -1 {
		return cache[num]
	}

	result := fib(num-1, cache) + fib(num-2, cache)
	cache[num] = result

	return cache[num]
}

// We only need the (n-1, n-2) to compute fib(n). We can reduce the space
// complexity of O(n) -> O(1).
func BottomUp(num int32) int32 {
	// Initialise with the first 3 base cases.
	cache := []int32{0, 1, 1}

	for i := int32(3); i <= num; i++ {
		result := cache[i-1] + cache[i-2]
		cache = append(cache, result)
	}

	return cache[num]
}

func ImprovedBottomUp(num int32) int32 {
	// Check and return the base cases.
	if num < 2 {
		return num
	}

	var n1 int32 = 1
	var n2 int32

	for i := int32(3); i <= num; i++ {
		result := n1 + n2

		n2 = n1
		n1 = result
	}

	return n1 + n2
}
