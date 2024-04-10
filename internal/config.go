package internal

import (
	"encoding/json"
	"io"
	"os"
)

const DefaultConfigPath = "dorcs.config.json"

type Config struct {
	Source       *string         `json:"source,omitempty"`
	Output       *string         `json:"output,omitempty"`
	Server       *ServerSettings `json:"server,omitempty"`
	PageSettings *PageSettings   `json:"page_settings,omitempty"`
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

	return &Config{
		Source:       &source,
		Output:       &output,
		Server:       &ServerSettings{Port: 8080},
		PageSettings: &PageSettings{PageTitle: &pageTitle},
	}
}

func ConfigFromFile() (*Config, error) {
	c := &Config{}

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

	// Set default values for nullable fields
	if c.Source == nil {
		source := "docs"
		c.Source = &source
	}
	if c.Output == nil {
		output := "output"
		c.Output = &output
	}
	if c.Server == nil {
		c.Server = &ServerSettings{Port: 8080}
	}
	if c.PageSettings == nil {
		pageTitle := "Documentation"
		c.PageSettings = &PageSettings{PageTitle: &pageTitle}
	}

	return c, nil
}
