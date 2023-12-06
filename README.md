# aoc_template
A Rust template for Advent of Code. You can clone this with `cargo-generate` using:
```bash
cargo generate Skadic/aoc_template
```

You have several [just]() targets to run parts and benchmark them:
```bash
Available recipes:
    bench nr # Benchmark part
    inputs day=today year=current_year # Download the input for a given day and year.
    load day=today year=current_year # alias for `inputs`
    part nr  # Run part
```
All of the targets have aliases equal to their first letter.

For example, to run part 1, use:
```bash
just part 1
```

To benchmark part 1, use:
```bash
just bench 1
```
