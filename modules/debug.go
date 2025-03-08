package modules

import "os"

func Debug_mode() bool {
	return os.Getenv("DEBUG") == "1"
}
