use core::fmt;

use crossterm::style::{Color, Stylize};
use riven::{consts::Tier, models::league_v4::LeagueEntry};

pub struct LeagueEntryDisplay {
    pub league_entry: LeagueEntry,
}

impl fmt::Display for LeagueEntryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = &self.league_entry;
        let text = format!(
            r###"
{}
    {}
    {} {}
    {}/{}
            "###,
            entry
                .summoner_name
                .as_str()
                .with(crossterm::style::Color::Yellow),
            "Rank"
                .with(Color::Grey)
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined),
            entry
                .tier
                .unwrap_or(Tier::IRON)
                .to_string()
                .with(Color::Yellow),
            entry
                .rank
                .unwrap_or(riven::consts::Division::I)
                .to_string()
                .with(Color::Blue),
            entry.wins.to_string().with(Color::Green),
            entry.losses.to_string().with(Color::Red)
        );
        write!(f, "{}", text)
    }
}
