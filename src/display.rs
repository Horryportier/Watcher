use core::fmt;
use std::fmt::Display;

use ansi_to_tui::IntoText;
use crossterm::style::{Color, Stylize};
use ratatui::{
    style::{self, Modifier, Style},
    text::{Line, Span, Text},
};
use riven::{
    consts::{Team, Tier},
    models::{
        champion_mastery_v4::ChampionMastery, league_v4::LeagueEntry, match_v5::Match,
        summoner_v4::Summoner,
    },
};

use crate::ui::app::Window;

#[macro_export]
macro_rules! no_data {
    () => {
        Text::from("no data")
    };
}

pub fn concat_text(texts: Vec<Text>) -> Text {
    let mut lines: Vec<Line> = vec![Line::default()];
    for t in texts {
        lines.append(&mut t.into_iter().collect::<Vec<Line>>())
    }
    Text::from(lines)
}
enum Pad {
    Left,
    Right,
    Center,
}

fn padding(text: String, padding: Pad, amount: usize, ch: u8) -> String {
    let fill = amount.checked_sub(text.len()).unwrap_or(0);
    if fill == 0 {
        return text;
    }
    match padding {
        Pad::Left => {
            let other = String::from_utf8(vec![ch; fill]).unwrap();
            return format!("{}{}", text, other);
        }
        Pad::Right => {
            let other = String::from_utf8(vec![ch; fill]).unwrap();
            return format!("{}{}", other, text);
        }
        Pad::Center => {
            let pre = String::from_utf8(vec![ch; fill / 2]).unwrap();
            let mut suf = String::from_utf8(vec![ch; fill / 2]).unwrap();

            if text.len() % 2 == 1 {
                suf = String::from_utf8(vec![ch; fill / 2 + 1]).unwrap();
            }

            return format!("{}{}{}", pre, text, suf);
        }
    }
}

pub trait DisplayToText<T: Display>
where
    Self: std::fmt::Display + Sized,
{
    fn into_text(&self) -> Text<'static> {
        let a = format!("{}", self);
        let a = a
            .split("\n")
            .into_iter()
            .filter(|f| *f != "")
            .collect::<Vec<_>>()
            .join("\n");
        match a.into_text() {
            Ok(t) => t,
            Err(_) => Text::from(no_data!()),
        }
    }
}

pub trait With {
    type Struct;
    fn with(entry: Self::Struct) -> Self;
}

#[derive(Clone)]
pub struct SummonerDisplay(pub Summoner);

impl Default for SummonerDisplay {
    fn default() -> Self {
        SummonerDisplay(Summoner {
            account_id: "".to_string(),
            profile_icon_id: 23,
            revision_date: 23,
            name: "test".to_string(),
            id: "lol".to_string(),
            puuid: "24365".to_string(),
            summoner_level: 69,
        })
    }
}

impl Display for SummonerDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = &self.0;

        let text = format!(
            r###"
{}  {}:{}
                           "###,
            entry
                .name
                .clone()
                .with(Color::Blue)
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined),
            "lvl".with(Color::Reset),
            entry.summoner_level.to_string().with(Color::Cyan)
        );

        write!(f, "{}", text)
    }
}

impl With for SummonerDisplay {
    type Struct = Summoner;
    fn with(entry: Self::Struct) -> SummonerDisplay {
        SummonerDisplay(entry)
    }
}
impl DisplayToText<SummonerDisplay> for SummonerDisplay {}

#[derive(Clone)]
pub struct LeagueEntryDisplay(pub LeagueEntry);

impl fmt::Display for LeagueEntryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = &self.0;

        let text = format!(
            r###"
{}
    {}
    {} {}
    {}/{}  {}%
    {}
            "###,
            entry
                .summoner_name
                .as_str()
                .with(crossterm::style::Color::Yellow),
            entry
                .queue_type
                .to_string()
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
            entry.losses.to_string().with(Color::Red),
            (entry.wins * 100 / (entry.wins + entry.losses))
                .to_string()
                .with(Color::Cyan)
                .attribute(crossterm::style::Attribute::Bold),
            entry
                .hot_streak
                .then(|| "🔥")
                .unwrap_or("❄")
                .attribute(crossterm::style::Attribute::Underlined)
        );
        write!(f, "{}", text)
    }
}

impl With for LeagueEntryDisplay {
    type Struct = LeagueEntry;
    fn with(entry: LeagueEntry) -> LeagueEntryDisplay {
        LeagueEntryDisplay(entry)
    }
}
impl DisplayToText<LeagueEntryDisplay> for LeagueEntryDisplay {}

#[derive(Clone)]
pub struct ChampionMasteryDisplay(pub ChampionMastery);

impl DisplayToText<ChampionMasteryDisplay> for ChampionMasteryDisplay {}

