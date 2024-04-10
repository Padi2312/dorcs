package dorcs

import (
	"log/slog"
	"os"
	"path/filepath"
)

type PathHandler struct {
	paths []string
}

func NewPathHandler(src_dir string) *PathHandler {
	// Read all files from source directory
	// and store them in the paths slice
	var paths []string
	filepath.Walk(src_dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			slog.Error(err.Error())
		}

		if !info.IsDir() {
			paths = append(paths, path)
		}
		return nil
	})

	return &PathHandler{
		paths: paths,
	}
}

func (f *PathHandler) GetMarkdownFiles() []string {
	var markdownPaths []string
	for _, path := range f.paths {
		if filepath.Ext(path) == ".md" {
			markdownPaths = append(markdownPaths, path)
		}
	}
	return markdownPaths
}

func (f *PathHandler) GetAssetFiles() []string {
	var assetPaths []string
	for _, path := range f.paths {
		if filepath.Ext(path) != ".md" {
			assetPaths = append(assetPaths, path)
		}
	}
	return assetPaths
}
