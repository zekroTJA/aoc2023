function run_day {
    if [ "$RELEASE" == "true" ]; then 
        TARGET=release
        RELEASE_FLAG="--release"
    else 
        TARGET=debug
    fi
    
    cargo build $RELEASE_FLAG -p "$1"

    if [ -f "$1/test_input.txt" ] && [ -z "$2" ]; then
        "./target/$TARGET/$1" --test
    fi

    printf "\033[35m"
    "./target/$TARGET/$1" "$2"
    printf "\033[0m"
}

if [ -z "$1" ] || [ "$1" == "--test" ]; then
    current_day=$(find . -maxdepth 1 -type d -name "day-*" | sort | tail -1 | cut -c 3-)
    run_day "$current_day" "$1"
else
    # cargo run -p $(printf "day-%02d" $1)
    run_day "$(printf "day-%02d" "$1")" "$2"
fi