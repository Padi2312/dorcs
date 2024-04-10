package dorcs 

import (
	"regexp"
	"strconv"
	"strings"
)

type MetaData struct {
	Title    string
	Position int
}

// Parses the metadata from a markdown file.
func ParseMetadata(content string) *MetaData {
	re := regexp.MustCompile(`(?s)^---\s*(.*?)\s*---`)
	metaData := &MetaData{}

	if cap := re.FindStringSubmatch(content); len(cap) > 0 {
		metaStr := cap[1]
		lines := strings.Split(metaStr, "\n")

		for _, line := range lines {
			parts := strings.SplitN(line, ":", 2)
			if len(parts) == 2 {
				key := strings.TrimSpace(parts[0])
				value := strings.TrimSpace(parts[1])

				// Match the key and update the struct fields accordingly
				switch key {
				case "title":
					metaData.Title = value
				case "position":
					if pos, err := strconv.ParseInt(value, 10, 64); err == nil {
						metaData.Position = int(pos)
					}
				default:
					// Ignore unknown keys
				}
			}
		}
	}

	return metaData
}

// Remove metadata from the content
func RemoveMetadata(content string) string {
	re := regexp.MustCompile(`(?s)^---\s*(.*?)\s*---`)
	return re.ReplaceAllString(content, "")
}
