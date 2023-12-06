#[tracing::instrument(name="part2", parent=None)]
pub fn process() -> miette::Result<()> {
    // Edit here
    todo!("part 2")
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
