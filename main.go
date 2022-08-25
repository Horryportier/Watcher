package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"
)

var client *http.Client

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
        URL := "https://" + region + ".op.gg/_next/data/qx1tV_J8eyd7sy37QyPtj/summoners/"+ region + "/" + name +".json?region=" + region + "&summoner=" + strings.ReplaceAll(name, " ", "+")
        var player Player

        err := GetJson(URL, &player)

                return err, player
}

func GetJson(url string, target interface {}) error {
        resp, err := client.Get(url)
        if err != nil {
                return err
        }

        defer resp.Body.Close()

        return json.NewDecoder(resp.Body).Decode(target)
}

func main() {

        client = &http.Client{Timeout: 10 * time.Second}

        err, player := GetPlayer("eune", "horryportier6")
        
        fmt.Printf("error: %s\n", err)
        fmt.Printf("Player: %s\n", player)
}
