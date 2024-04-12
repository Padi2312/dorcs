package main

import (
	"embed"
	"log/slog"
	"os"
	"os/signal"
	"time"

	"github.com/padi2312/dorcs/pkg/dorcs"
	"github.com/padi2312/dorcs/pkg/server"
	"github.com/padi2312/dorcs/pkg/utils"
)

var version = "0.4.0"

//go:embed frontend/dist/*
var assets embed.FS

func main() {

	isWatchMode := false
	isPreviewMode := false

	d := dorcs.NewDorcs(assets)
	args := os.Args[1:]
	if len(args) > 0 {

		switch args[0] {
		case "--watch":
			isWatchMode = true
			d.SetWatchMode()

		case "--version":
			slog.Info("🚀 dorcs v" + version)
			os.Exit(0)

		case "--preview":
			isPreviewMode = true

		default:
			slog.Error("Invalid argument: " + args[0])
		}
	}

	slog.Info("🚀 Starting dorcs v" + version)
	d.Build()

	server := server.NewServer(d.Config)
	server.EnableStaticServe()

	// Only enable websocket in watch mode
	if isWatchMode {
		server.EnableWebsocket()
	}

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


	if isWatchMode || isPreviewMode {
		quit := make(chan os.Signal, 1)
		server.Start()
		signal.Notify(quit, os.Interrupt)

		<-quit
		slog.Info("🚀 Exiting dorcs...")
	}

}
