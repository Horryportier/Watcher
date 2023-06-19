use std::env::args;

use args::Args;
use ui::ui::ui;
use utils::print_help ;

mod api;
mod display;
mod ui;
mod utils;
mod args;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();

    let api_key: &str; 
    match option_env!("RGAPI_KEY") {
        None => {
            println!("RGAPI_KEY is empty check if its exported");
            return Ok(());
        }
        Some(key) => api_key = key,
    }

    if args.len() == 1 {
        if let Err(err) = ui(api_key).await {
            println!("ERR: {}", err)
        }
        return Ok(());
    };


    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            print_help();
            return Ok(());
        }
    }

    let a = Args::new(args[1..].to_vec());
    let _ = a.execute(api_key).await;
        
    Ok(())
}
