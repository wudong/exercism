package tree

import (
	"sort"
)

func Build(records []Record) (*Node, error){
	//sort the records,
	sort.Slice(records, func(i, j int) bool {
		return records[i].ID < records[j].ID
	} )

	currentNodes := map[int]*Node{}

	for _, r := range records {
		if currentNodes[r.ID] == nil {
			var children = make([]*Node, 0)
			currentNodes[r.ID]= &Node{
				ID: r.ID,
				Children: children,
			}
			if r.ID!=r.Parent {
				currentNodes[r.Parent].Children = append(currentNodes[r.Parent].Children, currentNodes[r.ID])
			}
		}
	}
	return currentNodes[0], nil
}

type Record struct {
	ID, Parent int
}

type Node struct {
	ID int
	Children []*Node
}
