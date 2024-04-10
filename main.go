package main

import (
	"embed"
	"time"

	"github.com/padi2312/dorcs/pkg/dorcs"
)

var version = "0.4.0-alpha-2"

//go:embed frontend/dist/*
var assets embed.FS

func main() {
	dorcs := dorcs.NewDorcs(assets)
	start := time.Now()
	dorcs.Run()
	duration := time.Since(start)
	println("✅ Build complete in ", duration.Round(time.Millisecond).String())
}
