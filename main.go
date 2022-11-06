package main

import (
	"log"
	app "watcher/src/v1"
)

func main() {
	err := app.Start()

	if err != nil {
		log.Fatal(err)
	}
}
