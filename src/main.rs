use anyhow::{anyhow, Context, Result};
use std::io::{Lines, StdinLock};

fn main() {
    if false {
        // this is untouched comment, this is untouched comment
        // this is untouched comment
    }
    print!("main");
    bar();
}

async fn foo(val: Option<i32>) {
    let Some(x) = val else { return };
}

#[test]
fn test() {
    main();
}

trait InputHandle {
    type Input<'a>
    where
        Self: 'a;
    fn get_input<'a>(&self) -> Self::Input<'a>;
    fn process_input<'a, T>(&self, f: impl FnOnce(Self::Input<'a>) -> Result<T>) -> Result<T>
    where
        Self: 'a;
}

struct StdioInput;
impl InputHandle for StdioInput {
    type Input<'a> = Lines<StdinLock<'a>>;

    fn get_input<'a>(&self) -> Self::Input<'a> {
        std::io::stdin().lines()
    }

    fn process_input<'a, T>(&self, f: impl FnOnce(Self::Input<'a>) -> Result<T>) -> Result<T>
    where
        Self: 'a,
    {
        f(self.get_input())
    }
}

fn bar() -> Result<()> {
    // cc = 1 (default)
    let index: usize = StdioInput.process_input(|mut input| {
        // cc = 3 (?, ?)
        str::parse(input.next().context("no input from stdin")??.as_str()).context("parse failed")
    })?; // cc = 4 (?)

    // cc = 5 (for)
    for i in 0..index {
        let fizzbuzz = match i % 15 {
            // cc = 6 (match_arm 0)
            0 => "fizzbuzz".into(),
            // cc = 9 (match_arm 3 | 6 | 9)
            3 | 6 | 9 | 12 => "fizz".into(),
            // cc = 11 (match_arm 5 | 10)
            5 | 10 => "buzz".into(),
            x => format!("{x}"),
        };
        println!("{i}: {fizzbuzz}");
    }

    Ok(())
}

// Lorem ipsum dolor sit amet, consectetur adipiscing elit.
