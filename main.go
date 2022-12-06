package main

import (
	"log"
	app "src/watcher/v1"
)

func main() {
	err := app.Start()

	if err != nil {
		log.Fatal(err)
	}
}
