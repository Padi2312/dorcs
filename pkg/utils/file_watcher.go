package utils

import (
	"io/fs"
	"log/slog"
	"os"
	"path/filepath"
	"time"

	"github.com/fsnotify/fsnotify"
)

type FileWatcher struct {
	SourceDir string
	// Map to store the last event time for each file to debounce events
	events           map[string]time.Time
	debounceDuration time.Duration
}

func NewFileWatcher(sourceDir string) *FileWatcher {
	return &FileWatcher{
		SourceDir:        sourceDir,
		events:           make(map[string]time.Time),
		debounceDuration: 430 * time.Millisecond,
	}
}

func (fw *FileWatcher) Watch(callback func(string)) {
	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		slog.Error(err.Error())
		return
	}
	defer watcher.Close()

	// Walk through the source directory and add directories to the watcher
	err = filepath.Walk(fw.SourceDir, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			err := watcher.Add(path)
			if err != nil {
				return err
			}
		}
		return nil
	})

	if err != nil {
		slog.Error("Error walking through source directory:", err)
		return
	}

	for {
		select {
		case event, ok := <-watcher.Events:
			if !ok {
				return
			}
			if event.Op&fsnotify.Write == fsnotify.Write ||
				event.Op&fsnotify.Create == fsnotify.Create ||
				event.Op&fsnotify.Remove == fsnotify.Remove {

				currentTime := time.Now()
				lastEventTime, found := fw.events[event.Name]

				// Debouncing event
				if !found || currentTime.Sub(lastEventTime) > fw.debounceDuration {
					fw.events[event.Name] = currentTime
					time.Sleep(50 * time.Millisecond) // Debounce hack
					callback(event.Name)
				}
			}

			// If the event is a create event and the file is a directory, add it to the watcher
			if event.Op&fsnotify.Create == fsnotify.Create {
				info, err := os.Stat(event.Name)
				if err == nil && info.IsDir() {
					watcher.Add(event.Name)
				}
			}
		case err := <-watcher.Errors:
			slog.Error("Error:", err)
		}
	}
}
