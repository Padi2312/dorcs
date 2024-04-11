package server

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/gorilla/websocket"
	"github.com/padi2312/dorcs/internal"
)

type Server struct {
	Config   *internal.Config
	upgrader websocket.Upgrader
	Clients  map[*websocket.Conn]bool
}

func NewServer(config *internal.Config) *Server {
	return &Server{
		Config: config,
		upgrader: websocket.Upgrader{
			CheckOrigin: func(r *http.Request) bool {
				return true // Allow connections from any origin
			},
		},
		Clients: make(map[*websocket.Conn]bool),
	}
}

func (s *Server) EnableStaticServe() {
	http.HandleFunc("/", s.ServeSPA)
}

func (s *Server) EnableWebsocket() {
	http.HandleFunc("/ws", s.handleConnections)
}

func (s *Server) Start() {
	// Start the server
	addr := fmt.Sprintf("0.0.0.0:%d", s.Config.Server.Port)
	slog.Info(fmt.Sprintf("Server started on %s", addr))
	slog.Error(http.ListenAndServe(addr, nil).Error())
}

func (s *Server) handleConnections(w http.ResponseWriter, r *http.Request) {
	conn, err := s.upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("Upgrade error:", err)
		return
	}
	s.Clients[conn] = true
	// Close the connection and remove the client when it's closed
	defer func() {
		conn.Close()
		delete(s.Clients, conn)
	}()

	// Read messages from the client
	for {
		_, _, err := conn.ReadMessage()
		if err != nil {
			slog.Error("Read error:", err)
			break
		}
	}

}

func (s *Server) BroadcastMessage(message string) {
	for client := range s.Clients {
		if err := client.WriteMessage(websocket.TextMessage, []byte(message)); err != nil {
			slog.Error(fmt.Sprintf("Write error: %s", err))
		}
	}
}

func (s *Server) ServeSPA(w http.ResponseWriter, r *http.Request) {
	// Check if the requested file exists in the SPA build directory
	path := filepath.Join(*s.Config.Output, r.URL.Path)
	if _, err := os.Stat(path); os.IsNotExist(err) || strings.HasSuffix(r.URL.Path, "/") && !strings.Contains(r.URL.Path, "pages") {
		// If not, serve index.html
		http.ServeFile(w, r, filepath.Join(*s.Config.Output, "index.html"))
		return
	}
	// Serve the requested static file
	http.ServeFile(w, r, path)
}
