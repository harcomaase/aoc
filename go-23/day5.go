package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Relation struct {
	destCategory string
	rel          map[int]int
}

func main() {
	raw_input, err := os.ReadFile("../input/23/day5.txt")
	if err != nil {
		panic(err)
	}
	input := string(raw_input)

	splits := strings.Split(input, "\n\n")

	// first paragraph contains seeds
	seeds_input := strings.Split(strings.TrimSpace(splits[0]), " ")[1:]
	seeds := make([]int, 0)
	for _, v := range seeds_input {
		i, err := strconv.Atoi(v)
		if err != nil {
			panic(err)
		}
		seeds = append(seeds, i)
	}

	relations := make(map[string]Relation)

	// read relations
	for _, paragraph := range splits[1:] {
		lines := strings.Split(paragraph, "\n")
		relationLine := lines[0]
		firstDashIndex := strings.Index(relationLine, "-")
		rel1 := relationLine[:firstDashIndex]
		whitespaceIndex := strings.Index(relationLine, " ")
		rel2 := relationLine[firstDashIndex+4 : whitespaceIndex]

		relation := make(map[int]int)
		for _, line := range lines[1:] {
			if len(line) == 0 {
				continue
			}
			parts := strings.Split(line, " ")
			dest, err := strconv.Atoi(parts[0])
			if err != nil {
				panic(err)
			}
			src, err := strconv.Atoi(parts[1])
			if err != nil {
				panic(err)
			}
			offset, err := strconv.Atoi(parts[2])
			if err != nil {
				panic(err)
			}

			for i := 0; i < offset; i += 1 {
				relation[src+i] = dest + i
			}
		}
		category := rel1
		relations[category] = Relation{rel2, relation}
	}

	lowestLocation := -1
	for _, seed := range seeds {
		src := seed
		category := "seed"
		for category != "location" {
			relation := relations[category]
			dest, found := relation.rel[src]
			if !found {
				dest = src
			}
			src = dest
			category = relation.destCategory
		}

		if lowestLocation > src || lowestLocation == -1 {
			lowestLocation = src
		}
	}
	fmt.Printf("lowest location: %v\n", lowestLocation)

}
