package config

import (
	"github.com/BurntSushi/toml"
	"github.com/darkmatterorg/orbit/utils"
)

type Settings struct {
	Insults bool   `toml:"insults"`
	Mode    string `toml:"mode"`
}

var Config Settings

const full_path = "/etc/nebula/config.toml"

func LoadConfig() {

	// Load config

	_, err := toml.DecodeFile(full_path, &Config)
	if err != nil {

	}
}

func FindConfig() {
	if utils.PathExists(full_path) {
		utils.Notice("Config can be found at " + full_path)
	} else {
		utils.Notice("Config should be created at " + full_path)
	}
}
