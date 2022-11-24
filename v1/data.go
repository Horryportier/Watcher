package v1

// TODO Game data

type AllData struct {
	PageProps PageProps
}

type PageProps struct {
	Data   Data
	Region string
	Games  Games
}

type Data struct {
	Id               int
	Name             string
	Level            int
	Lp_histories     []Lp_histories
	Previous_seasons []Previous_seasons
	League_stats     []League_stats
}

type League_stats struct {
	Queue_info     Queue_info
	Tier_info      Tier_info
	Win            int
	Lose           int
	Is_hot_streak  bool
	Is_flesh_blood bool
	Is_veteran     bool
	Is_Inactive    bool
}

type Queue_info struct {
	Id              int
	Queue_translate string
	game_type       string
}

type Previous_seasons struct {
	Season_id int
	Tier_info Tier_info
}

type Lp_histories struct {
	Tier_info Tier_info
	Elo_point int
}

type Tier_info struct {
	Tier     string
	Division int
	Lp       int
}

type Games struct {
	Data []struct {
		Created_at         string
		Game_map           string
		Queue_info         Queue_info
		Version            string
		Game_length_second int
		Is_remake          bool
		Is_recorded        bool
		Average_tier_info  Average_tier_info
		Participants       []Participants
		Items              []int
		Trinket_item       int
		Rune               Rune
		Tier_info          Tier_info
	}
}

type Rune struct {
	Primary_page_id   int
	Primary_rune_id   int
	Secondary_page_id int
}

type Participants struct {
	Summoner Summoner
}

type Summoner struct {
	Name        string
	Level       int
	Champion_id int
	Team_key    string
	Position    string
}

type Average_tier_info struct {
	Tier     string
	Division int
}
