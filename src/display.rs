use core::fmt;
use std::fmt::Display;

use crossterm::style::{Color, Stylize};
use riven::{
    consts::{Team, Tier},
    models::{
        champion_mastery_v4::ChampionMastery, league_v4::LeagueEntry, match_v5::Match,
        summoner_v4::Summoner,
    },
};
use tui::{
    style::{self, Modifier, Style},
    text::{Span, Spans},
};

use crate::ui::app::Window;

#[macro_export]
macro_rules! no_data {
    () => {
        vec![Spans::from("no data")]
    };
}

pub trait VecSpans {
    fn spans(&self) -> Vec<Spans>;
}

pub trait With {
    type Struct;
    fn with(entry: Self::Struct) -> Self;
}

#[derive(Clone)]
pub struct SummonerDisplay(pub Summoner);

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
impl VecSpans for SummonerDisplay {
    fn spans(&self) -> Vec<Spans> {
        let entry = &self.0;
        vec![Spans::from(vec![
            Span::from("  "),
            Span::styled(
                entry.name.clone(),
                Style::default()
                    .fg(style::Color::Green)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
            Span::from("    "),
            Span::styled("lvl", Style::default().fg(style::Color::Reset)),
            Span::from(":"),
            Span::styled(
                entry.summoner_level.to_string(),
                Style::default().fg(style::Color::Yellow),
            ),
            Span::from("\n"),
        ])]
    }
}

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
impl VecSpans for LeagueEntryDisplay {
    fn spans(&self) -> Vec<Spans> {
        let entry = &self.0;
        vec![
            Spans::from(vec![
                Span::from("  "),
                Span::styled(
                    entry.queue_type.to_string(),
                    Style::default()
                        .fg(style::Color::Gray)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            ]),
            Spans::from(vec![
                Span::from("    "),
                Span::styled(
                    entry.tier.unwrap_or(Tier::UNRANKED).to_string(),
                    Style::default().fg(style::Color::Yellow),
                ),
                Span::from("  "),
                Span::styled(
                    entry.rank.unwrap_or(riven::consts::Division::I).to_string(),
                    Style::default().fg(style::Color::Blue),
                ),
            ]),
            Spans::from(vec![
                Span::from("    "),
                Span::styled(
                    entry.wins.to_string(),
                    Style::default().fg(style::Color::Green),
                ),
                Span::from("/"),
                Span::styled(
                    entry.losses.to_string(),
                    Style::default().fg(style::Color::Red),
                ),
                Span::from("  "),
                Span::styled(
                    (entry.wins * 100 / (entry.wins + entry.losses)).to_string(),
                    Style::default().fg(style::Color::Cyan),
                ),
                Span::from("%"),
            ]),
            Spans::from(vec![
                Span::from("    "),
                entry
                    .hot_streak
                    .then(|| Span::from("🔥"))
                    .unwrap_or(Span::from("❄")),
                Span::from("\n"),
            ]),
        ]
    }
}

#[derive(Clone)]
pub struct ChampionMasteryDisplay(pub ChampionMastery);

impl Display for ChampionMasteryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = &self.0;
        let text = format!(
            "{: <9}  {: >7}  ({})",
            entry
                .champion_id
                .name()
                .unwrap_or("UNKNOWN")
                .with(Color::Green),
            entry.champion_points.to_string().with(Color::Yellow),
            entry
                .champion_level
                .to_string()
                .with(Color::Cyan)
                .attribute(crossterm::style::Attribute::Bold)
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

impl VecSpans for ChampionMasteryDisplay {
    fn spans(&self) -> Vec<Spans> {
        let entry = &self.0;
        vec![Spans::from(vec![
            Span::from("  "),
            Span::styled(
                entry.champion_id.name().unwrap_or("UNKNOWN"),
                Style::default()
                    .fg(style::Color::Green)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
            Span::from("    "),
            Span::styled(
                entry.champion_points.to_string(),
                Style::default().fg(style::Color::Yellow),
            ),
            Span::from("  ("),
            Span::styled(
                entry.champion_level.to_string(),
                Style::default().fg(style::Color::Cyan),
            ),
            Span::from(")"),
            Span::from("\n"),
        ])]
    }
}

#[derive(Clone)]
pub struct MatchDisplay(pub Match);

impl Display for MatchDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entry = self.0.info.clone();
        let team_red = entry.teams.pop().unwrap();
        let team_blue = entry.teams.pop().unwrap();
        let text = format!(
            r###"
{}

blue team: {}
bans: {}
Players:
    {}

red team: {}
bans: {}
Players:
    {}
            "###,
            entry
                .queue_id
                .to_string()
                .with(Color::Reset)
                .attribute(crossterm::style::Attribute::Bold),
            team_blue
                .win
                .then(|| "win".with(Color::Green))
                .unwrap_or("lose".with(Color::Red)),
            team_blue
                .bans
                .iter()
                .map(|f| format!(
                    "{}",
                    f.champion_id
                        .name()
                        .unwrap_or("UNKNOWN")
                        .with(Color::Yellow)
                ))
                .collect::<Vec<_>>()
                .join(", "),
            entry
                .participants
                .iter()
                .filter(|f| f.team_id == Team::BLUE)
                .map(|f| format!(
                    "{} {} {}",
                    f.clone().role.with(Color::Cyan),
                    f.clone().champion_name.with(Color::Magenta),
                    f.clone().summoner_name.with(Color::Yellow)
                ))
                .collect::<Vec<_>>()
                .join("\n   "),
            team_red
                .win
                .then(|| "win".with(Color::Green))
                .unwrap_or("lose".with(Color::Red)),
            team_red
                .bans
                .iter()
                .map(|f| format!(
                    "{}",
                    f.champion_id
                        .name()
                        .unwrap_or("UNKNOWN")
                        .with(Color::Yellow)
                ))
                .collect::<Vec<_>>()
                .join(", "),
            entry
                .participants
                .iter()
                .filter(|f| f.team_id == Team::RED)
                .map(|f| format!(
                    "{} {} {}",
                    f.clone().role.with(Color::Cyan),
                    f.clone().champion_name.with(Color::Magenta),
                    f.clone().summoner_name.with(Color::Yellow)
                ))
                .collect::<Vec<_>>()
                .join("\n   "),
        );
        write!(f, "{}", text)
    }
}

impl With for MatchDisplay {
    type Struct = Match;
    fn with(entry: Match) -> MatchDisplay {
        MatchDisplay(entry)
    }
}

impl MatchDisplay {
    pub fn list(&mut self, name: String) -> Vec<Spans> {
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
        vec![Spans::from(text)]
    }
}

impl VecSpans for MatchDisplay {
    fn spans(&self) -> Vec<Spans> {
        let red: Style = Style::default().fg(style::Color::Red);
        let blue: Style = Style::default().fg(style::Color::Blue);
        let green: Style = Style::default().fg(style::Color::Green);
        let yellow: Style = Style::default().fg(style::Color::Yellow);
        let cyan: Style = Style::default().fg(style::Color::Cyan);
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

        let mut spans = vec![Spans::from(vec![Span::styled(
            format!("{}", entry.game_mode),
            Style::default()
                .fg(style::Color::Reset)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )])];

        spans.append(
            vec![Spans::from(vec![Span::styled(
                format!("{}", "Team Red"),
                Style::default()
                    .fg(style::Color::Red)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )])]
            .as_mut(),
        );

        spans.append(
            &mut team_red
                .iter()
                .enumerate()
                .map(|f| {
                    let (_, r) = f;
                    let kda = format!("{}/{}/{}", r.kills, r.deaths, r.assists);

                    Spans::from(vec![
                        Span::from("    "),
                        Span::styled(format!("{: <7}", r.team_position), cyan),
                        Span::from("  "),
                        Span::styled(format!("{: <16}", r.summoner_name), red),
                        Span::from("  "),
                        Span::styled(format!("{: <12}", r.champion_name), yellow),
                        Span::from("  |  "),
                        Span::styled(format!("{: <8}", kda), green),
                        Span::from("  "),
                        Span::styled(format!("{: <4}", r.total_minions_killed), cyan),
                    ])
                })
                .collect::<Vec<Spans>>(),
        );
        spans.append(
            vec![Spans::from(vec![Span::styled(
                format!("{}", "Team Blue"),
                Style::default()
                    .fg(style::Color::Blue)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )])]
            .as_mut(),
        );

        spans.append(
            &mut team_blue
                .iter()
                .enumerate()
                .map(|f| {
                    let (_, b) = f;
                    let kda = format!("{}/{}/{}", b.kills, b.deaths, b.assists);

                    Spans::from(vec![
                        Span::from("    "),
                        Span::styled(format!("{: <7}", b.team_position), cyan),
                        Span::from("  "),
                        Span::styled(format!("{: <16}", b.summoner_name), blue),
                        Span::from("  "),
                        Span::styled(format!("{: <12}", b.champion_name), yellow),
                        Span::from("  |  "),
                        Span::styled(format!("{: <8}", kda), green),
                        Span::from("  "),
                        Span::styled(format!("{: <4}", b.total_minions_killed), cyan),
                    ])
                })
                .collect::<Vec<Spans>>(),
        );

        spans
    }
}

pub fn border_color(curr: Window, focused: Option<Window>) -> Style {
    if let Some(focused) = focused {
        if curr == focused {
            return Style::default().fg(style::Color::Black);
        }
    }
    Style::default().fg(style::Color::White)
}
