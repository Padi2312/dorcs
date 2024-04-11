package dorcs

import (
	"log/slog"
	"os"
	"strings"

	"github.com/gomarkdown/markdown"
	"github.com/gomarkdown/markdown/html"
	"github.com/gomarkdown/markdown/parser"
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
	extensions := parser.CommonExtensions | parser.AutoHeadingIDs
	p := parser.NewWithExtensions(extensions)

	htmlFlags := html.CommonFlags | html.HrefTargetBlank
	opts := html.RendererOptions{Flags: htmlFlags}
	renderer := html.NewRenderer(opts)

	md := []byte(f.Content)
	return string(markdown.ToHTML(md, p, renderer))
}

func (f *DorcsFile) HasContent() bool {
	return f.Content != ""
}
