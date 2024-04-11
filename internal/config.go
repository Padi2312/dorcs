package internal

import (
	"encoding/json"
	"io"
	"os"
)

const DefaultConfigPath = "dorcs.config.json"

type Config struct {
	Source       *string         `json:"source"`
	Output       *string         `json:"output"`
	Server       *ServerSettings `json:"server"`
	PageSettings *PageSettings   `json:"page_settings"`
	Schema       *string         `json:"schema"`
}

func NewConfig() *Config {
	// First check if there is a dorcs.config.json file
	// If there is, read the file and return the Config struct
	// If there isn't, return a default Config struct

	// Check if the file exists
	if _, err := os.Stat(DefaultConfigPath); os.IsNotExist(err) {
		// File does not exist, return default config
		return Default()
	}

	config, err := ConfigFromFile()
	if err != nil {
		return Default()
	}
	return config
}

func Default() *Config {
	source := "docs"
	output := "output"
	pageTitle := "Documentation"
	schema := "https://dorcs.allthing.eu/dorcs.config.schema.json"
	dev := false

	return &Config{
		Source:       &source,
		Output:       &output,
		Server:       &ServerSettings{Port: 8080},
		PageSettings: &PageSettings{PageTitle: &pageTitle, Dev: &dev},
		Schema:       &schema,
	}
}

func ConfigFromFile() (*Config, error) {
	c := Default()

	// Read file
	file, err := os.Open(DefaultConfigPath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	// Read file content
	content, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	// Unmarshal JSON content into Config struct
	err = json.Unmarshal(content, c)
	if err != nil {
		return nil, err
	}

	return c, nil
}
