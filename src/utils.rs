#[macro_export]
macro_rules! err_print  {
    ($($token:tt)*) => {
        format!("{}",  $(stringify!($token).with(crossterm::style::Color::Red),),* )
    };
}


pub fn  print_help() {
    const TEXT: &str = r###"
            Watcher 
______________________________

Usage: Watcher [name] [region] [flags]

Flags:
------------------------------
NONE            lunches Tui 
-h | --help     prints help
-s | --summoner searches for summoner
-r | --rank     get's summoner rank 
-m | --mastery  get's first 10 highest champions mastery's
-g | --game     -g 0..20 get's game from 20 games
        "###;
}
