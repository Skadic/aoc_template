#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    // Edit here
    todo!("part 1")
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
