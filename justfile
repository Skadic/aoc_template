today := `date "+%-d"`
current_year := `date "+%Y"`

# List all targets
_list:
  just --list --justfile {{justfile()}}

# Run part
run nr: 
  cargo test --release part{{nr}} -- --nocapture
[private]
alias r:=run

alias part:=run
[private]
alias p:=run

# Benchmark part
bench nr:
  cargo bench --bench part{{nr}}
[private]
alias b:=bench

# Download the input for a given day and year.
inputs day=today year=current_year:
  curl -o input.txt --cookie "session=$AOC_SESSION" https://adventofcode.com/{{year}}/day/{{day}}/input
[private]
alias i:=inputs

alias load:=inputs
[private]
alias l:=inputs
