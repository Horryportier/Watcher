package v1


import (
        "strconv"
        "strings"

        tea "github.com/charmbracelet/bubbletea"
        "github.com/charmbracelet/lipgloss"
)

func DashbordUpdate(m model, msg tea.Msg) (model, tea.Cmd) {

        numOfGameModes = len(player.game_mode)

        switch msg := msg.(type) {
        case tea.WindowSizeMsg:
                dockStyle.Width(msg.Width)
                dockStyle.Height(msg.Height)
                return m, nil

        case tea.KeyMsg:
                switch keypress := msg.String(); keypress {
                case "ctrl+c", "esc", "q":
                        return m, tea.Quit

                case "s", "enter", "tab":
                        player.game_mode = nil
                        player.previous_seasons = nil
                        m.state = Search
                        return m, nil
                case "j":
                        gameMode++
                        if gameMode >= numOfGameModes {
                                gameMode = 0
                        }
                        return m, nil
                case "k":
                        gameMode--
                        if gameMode <= 0 {
                                gameMode = numOfGameModes-1
                        }
                        return m, nil
                }
        }

        return m, nil
}

func DashbordView(m model) string {
        var content strings.Builder
        var summoner strings.Builder
        var rank strings.Builder
        var winrate strings.Builder
        var prevSesons strings.Builder


        if player.name == "" {
                return dockStyle.Render(box.Render(accentText.Render("Account doesn't exist")))
        }
        summoner.WriteString(primaryText.Render(player.name))
        summoner.WriteRune('\n')
        summoner.WriteString(primaryText.Render(player.region))
        summoner.WriteString(" | ")
        summoner.WriteString(strconv.Itoa(player.elopoint))

        rank.WriteString(player.game_mode[gameMode].queue_type)
        rank.WriteRune('\n')

        rankStyle := rankedColors[player.game_mode[gameMode].tier.braket]

        if player.game_mode[gameMode].tier.braket == "" {
                rank.WriteString(rankStyle.Render("UNRANKED"))
        }else {
                rank.WriteString(rankStyle.Render(player.game_mode[gameMode].tier.braket))
        }
        rank.WriteString(" | ")
        rank.WriteString(strconv.Itoa(player.game_mode[gameMode].tier.division))
        rank.WriteRune('\n')
        rank.WriteString("LP: ")
        rank.WriteString(strconv.Itoa(player.game_mode[gameMode].tier.lp))


        winrate.WriteRune('\n')
        winrate.WriteString("win: "+strconv.Itoa(player.game_mode[gameMode].win))
        winrate.WriteString(" | ")
        winrate.WriteString("lose: "+strconv.Itoa(player.game_mode[gameMode].lose))
        winrate.WriteRune('\n')
        winrate.WriteString("Hot Streak: ")
        if player.game_mode[gameMode].is_hot_streak == true {
                winrate.WriteString("🔥")
        }else {
                winrate.WriteString("❄️")
        }


        if len(player.previous_seasons) > 0 {
                for i, val := range player.previous_seasons {
                        prevSesons.WriteString(secondaryText.Render(strconv.Itoa(season-i) + ": "))
                        prevSesons.WriteString(unfocusedText.Render(val.tier.braket + " " + strconv.Itoa(val.tier.division) + " "))
                }
        }
        box1 := box.Copy().BorderForeground(rankStyle.GetForeground()).Padding(0,0,1)
        box2 := box.Copy().BorderForeground(rankStyle.GetForeground())

        rankStr := lipgloss.JoinHorizontal(lipgloss.Center, rank.String(), winrate.String())

        content.WriteString(box.Render(prevSesons.String()))
        content.WriteRune('\n')
        content.WriteString(lipgloss.JoinHorizontal(lipgloss.Center,box1.Render(summoner.String()),box2.Render(rankStr)))

        dockStyle.Align(lipgloss.Left)

        dockStyle = dockStyle.BorderForeground(rankStyle.GetForeground())

        return dockStyle.Render(content.String())
}
