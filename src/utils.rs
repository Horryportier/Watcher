use core::fmt;

use crossterm::style::{Color, Stylize};
use riven::{consts::Tier, models::league_v4::LeagueEntry};
use tui::{
    style::{self, Modifier, Style},
    text::{Span, Spans},
};

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

impl LeagueEntryDisplay {
    pub fn spans(&self) -> Vec<Spans> {
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
