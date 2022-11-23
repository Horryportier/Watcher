package v1

import (
        "github.com/charmbracelet/lipgloss"
)

var (
        border         = lipgloss.RoundedBorder()
        primaryColor   = lipgloss.Color("#34bdeb")
        secondaryColor = lipgloss.Color("#d8ebe9")
        unfocusedColor = lipgloss.Color("#505959")
        accentColor    = lipgloss.Color("#7521eb")
        dockStyle      = lipgloss.NewStyle().Align(lipgloss.Center).PaddingLeft(4).PaddingTop(2).Border(border).BorderForeground(unfocusedColor)
        box            = lipgloss.NewStyle().Border(border).BorderForeground(unfocusedColor)

        rankedColors = map[string]lipgloss.Style {
                "UNRANKED":    lipgloss.NewStyle().Foreground(unfocusedColor),
                "IRON":        lipgloss.NewStyle().Foreground(lipgloss.Color("#443f45")),
                "BRONZE":      lipgloss.NewStyle().Foreground(lipgloss.Color("#b57c5c")),
                "SILVER":      lipgloss.NewStyle().Foreground(lipgloss.Color("#5a6c8d")),
                "GOLD":        lipgloss.NewStyle().Foreground(lipgloss.Color("#fcbf49")),
                "PLATINUM":    lipgloss.NewStyle().Foreground(lipgloss.Color("#00e196")),
                "DIAMOND":     lipgloss.NewStyle().Foreground(lipgloss.Color("#74eaff")),
                "MASTER":      lipgloss.NewStyle().Foreground(lipgloss.Color("#e614e1")),
                "GRANDMASTER": lipgloss.NewStyle().Foreground(lipgloss.Color("#df130e")),
                "CHALLENGER":  lipgloss.NewStyle().Foreground(lipgloss.Color("#005cea")),
        }


        primaryText   = lipgloss.NewStyle().Foreground(primaryColor)
        accentText    = lipgloss.NewStyle().Foreground(accentColor)
        secondaryText = lipgloss.NewStyle().Foreground(secondaryColor)
        unfocusedText = lipgloss.NewStyle().Foreground(unfocusedColor)

        itemStyle         = lipgloss.NewStyle().Foreground(secondaryColor)
        selectedItemStyle = lipgloss.NewStyle().Foreground(primaryColor)
)
