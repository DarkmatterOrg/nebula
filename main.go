package main

import (
	"github.com/darkmatterorg/nebula/cmd"
	"github.com/darkmatterorg/nebula/config"
)

func main() {
	// Load config
	config.LoadConfig()

	cmd.Execute()
}
