fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    {{project-name}}::part2::process().unwrap();
}

