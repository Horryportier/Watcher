package v1

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
)

func getDefultSearch() (string, string) {
	defName := os.Getenv("WATCHER_NAME")
	defRegion := os.Getenv("WATCHER_REGION")
	return defName, defRegion
}

func searchPlayer(m model, debug bool) (model, tea.Cmd) {
	var err error
	defName, defRegion = getDefultSearch()
	if m.SearchModel.textinput.Focused() && choice != "" && m.SearchModel.textinput.Value() != "" {
		region = choice
		name = m.SearchModel.textinput.Value()

		player, err = player.GetPlayer(name, region)

		if err != nil {
			fmt.Printf("could not get player")
			return m, tea.Quit
		}

	}
	if m.SearchModel.textinput.Focused() && choice == "" && m.SearchModel.textinput.Value() == "" {
		region = defRegion
		name = defName

		player, err = player.GetPlayer(name, region)

		if err != nil {
			fmt.Printf("could not get player")
			return m, tea.Quit
		}

	}
	if debug {
		m.state = Debug
	} else {
		m.state = Dashbord
	}
	return m, nil
}
