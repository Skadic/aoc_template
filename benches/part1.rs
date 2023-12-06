fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    {{project-name}}::part1::process().unwrap();
}
