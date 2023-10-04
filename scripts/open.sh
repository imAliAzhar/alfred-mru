#!/bin/zsh

$HOME/.local/bin/mru cache $1

projects_dir="$HOME/Projects"
config_dir="$HOME/.config"

mime_type=$(/usr/bin/file -b --mime-type "$1")

if [[ $mime_type == text/* ]]; then
    is_file=true
else
    is_file=false
fi

if $is_file; then
    /usr/local/bin/code "$1"
    exit
fi

if [[ $1 == $projects_dir* || $1 == $config_dir* ]]; then
    /usr/local/bin/code -n "$1"
    exit
fi

/usr/bin/open $path