package v1

import tea "github.com/charmbracelet/bubbletea"

func DashbordUpdate(m model, msg tea.Msg) (model, tea.Cmd) {

	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		dockStyle.Width(msg.Width)
		dockStyle.Height(msg.Height)
		return m, nil

	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "cntl+c", "esc":
			return m, tea.Quit
		}
	}

	return m, nil
}

func DashbordView(m model) string {
	return  dockStyle.Render("Dashbord")
}
