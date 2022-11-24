package v1

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

func DebugUpdate(m model, msg tea.Msg) (model, tea.Cmd) {
        switch msg := msg.(type) {
        case tea.WindowSizeMsg:
                dockStyle.Width(msg.Width)
                dockStyle.Height(msg.Height)
                return m, nil

        case tea.KeyMsg:
                switch keypress := msg.String(); keypress {
                case "ctrl+c", "esc", "q":
                        return m, tea.Quit

                }
        }
        return m, nil
}

func DebugView(m model) string {
        return dockStyle.Render(fmt.Sprintf("%v", player))
}
