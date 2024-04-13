package navigation

import (
	"path/filepath"
	"strings"
)

type Tree struct {
	Root *Node `json:"root"`
}

func NewTree(rootDir string) *Tree {
	return &Tree{
		Root: &Node{
			Path:     rootDir,
			Title:    rootDir,
			Position: 0,
			Children: make([]*Node, 0),
			URL:      rootDir,
		}, // Add a comma here
	}
}

func (t *Tree) FindNodeByPath(path string) *Node {
	components := t.getPathComponents(path)
	currentNode := t.Root
	for _, component := range components {
		componentStr := component
		child := currentNode.FindChildByPath(componentStr)
		if child != nil {
			currentNode = child
		} else {
			return nil
		}
	}
	return currentNode
}

func (t *Tree) AddNode(n *Node) {
	if t.isSectionIndexFile(n.Path) {
		t.insertSectionIndex(n)
		return
	}
	t.Insert(n)
}

func (t *Tree) Insert(n *Node) {
	components := t.getPathComponents(n.Path)
	currentNode := t.Root
	// Iterate over the components of the path
	for _, component := range components {
		componentStr := component

		// Skip the root path and the current node path
		if componentStr == t.Root.Path || currentNode.Path == componentStr {
			continue
		}

		child := currentNode.FindChildByPath(componentStr)
		if child != nil {
			currentNode = child
		} else {
			urlPath := t.PrepareUrlFromPath(n.Path)
			newNode := &Node{
				Path:      componentStr,
				Title:     n.Title,
				Position:  n.Position,
				Children:  make([]*Node, 0),
				URL:       urlPath,
				HasConent: n.HasConent,
			}
			currentNode.AddChild(newNode)
			currentNode = newNode
		}
	}
	t.Root.Sort()
}

func (t *Tree) insertSectionIndex(n *Node) {
	parentComponents := strings.Split(n.Path, string(filepath.Separator))
	firstParentFolder := parentComponents[len(parentComponents)-2]
	currentNode := t.Root
	for _, component := range parentComponents {
		componentStr := component

		if componentStr == t.Root.Path {
			continue
		}

		if currentNode.Path == firstParentFolder && componentStr == "index.md" {
			currentNode.Title = n.Title
			currentNode.Position = n.Position
			if n.HasConent {
				currentNode.URL = t.PrepareUrlFromPath(n.Path)
			} else {
				currentNode.URL = ""
			}
			return
		}

		maybeChild := currentNode.FindChildByPath(componentStr)
		if maybeChild != nil {
			currentNode = maybeChild
		}
	}
}

func (t *Tree) isSectionIndexFile(path string) bool {
	components := t.getPathComponents(path)
	if len(components) <= 2 {
		return false
	}
	fileName := components[len(components)-1]

	return fileName == "index.md"
}

func (t *Tree) getPathComponents(path string) []string {
	components := strings.Split(filepath.Clean(path), string(filepath.Separator))
	return components
}

func (t *Tree) PrepareUrlFromPath(path string) string {
	var urlPath string = path
	urlPath = strings.TrimPrefix(urlPath, t.Root.Path)
	urlPath = strings.TrimSuffix(urlPath, ".md")
	urlPath = strings.TrimPrefix(urlPath, "/")
	urlPath = strings.ReplaceAll(urlPath, string(filepath.Separator), "/")

	if !strings.HasPrefix(urlPath, "/") {
		urlPath = "/" + urlPath
	}
	urlPath = "pages" + urlPath
	return urlPath
}
