package navigation

import "testing"

func TestTree_Insert(t *testing.T) {
	tree := Tree{
		Root: &Node{
			Path:     "root",
			Children: make([]*Node, 0),
		},
	}

	node := &Node{
		Path:      "root/child1.md",
		Title:     "Child 1",
		Position:  1,
		Children:  make([]*Node, 0),
		URL:       "/root/child1",
		HasConent: true,
	}

	tree.Insert(node)

	if len(tree.Root.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(tree.Root.Children))
	}

	insertedNode := tree.Root.Children[0]
	if insertedNode.Path != "child1.md" {
		t.Errorf("Expected inserted node path to be 'child1', got '%s'", insertedNode.Path)
	}
	if insertedNode.Title != "Child 1" {
		t.Errorf("Expected inserted node title to be 'Child 1', got '%s'", insertedNode.Title)
	}
	if insertedNode.Position != 1 {
		t.Errorf("Expected inserted node position to be 1, got %d", insertedNode.Position)
	}
	if insertedNode.URL != "pages/child1" {
		t.Errorf("Expected inserted node URL to be 'pages/child1', got '%s'", insertedNode.URL)
	}
	if !insertedNode.HasConent {
		t.Error("Expected inserted node to have content")
	}
}

func TestTree_InsertExistingNode(t *testing.T) {
	tree := Tree{
		Root: &Node{
			Path:     "root",
			Children: make([]*Node, 0),
		},
	}

	node := &Node{
		Path:     "root/child1",
		Title:    "Child 1",
		Position: 1,
		Children: make([]*Node, 0),
	}

	tree.Insert(node)
	tree.Insert(node)

	if len(tree.Root.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(tree.Root.Children))
	}
}

func TestTree_InsertNestedNode(t *testing.T) {
	tree := Tree{
		Root: &Node{
			Path:     "root",
			Children: make([]*Node, 0),
		},
	}

	node := &Node{
		Path:     "root/child1/grandchild1",
		Title:    "Grandchild 1",
		Position: 1,
		Children: make([]*Node, 0),
		URL:      "/root/child1/grandchild1",
	}

	tree.Insert(node)

	if len(tree.Root.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(tree.Root.Children))
	}

	// Verify the properties of the inserted node
	childNode := tree.Root.Children[0]
	if len(childNode.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(childNode.Children))
	}

	// Verify the properties of the inserted grandchild node
	grandchildNode := childNode.Children[0]
	if grandchildNode.Path != "grandchild1" {
		t.Errorf("Expected inserted grandchild node path to be 'grandchild1', got '%s'", grandchildNode.Path)
	}
	if grandchildNode.Title != "Grandchild 1" {
		t.Errorf("Expected inserted grandchild node title to be 'Grandchild 1', got '%s'", grandchildNode.Title)
	}
	if grandchildNode.Position != 1 {
		t.Errorf("Expected inserted grandchild node position to be 1, got %d", grandchildNode.Position)
	}
	if grandchildNode.URL != "pages/child1/grandchild1" {
		t.Errorf("Expected inserted grandchild node URL to be 'pages/child1/grandchild1', got '%s'", grandchildNode.URL)
	}
}

func TestTree_InsertOnExistingNode(t *testing.T) {
	tree := Tree{
		Root: &Node{
			Path:     "root",
			Children: make([]*Node, 0),
		},
	}

	node := &Node{
		Path:     "root/child1/index.md",
		Title:    "Child 1",
		Position: 1,
		Children: make([]*Node, 0),
	}

	tree.Insert(node)
	if len(tree.Root.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(tree.Root.Children))
	}

	grandchildNode := &Node{
		Path:     "root/child1/grandchild1.md",
		Title:    "Grandchild 1",
		Position: 1,
		Children: make([]*Node, 0),
	}

	tree.Insert(grandchildNode)
	childNode := tree.Root.Children[0]
	if len(childNode.Children) != 2 {
		t.Errorf("Expected 2 child, got %d", len(childNode.Children))
	}

	// Verify the properties of the inserted grandchild node
	insertedGrandchildNode := childNode.Children[1]
	if insertedGrandchildNode.Path != "grandchild1.md" {
		t.Errorf("Expected inserted grandchild node path to be 'grandchild1', got '%s'", insertedGrandchildNode.Path)
	}
	if insertedGrandchildNode.Title != "Grandchild 1" {
		t.Errorf("Expected inserted grandchild node title to be 'Grandchild 1', got '%s'", insertedGrandchildNode.Title)
	}
	if insertedGrandchildNode.Position != 1 {
		t.Errorf("Expected inserted grandchild node position to be 1, got %d", insertedGrandchildNode.Position)
	}
	if insertedGrandchildNode.URL != "pages/child1/grandchild1" {
		t.Errorf("Expected inserted grandchild node URL to be 'pages/child1/grandchild1', got '%s'", insertedGrandchildNode.URL)
	}
}

func TestTree_PrepareUrl(t *testing.T) {
	tree := Tree{
		Root: &Node{
			Path:     "root",
			Children: make([]*Node, 0),
		},
	}

	url := tree.PrepareUrlFromPath("root/child1.md")
	if url != "pages/child1" {
		t.Errorf("Expected URL to be 'pages/child1', got '%s'", url)
	}
}
