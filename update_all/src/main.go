package main

import (
	"update_all/src/cmd"

	log "github.com/sirupsen/logrus"
)

func init() {
	log.SetLevel(log.InfoLevel)
	log.SetFormatter(&log.TextFormatter{DisableTimestamp: true})
}

func main() {
	cmd.Execute()
}
