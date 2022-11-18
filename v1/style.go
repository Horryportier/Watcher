package v1

import (
        "github.com/charmbracelet/lipgloss"
)

var (

        border = lipgloss.RoundedBorder()
        primaryColor = lipgloss.Color("#34bdeb")
        secondaryColor = lipgloss.Color("#d8ebe9")
        unfocusedColor = lipgloss.Color("#505959")
        accentColor = lipgloss.Color("#7521eb")
        dockStyle = lipgloss.NewStyle().Align(lipgloss.Center).Padding(4).Border(border).BorderForeground(unfocusedColor)


        accentText = lipgloss.NewStyle().Foreground(accentColor)
        itemStyle = lipgloss.NewStyle().Foreground(secondaryColor)
        selectedItemStyle = lipgloss.NewStyle().Foreground(primaryColor)
)
