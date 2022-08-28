package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/spinner"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

var client *http.Client


type model struct {
        textInput textinput.Model
        spinner spinner.Model

        name string

        list bool
        typing bool
        loading bool

        display bool 

        err error

        data Player

        cursor int
        choices []string
        choise string
        selected map[int]struct{}
}

// Player struct
type Player struct {
        PageProps struct{
        
                Region string 
                Data struct{
                        Id int 
                        Summoner_id string 
                        Name string 
                        Level int 
                        Team_info string
                        Lp_histories []Lp_histories 
                        Previous_seasons []Previous_seasons
                        Leauge_stats Leauge_stats
                }
        }
}
type Lp_histories struct{
        Tier_info struct {
                Tier string
                Division int
                Lp int
        }
        Elo_point int
}
type Previous_seasons struct {
        Tier_info struct {
                Tier string
                Division int
                Lp int
        }
}
type Leauge_stats struct {
        Win int
        Lose int
        Is_hot_streak bool
}

func GetPlayer(region string, name string) (error , Player) {
        URL, _ := GetUrl(region, name)
        fmt.Printf("URL: %s", URL)
        var player Player

        err := GetJson(URL, &player)

                return err, player
}

func GetUrl(region string, name string) (string, error){
        resp, err := client.Get("https://op.gg/")

        if err != nil {
                return "", err
        }
         
        defer resp.Body.Close()
        bb, _ := ioutil.ReadAll(resp.Body)

        splited := strings.Split(string(bb), "/")

        hs := ""
        for i :=0; i < len(splited); i++{
                if splited[i] == "_buildManifest.js\" defer=\"\"><" {
                        hs = splited[i-1]
                }
        }
        return "https://op.gg/_next/data/" + hs +"/summoners/"+ region + "/" + strings.ReplaceAll(name, " ", "+") + ".json?region="+ region + "&summoner=" + strings.ReplaceAll(name, " ", "+"), err
}

func GetJson(url string, target interface {}) error {
        resp, err := client.Get(url)
        if err != nil {
                return err
        }

        defer resp.Body.Close()

        return json.NewDecoder(resp.Body).Decode(target)
}

func initialModel() model {
        t := textinput.NewModel()
        t.Focus()

        s := spinner.NewModel()
        s.Spinner = spinner.Pulse
        return model{
                list:  true,

                textInput: t,
                spinner: s,

                choices: []string{"br", "eune", "euw", "lan", "las", "na", "oce", "ru", "tr", "jp", "kr"},

                selected: make(map[int]struct{}),
        }
}

func (m model) Init() tea.Cmd {
        return textinput.Blink
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
        switch msg := msg.(type) {
        case tea.KeyMsg:
                switch msg.String() {
                case "ctrl+c", "q":
                        return m, tea.Quit
                case "up", "k":
                        if m.list {
                        if m.cursor > 0 {
                                m.cursor--
                        }
                        }
                case "down", "j":
                        if m.list {
                        if m.cursor < len(m.choices)-1 {
                                m.cursor++
                        }
                        }
                case "enter":
                        if m.list {
                        _, ok := m.selected[m.cursor]
                        if ok{
                                delete(m.selected, m.cursor)
                        }else {
                                m.selected[m.cursor] = struct{}{}
                                m.choise = m.choices[m.cursor]
                                m.list = false 
                                m.typing = true
                        }
                        }
                        if m.typing {
                        query := strings.TrimSpace(m.textInput.Value())
                        if query != "" {
                                m.typing = false 
                                m.loading= true
                                m, err := m.fetchPlayer(query)
                                if err != nil {
                                        m.err = err
                                } 
                                return m, spinner.Tick
                        }
                }
                }
        }
        if m.typing {
                var cmd tea.Cmd
                m.textInput, cmd = m.textInput.Update(msg)
                return m, cmd
        }
        if m.loading {
                var cmd tea.Cmd
                m.spinner, cmd = m.spinner.Update(msg)
                if m.data.PageProps.Data.Name != "" {
                        m.loading = false
                        m.display = true
                }
                return m, cmd
        }
        return m, nil
}

func (m model) fetchPlayer(query string) (model, error){
        var err error
        err, m.data = GetPlayer(m.choise, query)
        return m, err
}

func (m model) View() string {
        s := ""
        if m.list {
        s = "Choose Region:\n\n"

        for i, choice := range m.choices {
                cursor := " "
                if m.cursor == i {
                        cursor = ">"
                }

                checked := " "
                if _, ok := m.selected[i]; ok {
                        checked = "x"
                }

                s += fmt.Sprintf("%s [%s] %s\n", cursor, checked, choice)
        }
        s += "\n Press q to quit.\n"
        return s
        }

        if m.typing {                                                                                                                                                 
                return   "Type player name:\n" + m.textInput.View() +"\nPress q to quit"
        }

        if m.loading {
                return fmt.Sprintf("\n%s fetching data...\n\n Press q to  quit", m.spinner.View())
        }

        if m.err != nil {
                return fmt.Sprintf("Error: %s", m.err) +"\n\n Press q to quit"
        }
        
        if m.display {
                name := m.data.PageProps.Data.Name
                region := m.data.PageProps.Region
                level := strconv.Itoa( m.data.PageProps.Data.Level)
                team := m.data.PageProps.Data.Team_info
                tier := m.data.PageProps.Data.Lp_histories[0].Tier_info.Tier
                division := strconv.Itoa(m.data.PageProps.Data.Lp_histories[0].Tier_info.Division)
                lp := strconv.Itoa(m.data.PageProps.Data.Lp_histories[0].Tier_info.Lp)
                elopoint := strconv.Itoa(m.data.PageProps.Data.Lp_histories[0].Elo_point)
                return fmt.Sprintf("Name: %s Region: %s\nLevel: %s  Team: %s\nTier: %s %s\nLp: %v EloPiont: %s", 
                name, region, level, team, tier, division, lp, elopoint,
        ) + "\n\n Press q to exit" 
        }

        return "Press q to exit"
}


func main() {

        client = &http.Client{Timeout: 10 * time.Second}


        p := tea.NewProgram(initialModel(), tea.WithAltScreen())
        if err := p.Start(); err != nil {
                fmt.Printf("Error: %v", err)
                os.Exit(1)
        }
}
