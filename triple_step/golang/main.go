package main

func main() {}

// TripleStep is the open function for running a recursive function to find all
// the possible combinations of "skipping steps" to reach the target step.
func TripleStep(target int16) int16 {
	steps := []int16{2, 3, 4}

	// Initialise a cache with the length of target.
	cache := []int16{}
	for i := int16(0); i < target+1; i++ {
		if i == 1 {
			cache = append(cache, 1)
			continue
		}

		cache = append(cache, 0)
	}

	return tripleStep(cache, steps, target)
}

func tripleStep(cache []int16, steps []int16, target int16) int16 {
	if target < 0 {
		return cache[0]
	}

	if target <= 2 {
		return cache[target]
	}

	if cache[target] != 0 {
		return cache[target]
	}

	for _, step := range steps {
		newTarget := target - step

		current := tripleStep(cache, steps, newTarget)
		cache[target] = cache[target] + current
	}

	return cache[target]
}
