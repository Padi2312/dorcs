package internal

type ServerSettings struct {
	Enabled    bool `json:"enabled"`
	Port       int  `json:"port"`
	AutoReload bool `json:"auto_reload"`
}
