use crate::{SAMPLE, parse_input};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(SAMPLE);
    // Edit here
    todo!("part 1")
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
