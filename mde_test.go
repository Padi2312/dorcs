// package main

// import (
// 	"bytes"
// 	"fmt"
// 	"io"

// 	"github.com/gomarkdown/markdown"
// 	"github.com/gomarkdown/markdown/ast"
// 	"github.com/gomarkdown/markdown/html"
// 	"github.com/gomarkdown/markdown/parser"
// )

// // AlertNode represents a custom node in the AST for alert blocks
// type AlertNode struct {
// 	ast.Leaf
// 	Content string
// }

// // Custom block parser for alert syntax
// func parseAlertBlock(data []byte) (ast.Node, []byte, int) {
// 	if bytes.HasPrefix(data, []byte("[alert]{")) {
// 		endIdx := bytes.Index(data, []byte("}"))
// 		if endIdx > 0 {
// 			content := data[8:endIdx] // skip past '[alert]{' to the start of content
// 			return &AlertNode{Content: string(content)}, data[endIdx+1:], endIdx + 1
// 		}
// 	}
// 	return nil, nil, 0
// }

// // Custom renderer function for AlertNode
// func renderAlertNode(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
// 	if alertNode, ok := node.(*AlertNode); ok && entering {
// 		fmt.Fprintf(w, `<div class="alert">%s</div>`, alertNode.Content)
// 		return ast.GoToNext, true
// 	}
// 	return ast.GoToNext, false
// }

// func main() {
// 	// Sample markdown with custom alert syntax
// 	markdownInput := `# Header

// [alert]{This is an alert message!}

// More text follows.`

// 	// Create a markdown parser with custom block parsing for alerts
// 	extensions := parser.CommonExtensions | parser.Attributes // or any other needed extensions
// 	p := parser.NewWithExtensions(extensions)
// 	p.Opts.ParserHook = parseAlertBlock // Register custom parser hook

// 	// Parse the markdown into an AST
// 	doc := markdown.Parse([]byte(markdownInput), p)

// 	// Create a renderer with our custom rendering function
// 	opts := html.RendererOptions{
// 		RenderNodeHook: renderAlertNode, // Use our custom alert renderer
// 		Flags:          html.CommonFlags,
// 	}
// 	renderer := html.NewRenderer(opts)

//		// Render the AST to HTML
//		htmlOutput := markdown.Render(doc, renderer)
//		fmt.Println("Generated HTML:", string(htmlOutput))
//	}

package main

import (
	"bytes"
	"fmt"
	"io"

	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/ast"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
)

// AlertNode represents a custom node in the AST for alert blocks
type AlertNode struct {
	ast.Container
}

// NewAlertNode creates a new AlertNode
func NewAlertNode() *AlertNode {
	return &AlertNode{}
}

// parseAlertBlock looks for our custom alert blocks and parses internal Markdown
func parseAlertBlock(data []byte, p *parser.Parser) (ast.Node, []byte, int) {
	if bytes.HasPrefix(data, []byte("[alert]{")) {
		endIdx := bytes.Index(data, []byte("}"))
		if endIdx > -1 {
			content := data[8:endIdx] // Extract the content inside the {}
			innerParser := parser.NewWithExtensions(parser.CommonExtensions)
			innerDoc := innerParser.Parse(content)
			node := NewAlertNode()
			ast.AppendChild(node, innerDoc)
			return node, data[endIdx+1:], endIdx + 1
		}
	}
	return nil, nil, 0
}

// renderAlertNode customizes rendering by handling AlertNode
func renderAlertNode(w io.Writer, node ast.Node, entering bool) (ast.WalkStatus, bool) {
	if _, ok := node.(*AlertNode); ok {
		if entering {
			io.WriteString(w, "<div class=\"alert\">")
		} else {
			io.WriteString(w, "</div>")
		}
		return ast.GoToNext, true
	}
	return ast.GoToNext, false
}

func main_mde() {
	// Sample markdown with custom alert syntax including nested Markdown
	markdownInput := `# Header

[alert]{
# Alert Heading
This is **strong** text with ` + "`code`" + ` snippet.
}

More text follows.`

	// Create a markdown parser with our custom parsing function
	p := parser.NewWithExtensions(parser.CommonExtensions)
	p.Opts.ParserHook = func(data []byte) (ast.Node, []byte, int) {
		return parseAlertBlock(data, p)
	}

	// Parse the markdown into an AST
	doc := markdown.Parse([]byte(markdownInput), p)

	// Create an HTML renderer with our custom rendering function
	opts := html.RendererOptions{
		RenderNodeHook: renderAlertNode,
	}
	renderer := html.NewRenderer(opts)

	// Render the AST to HTML
	htmlOutput := markdown.Render(doc, renderer)
	fmt.Println("Generated HTML:", string(htmlOutput))
}
