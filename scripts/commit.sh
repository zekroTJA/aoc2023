current_day=$(find . -maxdepth 1 -type d -name "day-*" | sort | tail -1 | cut -c 3-)

# ---- CHECKS ----

function abort {
    echo "Abort: $1"
    exit 1
}

grep "TODO:" "$current_day/src/main.rs" && {
    abort "main.rs still contains TODOs!"
}

[ "$(wc -c < "$current_day/challenge.txt")" == "0" ] && {
    abort "challenge.txt is empty!"
}

cargo build --release -p "$current_day" || {
    abort "Build fails!"
}

./target/release/"$current_day" || {
    abort "Run fails!"
}

cargo test -p "$current_day" || {
    abort "Unit Tests fail!"
}

# ---- CLEAN UP ----

rm -f "$current_day/test_input.txt"

# ---- COMMIT & PUSH ----

current_day=$(( ${current_day/*-} ))

git add .
git commit -m "add day $current_day solution"
git push