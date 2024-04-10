package utils

import (
	"io"
	"io/fs"
	"os"
	"path/filepath"
)

// copyFile copies the content from src (embedded file) to dst (file on filesystem).
func CopyFile(src fs.File, dstPath string) error {
	dst, err := os.Create(dstPath)
	if err != nil {
		return err
	}
	defer dst.Close()

	_, err = io.Copy(dst, src)
	return err
}

// copyDir recursively copies files and directories from the embedded filesystem to the target directory on the actual filesystem.
// It trims the rootPath from the source paths before joining them with the targetDir.
func CopyDir(fsys fs.FS, root, targetDir string) error {
	return fs.WalkDir(fsys, root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		// Trim the root path from the source path
		relPath, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}

		targetPath := filepath.Join(targetDir, relPath)

		if d.IsDir() {
			// Create the directory if it doesn't exist
			return os.MkdirAll(targetPath, 0755)
		}

		src, err := fsys.Open(path)
		if err != nil {
			return err
		}
		defer src.Close()

		// Copy the file content to the target directory
		return CopyFile(src, targetPath)
	})
}
