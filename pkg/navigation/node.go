package navigation

import "sort"

type Node struct {
	Path      string  `json:"path"`
	Title     string  `json:"title"`
	Position  int     `json:"position"`
	Children  []*Node `json:"children"`
	URL       string  `json:"url"`
	HasConent bool    `json:"-"`
}

func (n *Node) AddChild(child *Node) {
	n.Children = append(n.Children, child)
}

func (n *Node) GetChildren() []*Node {
	return n.Children
}

func (n *Node) FindChildByPath(path string) *Node {
	for _, child := range n.Children {
		if child.Path == path {
			return child
		}
	}
	return nil
}

func (n *Node) Sort() {
	if len(n.Children) == 0 {
		return
	}
	sort.Slice(n.Children, func(i, j int) bool {
		return n.Children[i].Position < n.Children[j].Position
	})

	for _, child := range n.Children {
		child.Sort()
	}
}
