package dorcs

import (
	"embed"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/padi2312/dorcs/internal"
	"github.com/padi2312/dorcs/pkg/navigation"
	"github.com/padi2312/dorcs/pkg/utils"
)

type Dorcs struct {
	Config            *internal.Config
	Tree              *navigation.Tree
	clientAssets      embed.FS
	LatestModifiedUrl *string
}

func NewDorcs(clientAssets embed.FS) *Dorcs {
	config := internal.NewConfig()
	rootDir := *config.Source
	tree := navigation.NewTree(rootDir)
	return &Dorcs{
		Config:       internal.NewConfig(),
		Tree:         tree,
		clientAssets: clientAssets,
	}
}

func (d *Dorcs) SetWatchMode() {
	*d.Config.PageSettings.Dev = true
}

func (d *Dorcs) Build() {
	start := time.Now()
	ph := NewPathHandler(*d.Config.Source)
	markdownFiles := ph.GetMarkdownFiles()
	assetFiles := ph.GetAssetFiles()

	errMdFile := d.processMarkdownFiles(markdownFiles)
	if errMdFile != nil {
		fmt.Println("Error processing markdown files:", errMdFile)
		return
	}

	errAssetFile := d.copyAssets(assetFiles)
	if errAssetFile != nil {
		slog.Error("Error copying asset files:", errAssetFile)
		return
	}

	errRoutesFile := d.writeRoutesFile()
	if errRoutesFile != nil {
		slog.Error("Error writing routes file:", errRoutesFile)
		return
	}

	errPageSettings := d.writePageSeetings()
	if errPageSettings != nil {
		slog.Error("Error writing page settings file:", errPageSettings)
		return
	}

	errClientAssets := d.writeClientAssets()
	if errClientAssets != nil {
		slog.Error("Error writing client assets:", errClientAssets)
		return
	}
	duration := time.Since(start)
	slog.Info("✅ Build complete in " + duration.Round(time.Millisecond).String())
}

func (d *Dorcs) processMarkdownFiles(files []string) error {
	for _, file := range files {
		err := d.ProcessFile(file)
		if err != nil {
			return err
		}
	}
	return nil
}

func (d *Dorcs) ProcessFile(file string) error {
	df := NewDorcsFile(file)
	var title = df.MetaData.Title
	// In case title is not provided, use the file name as title
	if title == "" {
		// Get only file name without extension
		title = strings.TrimSuffix(filepath.Base(file), filepath.Ext(file))
		df.MetaData.Title = title
	}
	n := &navigation.Node{
		Path:      df.Path,
		Title:     df.MetaData.Title,
		URL:       df.Path,
		Position:  df.MetaData.Position,
		HasConent: df.HasContent(),
	}
	d.Tree.AddNode(n)

	err := d.writeFileToOutput(df)
	if err != nil {
		return err
	}

	latestUrl := d.Tree.PrepareUrlFromPath(df.Path)
	d.LatestModifiedUrl = &latestUrl
	return nil
}

func (d *Dorcs) copyAssets(files []string) error {
	outputDir := *d.Config.Output
	for _, file := range files {
		var filePath = d.removeSourceDirFromPath(file)
		outputPath := filepath.Join(outputDir, filePath)

		// read file content of the asset file
		var fileContent, err = os.ReadFile(file)
		if err != nil {
			return err
		}

		err = os.WriteFile(outputPath, fileContent, 0644)
		if err != nil {
			return err
		}
	}
	return nil
}

func (d *Dorcs) writeFileToOutput(file *DorcsFile) error {
	var path = file.Path
	path = d.removeSourceDirFromPath(path)
	// Add pages to differentiating between docs and client html file
	var outputPath = strings.TrimSuffix(path, filepath.Ext(path)) + ".html"
	outputPath = filepath.Join("pages/", outputPath)
	outputPath = filepath.Join(*d.Config.Output, outputPath)

	outputDir := filepath.Dir(outputPath)
	var err = os.MkdirAll(outputDir, os.ModePerm)
	if err != nil {
		return err
	}

	fileContent := file.ToHtml()
	err = os.WriteFile(outputPath, []byte(fileContent), 0644)
	if err != nil {
		return err
	}

	return nil
}

func (d *Dorcs) writeRoutesFile() error {
	var jsonTree, err = json.Marshal(d.Tree.Root.Children)
	if err != nil {
		return err
	}

	outputPath := filepath.Join(*d.Config.Output, "routes.json")
	err = os.WriteFile(outputPath, jsonTree, 0644)
	if err != nil {
		return err
	}

	return nil
}

func (d *Dorcs) writePageSeetings() error {
	var jsonPageSettings, err = json.Marshal(d.Config.PageSettings)
	if err != nil {
		return err
	}

	outputPath := filepath.Join(*d.Config.Output, "page_settings.json")
	err = os.WriteFile(outputPath, jsonPageSettings, 0644)
	if err != nil {
		return err
	}

	return nil
}

func (d *Dorcs) writeClientAssets() error {
	return utils.CopyDir(d.clientAssets, "frontend/dist", *d.Config.Output)
}

func (d *Dorcs) removeSourceDirFromPath(path string) string {
	return strings.Replace(path, *d.Config.Source, "", 1)
}
