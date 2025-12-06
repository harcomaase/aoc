package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CategoryRelation struct {
	srcCategory  string
	destCategory string
	relations    []Relation
}

type Relation struct {
	srcStart  int
	destStart int
	offset    int
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

	categoryRelations := make([]CategoryRelation, 0)

	// read relations
	for _, paragraph := range splits[1:] {
		lines := strings.Split(paragraph, "\n")
		relationLine := lines[0]
		firstDashIndex := strings.Index(relationLine, "-")
		rel1 := relationLine[:firstDashIndex]
		whitespaceIndex := strings.Index(relationLine, " ")
		rel2 := relationLine[firstDashIndex+4 : whitespaceIndex]

		srcCategory := rel1
		destCategory := rel2
		relations := make([]Relation, 0)
		categoryRelation := CategoryRelation{srcCategory, destCategory, relations}
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

			relation := Relation{src, dest, offset}
			categoryRelation.relations = append(categoryRelation.relations, relation)
		}
		categoryRelations = append(categoryRelations, categoryRelation)
	}

	results := make([]chan int, 0)
	for i := 0; i < len(seeds_input); i += 2 {
		result := make(chan int, 1)
		index := i
		go func() {
			start, err := strconv.Atoi(seeds_input[index])
			if err != nil {
				panic(err)
			}
			offset, err := strconv.Atoi(seeds_input[index+1])
			if err != nil {
				panic(err)
			}
			lowestSubLocation := -1
			for j := 0; j < offset; j += 1 {
				seed := start + j
				location := followSeed(seed, &categoryRelations)
				if location < lowestSubLocation || lowestSubLocation == -1 {
					lowestSubLocation = location
				}
			}
			result <- lowestSubLocation
		}()

		results = append(results, result)
	}
	lowestLocation := -1
	for i := 0; i < len(results); i += 1 {
		result := results[i]
		location := <-result
		if location < lowestLocation || lowestLocation == -1 {
			lowestLocation = location
		}
	}
	fmt.Printf("%v\n", lowestLocation)

}

func followSeed(seed int, categoryRelations *[]CategoryRelation) int {
	src := seed
	srcCategory := "seed"
	for srcCategory != "location" {
		categoryRelation := findForSrcCategory(srcCategory, categoryRelations)

		// find target index
		// 1) has src different target?
		for _, relation := range categoryRelation.relations {
			if relation.srcStart <= src && src <= relation.srcStart+relation.offset {
				// we are within the range, now calculate the actual offset
				diff := relation.destStart - relation.srcStart
				src += diff
				break
			}
		}
		srcCategory = categoryRelation.destCategory
	}

	return src
}

func findForSrcCategory(srcCategory string, categoryRelations *[]CategoryRelation) CategoryRelation {
	for _, categoryRelation := range *categoryRelations {
		if categoryRelation.srcCategory == srcCategory {
			return categoryRelation
		}
	}
	panic("should not happen")
}
