package v1

import (

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

type State int

const (
	Search State = iota
	Dashbord
        Debug
        season = 12
)

var (
	regions = []string{"ru", "tr", "br", "oce", "las", "eune", "euw", "na", "kr", "lan"}
	name    string
	region  string
        player Player

        numOfGameModes int
        gameMode int = 0
)

type model struct {
	state       State
	SearchModel SearchModel
	Dashboard   string
}




func initialModel() model {
	// textinput
	ti := textinput.New()
	ti.Placeholder = "Player Name"
	ti.CharLimit = 64
	ti.Focus()
	ti.Width = 32
	// list
	var items []list.Item
	for _, val := range regions {
		i := item(val)
		items = append(items, i)
	}

	ls := list.New(items, itemDelegate{},0, 0)

	var searchModel = SearchModel{textinput: ti, list: ls}
	return model{state: Search, SearchModel: searchModel, Dashboard: ""}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	if m.state == Search {
		return SearchUpdate(m, msg)
	}
        if m.state == Dashbord {
                return DashbordUpdate(m, msg)
        }
        if m.state == Debug {
                return DebugUpdate(m, msg)
        }
	return m, nil
}

func (m model) View() string {
        if m.state == Search{
                return SearchView(m)
        }
        if m.state == Dashbord{
                return DashbordView(m)
        }
        if m.state == Debug{
                return DebugView(m)
        }
	return  ""
}

func Start() error {
        p := tea.NewProgram(initialModel(), tea.WithAltScreen())

        err := p.Start()
	return err
}
