package v1

// TODO Game data


type AllData struct {
        PageProps PageProps
}

type PageProps struct {
        Data Data
        Region string
}

type Data struct {
        Id int
        Name string
        Level int
        Lp_histories []Lp_histories
        Previous_seasons []Previous_seasons
        League_stats []League_stats
}

type League_stats struct {
        Queue_info Queue_info
        Tier_info Tier_info
        Win int
        Lose int
        Is_hot_streak bool
        Is_flesh_blood bool
        Is_veteran bool
        Is_Inactive bool
}

type Queue_info struct {
        Queue_translate string
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
        Tier string
        Division int
        Lp int
}
