query=$1

search_dirs=(
    ~/
    ~/Documents
    ~/.config
)

search_path_params=""
for dir in $search_dirs; do
    search_path_params+="--search-path $dir "
done


if [ -z "$query" ]
then
    paths=$($HOME/.local/bin/mru list)
else
    paths=$(/opt/homebrew/bin/fd -t d -t f --max-results 10 ${=search_path_params} -g "*$query*" -0 |
        /usr/bin/xargs -0 $HOME/.local/bin/mru sort)
fi

echo $paths | /opt/homebrew/bin/jq -nR '{
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