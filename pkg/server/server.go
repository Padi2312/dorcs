package server

import (
	"fmt"
	"net/http"

	"github.com/padi2312/dorcs/internal"
)

// Server is a struct that holds the server settings
type Server struct {
	Config *internal.Config
}

// NewServer creates a new server
func NewServer(config *internal.Config) *Server {
	return &Server{
		Config: config,
	}
}

// Start starts the server
func (s *Server) Start() {
	http.HandleFunc("/", s.index)
	addr := fmt.Sprintf("0.0.0.0:%d", s.Config.Server.Port)
	http.ListenAndServe(addr, nil)
}

func (s *Server) index(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, World!")
}
