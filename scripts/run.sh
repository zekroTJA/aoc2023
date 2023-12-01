[ "$RELEASE" == "true" ] && RELEASE="--release" || RELEASE=""

function run_day {
    [ "$RELEASE" == "true" ] && TARGET=release || TARGET=debug
    cargo build "$RELEASE" -p "$1"
    "./target/$TARGET/$1" "$2"
}

if [ -z "$1" ] || [ "$1" == "--test" ]; then
    current_day=$(find . -maxdepth 1 -type d -name "day-*" | head -1)
    run_day "$current_day" "$1"
else
    # cargo run -p $(printf "day-%02d" $1)
    run_day "$(printf "day-%02d" "$1")" "$2"
fi