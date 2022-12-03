package main

import (
	"fmt"
	"os"
	"strings"
)

const AsciLetters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

type Backpack struct {
	compartmentOne string
	compartmentTwo string
}

type Group struct {
	backpackOne   string
	backpackTwo   string
	backpackThree string
}

func (backpack Backpack) getRepeatingItems() []string {
	var repeatingItems []string

	for _, itemOfCompartmentOne := range backpack.compartmentOne {
		for _, itemOfCompartmentTwo := range backpack.compartmentTwo {
			if string(itemOfCompartmentOne) == string(itemOfCompartmentTwo) {
				repeatingItems = append(repeatingItems, string(itemOfCompartmentOne))
			}
		}
	}

	return repeatingItems
}

func (group Group) getRepeatingItem() string {
	var repeatingItems []string

	for _, itemOfBackpackOne := range group.backpackOne {
		itemString := string(itemOfBackpackOne)

		indexOfItemInBackpackTwo := strings.Index(group.backpackTwo, itemString)
		indexOfItemInBackpackThree := strings.Index(group.backpackThree, itemString)

		if indexOfItemInBackpackTwo != -1 && indexOfItemInBackpackThree != -1 {
			repeatingItems = append(repeatingItems, itemString)
		}
	}

	return repeatingItems[0]
}

func calculatePriority(items []string) int {
	var priorities []int

	for _, item := range items {
		priorities = append(priorities, strings.Index(AsciLetters, item)+1)
	}

	return sum(priorities)
}

func main() {
	data, err := os.ReadFile("./data/input.txt")
	check(err)

	rawBackpacks := strings.Split(string(data), "\n")
	var repeatingItems []string
	var backpacks []Backpack

	for _, rawBackpack := range rawBackpacks {
		numberOfItems := len(rawBackpack)
		if numberOfItems == 0 {
			continue
		}

		firstCompartment := rawBackpack[0 : numberOfItems/2]
		secondCompartment := rawBackpack[numberOfItems/2 : numberOfItems]

		backpack := Backpack{firstCompartment, secondCompartment}

		repeatingItem := backpack.getRepeatingItems()
		repeatingItems = append(repeatingItems, repeatingItem[0])

		backpacks = append(backpacks, backpack)
	}

	var rawGroups [][]string
	var row []string

	for index, rawBackpack := range rawBackpacks {
		row = append(row, rawBackpack)
		if (index+1)%3 == 0 {
			rawGroups = append(rawGroups, row)
			row = nil
		}
	}

	var groups []Group

	for _, rawGroup := range rawGroups {
		group := Group{backpackOne: rawGroup[0], backpackTwo: rawGroup[1], backpackThree: rawGroup[2]}
		groups = append(groups, group)
	}

	var repeatingItemsInGrups []string

	for _, group := range groups {
		repeatingItem := group.getRepeatingItem()
		repeatingItemsInGrups = append(repeatingItemsInGrups, repeatingItem)
	}

	taskOneResult := calculatePriority(repeatingItems)
	fmt.Println(taskOneResult)

	taskTwoResult := calculatePriority(repeatingItemsInGrups)
	fmt.Println(taskTwoResult)
}
