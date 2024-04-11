package main

import (
	"embed"
	"log/slog"
	"os"
	"time"

	"github.com/padi2312/dorcs/pkg/dorcs"
	"github.com/padi2312/dorcs/pkg/server"
	"github.com/padi2312/dorcs/pkg/utils"
)

var version = "0.4.0-alpha-2"

//go:embed frontend/dist/*
var assets embed.FS

func main() {

	isWatchMode := false
	slog.Info("🚀 Starting dorcs v" + version)

	d := dorcs.NewDorcs(assets)
	args := os.Args[1:]
	if len(args) > 0 && args[0] == "--watch" {
		isWatchMode = true
		d.SetWatchMode()
	}

	d.Build()

	server := server.NewServer(d.Config)
	server.EnableStaticServe()
	server.EnableWebsocket()

	if isWatchMode {
		slog.Info("👓 Watching for changes in dir: " + *d.Config.Source)
		go utils.NewFileWatcher(*d.Config.Source).Watch(func(s string) {
			start := time.Now()
			err := d.ProcessFile(s)
			if err != nil {
				slog.Error("Error processing file:", err)
			}
			duration := time.Since(start)
			slog.Info("🔄 Rebuild of " + s + " completed in " + duration.Round(time.Millisecond).String())

			if d.LatestModifiedUrl != nil {
				server.BroadcastMessage(*d.LatestModifiedUrl)
			}

		})
	}

	if isWatchMode {
		server.Start()
	}

}
