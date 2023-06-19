# Watcher
[![Rust](https://github.com/Horryportier/Watcher/actions/workflows/rust.yml/badge.svg)](https://github.com/Horryportier/Watcher/actions/workflows/rust.yml)

**Leauge of Legends terminal cil/dashboard**

Watcher is an basic CLI/TUI tool that retrieves summoner data from Riot API.

<img src="https://raw.githubusercontent.com/Horryportier/Watcher/main/tui.png">

## Installation 

You need Riot API key that you can get at https://developer.riotgames.com/ .
When you get export it as env variable  as `RGAPI_KEY=your_key`

Using cargo 
```bash
cargo install --git https://github.com/Horryportier/Watcher
```
compile from GitHub
```bash
git clone --git https://github.com/Horryportier/Watcher
cd Watcher    
cargo install 
```
*as cargo package maybe in the future*

## Usage
```bash

            Watcher 
______________________________

Usage: Watcher [name] [region] [flags]


# Regions:
["kr", "ru", "br", "jp", "la1", "la2", "na", "oce", "ph", "sg", "th", "tr", "tw", "eune", "euw"]
Flags:
------------------------------
NONE     lunches Tui 
-h | --help     prints help
-s | --summoner searches for summoner
-r | --rank     get's summoner rank 
-m | --mastery  get's first 10 highest champions mastery's
-g | --game     -g 0..20 get's game from 20 games
```
