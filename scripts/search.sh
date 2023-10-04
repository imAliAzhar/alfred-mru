query=$1

paths=(
    ~/
    ~/Documents
    ~/.config
)

search_path_params=""
for path in $paths; do
    search_path_params+="--search-path $path "
done

/opt/homebrew/bin/fd -t d -t f --max-results 10 ${=search_path_params} -g "*$query*" -0 |
    /usr/bin/xargs -0 /$HOME/.local/bin/mru sort |
    /opt/homebrew/bin/jq -nR '{
        "items": [
            inputs |
            rtrimstr("/") as $path |
            $path |
            split("/") |
            last as $name |
            {
                "uid": $path,
                "title": $name,
                "arg": $path,
                "match": $name,
                "icon": {
                    type: "fileicon",
                    path: $path
                }
            }
        ]
    }'