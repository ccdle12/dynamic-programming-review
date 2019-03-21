package main

func main() {}

func TripleStep(target int16) int16 {
	steps := []int16{2, 3, 4}

	return tripleStep(steps, target)
}

func tripleStep(steps []int16, target int16) int16 {
	if target == 1 {
		return 1
	}

	if target <= 2 {
		return 0
	}

	var result int16
	for _, step := range steps {
		newTarget := target - step
		current := tripleStep(steps, newTarget)
		result = result + current
	}

	return result
}
