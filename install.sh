#! /bin/bash

user="$(whoami)"

path="/home/$user/.local/bin"

if [[ -z "$(which go)" ]]; then
        echo -e "\033[0;31mGolang not installed\033[0m"
        exit 1
fi

#update bin 
go build main.go
cp main watcher

sudo cp "watcher" "$path/watcher"

echo -e "\033[1;32mInstalled watcher at {$path}\033[0m"
