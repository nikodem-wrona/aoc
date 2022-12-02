package main

import (
	"fmt"
	"os"
	"strings"
)

func calculateShapeScore(playerMove string) int {
	if playerMove == "X" {
		return 1
	}

	if playerMove == "Y" {
		return 2
	}

	return 3
}

func calculateGameScoreForStrategyOne(opponentMove string, playerMove string) int {
	if playerMove == "X" && opponentMove == "A" {
		return 3
	}

	if playerMove == "Y" && opponentMove == "B" {
		return 3
	}

	if playerMove == "Z" && opponentMove == "C" {
		return 3
	}

	if playerMove == "X" && opponentMove == "C" {
		return 6
	}

	if playerMove == "Y" && opponentMove == "A" {
		return 6
	}

	if playerMove == "Z" && opponentMove == "B" {
		return 6
	}

	return 0
}

func calculateGameScoreForStrategyTwo(opponentMove string, playerMove string) int {
	if playerMove == "X" && opponentMove == "A" {
		return 0 + 3
	}

	if playerMove == "X" && opponentMove == "B" {
		return 0 + 1
	}

	if playerMove == "X" && opponentMove == "C" {
		return 0 + 2
	}

	if playerMove == "Y" && opponentMove == "A" {
		return 3 + 1
	}

	if playerMove == "Y" && opponentMove == "B" {
		return 3 + 2
	}

	if playerMove == "Y" && opponentMove == "C" {
		return 3 + 3
	}

	if playerMove == "Z" && opponentMove == "A" {
		return 6 + 2
	}

	if playerMove == "Z" && opponentMove == "B" {
		return 6 + 3
	}

	return 6 + 1
}

func main() {
	data, err := os.ReadFile("./data/input.txt")
	check(err)

	var gameScoresForStrategyOne []int
	var gameScoresForStrategyTwo []int

	for _, i := range strings.Split(string(data), "\n") {
		moves := strings.Split(i, " ")

		if len(moves) == 0 || len(moves) == 1 {
			continue
		}

		opponentMove := moves[0]
		playerMove := moves[1]

		shapeScore := calculateShapeScore(playerMove)
		gameScoreStrategyOne := calculateGameScoreForStrategyOne(opponentMove, playerMove)

		scoreStrategyOne := shapeScore + gameScoreStrategyOne
		gameScoresForStrategyOne = append(gameScoresForStrategyOne, scoreStrategyOne)

		gameScoreStrategyTwo := calculateGameScoreForStrategyTwo(opponentMove, playerMove)
		gameScoresForStrategyTwo = append(gameScoresForStrategyTwo, gameScoreStrategyTwo)
	}

	totalGameScoreForStrategyOne := sum(gameScoresForStrategyOne)
	fmt.Printf("Total score of all games for strategy 1 : %d\n", totalGameScoreForStrategyOne)

	totalGameScoreForStrategyTwo := sum(gameScoresForStrategyTwo)
	fmt.Printf("Total score of all games for strategy 2 : %d\n", totalGameScoreForStrategyTwo)
}
