package dorcs

import (
	"log/slog"
	"os"
	"strings"

	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
	customelements "github.com/padi2312/dorcs/pkg/dorcs/custom_elements"
)

type DorcsFile struct {
	Path     string
	MetaData *MetaData
	Content  string
}

func NewDorcsFile(path string) *DorcsFile {
	var raw_content, err = os.ReadFile(path)
	if err != nil {
		slog.Error(err.Error())
	}

	var content = string(raw_content)

	meta_data := ParseMetadata(content)
	content = RemoveMetadata(content)

	content = strings.TrimSpace(content)

	return &DorcsFile{
		Path:     path,
		Content:  content,
		MetaData: meta_data,
	}
}

func (f *DorcsFile) ToHtml() string {
	extensions := parser.CommonExtensions |
		parser.NoIntraEmphasis |
		parser.Tables |
		parser.FencedCode |
		parser.Autolink |
		parser.Strikethrough |
		parser.SpaceHeadings |
		parser.BackslashLineBreak |
		parser.DefinitionLists |
		parser.NoEmptyLineBeforeBlock |
		parser.Attributes |
		parser.HeadingIDs |
		parser.OrderedListStart

	p := parser.NewWithExtensions(extensions)
	// add custom elements
	p.Opts.ParserHook = customelements.ParseAlertBlock

	doc := markdown.Parse([]byte(f.Content), p)
	opts := html.RendererOptions{
		Flags:          html.CommonFlags,
		RenderNodeHook: customelements.RenderAlertNode,
	}
	renderer := html.NewRenderer(opts)
	htmlOutput := markdown.Render(doc, renderer)
	return string(htmlOutput)

	// htmlFlags := html.CommonFlags | html.HrefTargetBlank | html.CompletePage
	// opts := html.RendererOptions{Flags: htmlFlags}
	// renderer := html.NewRenderer(opts)

	// md := []byte(f.Content)
	// return string(markdown.ToHTML(md, p, renderer))
}

func (f *DorcsFile) HasContent() bool {
	return f.Content != ""
}
