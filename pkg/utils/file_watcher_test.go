package utils

import (
	"os"
	"testing"
	"time"
)

func TestFileWatcher_Watch(t *testing.T) {

	// Create a temporary directory
	tmpDir := t.TempDir()

	// Create a markdown file in the temporary directory
	filePath := tmpDir + "/test.md"
	var file, err = os.Create(filePath)
	if err != nil {
		t.Fatal(err)
	}

	// Write some content to the file
	err = os.WriteFile(filePath, []byte("# Test\n"), 0644)
	if err != nil {
		t.Fatal(err)
	}
	file.Close()

	// Create a new file watcher
	fw := NewFileWatcher(tmpDir)

	// Create a channel to receive the file path
	filePathChan := make(chan string)

	// Start watching the directory
	go fw.Watch(func(path string) {
		filePathChan <- path
	})

	// Wait for the file path to be received
	go func() {
		select {
		case filePathReceived := <-filePathChan:
			// Check if the received file path is the same as the file path
			if filePathReceived != filePath {
				t.Errorf("Expected file path %s, got %s", filePath, filePathReceived)
			}
		case <-time.After(2 * time.Second):
			t.Error("Timeout: File path not received")
		}
	}()

	// Append some content to the file
	file, err = os.OpenFile(filePath, os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		t.Fatal(err)
	}
	_, err = file.WriteString("## Test 3\n")
	if err != nil {
		t.Fatal(err)
	}
	file.Close()

}
