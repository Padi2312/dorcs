package customelements

import (
	"bytes"
	"fmt"
	"io"

	"github.com/gomarkdown/markdown/ast"
)

// AlertNode represents a custom node in the AST for alert blocks
type AlertNode struct {
	ast.Leaf
	Content string
}

// Custom block parser for alert syntax
func ParseAlertBlock(data []byte) (ast.Node, []byte, int) {
	if bytes.HasPrefix(data, []byte("[alert]{")) {
		endIdx := bytes.Index(data, []byte("}"))
		if endIdx > 0 {
			content := data[8:endIdx] // skip past '[alert]{' to the start of content
			return &AlertNode{Content: string(content)}, data[endIdx+1:], endIdx + 1
		}
	}
	return nil, nil, 0
}

// Custom renderer function for AlertNode
func RenderAlertNode(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
	if alertNode, ok := node.(*AlertNode); ok && entering {
		fmt.Fprintf(w, `<div class="alert">%s</div>`, alertNode.Content)
		return ast.GoToNext, true
	}
	return ast.GoToNext, false
}