impl Display for ChampionMasteryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = &self.0;

        let ch_id = format!("{}", entry.champion_id.name().unwrap_or("UNKNOWN").green());
        let ch_points = format!("{}", entry.champion_points.to_string().yellow().to_string()); //.with(Color::Yellow),
        let ch_level = format!(
            "{}",
            entry.champion_level.to_string().cyan().bold().to_string()
        );
        let text = format!(
            "{} {} ({})",
            padding(ch_id, Pad::Left, 30, b' '),
            padding(ch_points, Pad::Left, 23, b' '),
            ch_level
        );

        write!(f, "{}", text)
    }
}

impl With for ChampionMasteryDisplay {
    type Struct = ChampionMastery;
    fn with(entry: ChampionMastery) -> ChampionMasteryDisplay {
        ChampionMasteryDisplay(entry)
    }
}

#[derive(Clone)]
pub struct MatchDisplay(pub Match);

impl DisplayToText<MatchDisplay> for MatchDisplay {}

impl Display for MatchDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = Vec::default();
        let entry = &self.0.info;

        let team_red = entry
            .participants
            .iter()
            .filter(|f| f.team_id == Team::RED)
            .collect::<Vec<_>>();
        let team_blue = entry
            .participants
            .iter()
            .filter(|f| f.team_id == Team::BLUE)
            .collect::<Vec<_>>();

        lines.push(format!(
            "{}",
            entry
                .game_mode
                .to_string()
                .with(Color::Reset)
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined),
        ));
        lines.push(format!(
            "{}",
            "Team Red"
                .with(Color::Red)
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined)
        ));

        lines.append(
            &mut team_red
                .iter()
                .enumerate()
                .map(|f| {
                    let (_, r) = f;
                    let kda = format!("{}/{}/{}", r.kills, r.deaths, r.assists);
                    format!(
                        "      {} {} {}  |  {} {}",
                        padding(
                            r.team_position.clone().with(Color::Cyan).to_string(),
                            Pad::Left,
                            25,
                            b' '
                        ),
                        padding(
                            r.summoner_name.clone().with(Color::Red).to_string(),
                            Pad::Left,
                            35,
                            b' '
                        ),
                        padding(
                            r.champion_name.clone().with(Color::Yellow).to_string(),
                            Pad::Left,
                            30,
                            b' '
                        ),
                        padding(kda.with(Color::Green).to_string(), Pad::Left, 20, b' '),
                        padding(
                            r.total_minions_killed
                                .to_string()
                                .with(Color::Cyan)
                                .to_string(),
                            Pad::Left,
                            0,
                            b' '
                        ),
                    )
                })
                .collect::<Vec<String>>(),
        );
        lines.push(format!(
            "{}",
            "Team Blue"
                .with(Color::Blue)
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined)
        ));

        lines.append(
            &mut team_blue
                .iter()
                .enumerate()
                .map(|f| {
                    let (_, b) = f;
                    let kda = format!("{}/{}/{}", b.kills, b.deaths, b.assists);
                    format!(
                        "      {} {} {}  |  {} {}",
                        padding(
                            b.team_position.clone().with(Color::Cyan).to_string(),
                            Pad::Left,
                            25,
                            b' '
                        ),
                        padding(
                            b.summoner_name.clone().with(Color::Red).to_string(),
                            Pad::Left,
                            35,
                            b' '
                        ),
                        padding(
                            b.champion_name.clone().with(Color::Yellow).to_string(),
                            Pad::Left,
                            30,
                            b' '
                        ),
                        padding(kda.with(Color::Green).to_string(), Pad::Left, 20, b' '),
                        padding(
                            b.total_minions_killed
                                .to_string()
                                .with(Color::Cyan)
                                .to_string(),
                            Pad::Left,
                            0,
                            b' '
                        ),
                    )
                })
                .collect::<Vec<String>>(),
        );

        write!(f, "{}", lines.join("\n"))
    }
}

impl With for MatchDisplay {
    type Struct = Match;
    fn with(entry: Match) -> MatchDisplay {
        MatchDisplay(entry)
    }
}

impl MatchDisplay {
    pub fn list(&mut self, name: String) -> Vec<Line> {
        let text = self
            .0
            .info
            .participants
            .iter()
            .map(|f| {
                if f.summoner_name == name && f.win {
                    Span::styled(
                        "won",
                        Style::default()
                            .fg(style::Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(
                        "lose",
                        Style::default()
                            .fg(style::Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                }
            })
            .collect::<Vec<Span>>();
        vec![Line::from(text)]
    }
}

/// color is Some((focused, unfocesed)) color
pub fn border_color(curr: Window, focused: Option<Window>, colors: Option<(ratatui::style::Color, ratatui::style::Color)>) -> Style {
    let mut color = (ratatui::style::Color::Black, ratatui::style::Color::White);
    if let Some(c) = colors {
        color = c;
    }
    if let Some(focused) = focused {
        if curr == focused {
            return Style::default().fg(color.0);
        }
    }
    Style::default().fg(color.1)
}
