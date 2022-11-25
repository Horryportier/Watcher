package v1

import (
	"fmt"
	"io"
	"strconv"
	"strings"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var (
	choice             string
	listFocused        bool
	defName, defRegion = getDefultSearch()
	debug              bool
)

type SearchModel struct {
	textinput textinput.Model
	list      list.Model
}

type item string

type itemDelegate struct{}

func (i item) FilterValue() string { return choice }

func (d itemDelegate) Height() int                               { return 10 }
func (d itemDelegate) Spacing() int                              { return 0 }
func (d itemDelegate) Update(msg tea.Msg, m *list.Model) tea.Cmd { return nil }
func (d itemDelegate) Render(w io.Writer, m list.Model, index int, listitem list.Item) {
	i, ok := listitem.(item)
	if !ok {
		return
	}

	str := fmt.Sprintf("%d %s.", index+1, i)
	fn := itemStyle.Render
	if index == m.Index() && listFocused {
		choice = fmt.Sprintf("%s", i)
		fn = func(s string) string {
			return selectedItemStyle.Render("> " + s)
		}
	}
	fmt.Fprint(w, fn(str))
}

func SearchUpdate(m model, msg tea.Msg) (model, tea.Cmd) {
	var cmd tea.Cmd
	m.SearchModel.list.SetShowHelp(false)
	m.SearchModel.list.SetShowTitle(false)
	m.SearchModel.list.SetFilteringEnabled(false)
	m.SearchModel.list.SetShowStatusBar(false)
	m.SearchModel.list.SetShowPagination(false)

	tFocus := m.SearchModel.textinput.Focused()

	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		dockStyle.Width(msg.Width)
		dockStyle.Height(msg.Height)
		m.SearchModel.list.SetWidth(msg.Width)
		m.SearchModel.list.SetHeight(10)
		return m, nil
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "ctrl+c":
			return m, tea.Quit
		case "tab":
			if tFocus {
				m.SearchModel.textinput.Blur()
			} else {
				m.SearchModel.textinput.Focus()
			}

		case "enter":
			if !tFocus {
				region = m.SearchModel.list.FilterValue()
				m.SearchModel.textinput.Focus()
				return m, nil
			}
			return searchPlayer(m, debug)
		case "1", "2", "3", "4", "5", "6", "7", "8", "9", "0":
			if !tFocus {
				i, _ := strconv.Atoi(keypress)
				if i == 0 {
					m.SearchModel.list.Select(9)
				} else {
					m.SearchModel.list.Select(i - 1)
				}
				return m, nil
			}

		case "ctrl+d":
			debug = !debug
		}
	}

	tFocus = m.SearchModel.textinput.Focused()

	if tFocus {
		listFocused = false
		m.SearchModel.textinput, cmd = m.SearchModel.textinput.Update(msg)
	}

	if !tFocus {
		listFocused = true
		m.SearchModel.list, cmd = m.SearchModel.list.Update(msg)
	}
	return m, cmd
}

func SearchView(m model) string {
	var str strings.Builder

	str.WriteString(m.SearchModel.textinput.View())
	str.WriteString(" [ " + accentText.Render(choice) + " ]")
	if m.SearchModel.textinput.Value() == defName && choice == defRegion {
		str.WriteString(" ✔️ ")
	}
	str.WriteRune('\n')
	str.WriteRune('\n')
	str.WriteRune('\n')
	str.WriteRune('\n')
	str.WriteString(m.SearchModel.list.View())
	str.WriteRune('\n')
	str.WriteRune('\n')
	box3 := box.Copy().Foreground(primaryColor).Align(lipgloss.Bottom)
	str.WriteString(box3.Render(unfocusedText.Render("Def Search: ") + defName + " | " + defRegion))

	dockStyle.Align(lipgloss.Center)
	return dockStyle.Render(str.String())
}
