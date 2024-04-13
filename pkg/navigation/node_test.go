package navigation

import "testing"

func TestNode_AddChild(t *testing.T) {
	var child = &Node{Path: "first"}
	node := Node{}
	node.AddChild(child)
	if len(node.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(node.Children))
	}
	child = &Node{Path: "first"}
	node.AddChild(child)
	if len(node.Children) != 2 {
		t.Errorf("Expected 2 child, got %d", len(node.Children))
	}
}

func TestNode_FindChildByPath(t *testing.T) {
	node := Node{}
	child := &Node{Path: "child"}
	node.AddChild(child)
	found := node.FindChildByPath("child")
	if found == nil {
		t.Error("Expected to find child")
	}
}

func TestNode_FindChildByPathNil(t *testing.T) {
	node := Node{}
	child := &Node{Path: "child"}
	node.AddChild(child)
	found := node.FindChildByPath("not-child")
	if found != nil {
		t.Error("Expected to not find child")
	}
}

func TestNode_Sort(t *testing.T) {
	node := Node{}
	child1 := &Node{Position: 2}
	child2 := &Node{Position: 1}
	node.AddChild(child1)
	node.AddChild(child2)
	node.Sort()
	if node.Children[0].Position != 1 {
		t.Errorf("Expected first child to have position 1, got %d", node.Children[0].Position)
	}
	if node.Children[1].Position != 2 {
		t.Errorf("Expected second child to have position 2, got %d", node.Children[1].Position)
	}
}
