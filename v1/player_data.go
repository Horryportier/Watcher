package v1

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

type Player struct {
	region           string
	name             string
	game_mode        []game_mode
	elopoint         int
	previous_seasons []previous_seasons
}

type game_mode struct {
	queue_type    string
	tier          tier
	win           int
	lose          int
	is_hot_streak bool
}

type previous_seasons struct {
	season_id int
	tier      tier
}

type tier struct {
	braket   string
	division int
	lp       int
}

func getHash() string {
	resp, err := http.Get("https://www.op.gg/")
	if err != nil {
		log.Fatal(err)
	}
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}
	d := string(body)

	str := strings.Split(d, "/")

	var val string
	for i := 0; i < len(str); i++ {
		if str[i] == "_buildManifest.js\" defer=\"\"><" {
			val = str[i-1]
		}
	}

	return val
}

func MakeUrl(name string, region string) string {
	Hash := getHash()

	url := "https://www.op.gg/_next/data/" + Hash + "/summoners/" + region + "/" + strings.ReplaceAll(name, " ", "+") + ".json?region=" + region + "&summoner=" + strings.ReplaceAll(name, " ", "+")

	return url
}

func AssingData(p Player, data AllData) Player {
	var tmp_game_mode game_mode
	var tmp_previous_seasons previous_seasons
	p.region = data.PageProps.Region
	p.name = data.PageProps.Data.Name
	for _, val := range data.PageProps.Data.League_stats {
		tmp_game_mode.queue_type = val.Queue_info.Queue_translate
		tmp_game_mode.win = val.Win
		tmp_game_mode.lose = val.Lose
		tmp_game_mode.is_hot_streak = val.Is_hot_streak
		tmp_game_mode.tier.braket = val.Tier_info.Tier
		tmp_game_mode.tier.division = val.Tier_info.Division
		tmp_game_mode.tier.lp = val.Tier_info.Lp

		p.game_mode = append(p.game_mode, tmp_game_mode)
	}
	for _, val := range data.PageProps.Data.Previous_seasons {
		tmp_previous_seasons.season_id = val.Season_id
		tmp_previous_seasons.tier.braket = val.Tier_info.Tier
		tmp_previous_seasons.tier.division = val.Tier_info.Division
		tmp_previous_seasons.tier.lp = val.Tier_info.Lp

		p.previous_seasons = append(p.previous_seasons, tmp_previous_seasons)
	}

	return p
}

func Parse(p Player, url string) (Player, error) {
	var data AllData
	resp, err := http.Get(url)
	if err != nil {
		return p, err
	}

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return p, err
	}

	err = json.Unmarshal(body, &data)
	if err != nil {
		return p, err
	}

	p = AssingData(p, data)

	return p, nil
}

func (p Player) GetPlayer(name string, region string) (Player, error) {
	url := MakeUrl(name, region)
	p, err := Parse(p, url)
	return p, err
}
