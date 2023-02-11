use clap::Parser;
use std::io::stdin;

mod cli;
mod grouper;
mod splitters;
mod util;

use grouper::Grouper;
use splitters::WhiteSpaceSplitter;
use util::SplitLine;

/// Wrapper function for main logic, to enable testing
fn app<S: Iterator<Item = String> + 'static>(
    args: cli::Args,
    source: S,
) -> Grouper<Box<dyn Iterator<Item = SplitLine>>> {
    // let boxed_source = Box::new(source);
    let boxed_source: Box<dyn Iterator<Item = SplitLine>> = match args.mode {
        cli::Mode::Quoted => {
            todo!()
        }
        cli::Mode::WhiteSpace => {
            let wss = WhiteSpaceSplitter::from(source);
            Box::new(wss)
        }
    };
    Grouper::from(boxed_source)
}

#[cfg(not(tarpaulin_include))]
fn main() {
    // let mut last_key = None;
    let line_source = stdin().lines().filter_map(|l| l.ok());

    let args = cli::Args::parse();
    app(args, line_source).for_each(|l| println!("{l}"));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_app() {
        let input = vec![
            "a 1".to_owned(),
            "a 2".to_owned(),
            "a 3".to_owned(),
            "b 1".to_owned(),
            "a 2".to_owned(),
            "c 3".to_owned(),
        ];
        let args = cli::Args {
            mode: cli::Mode::WhiteSpace,
        };
        let mut output = app(args, input.into_iter()).map(|l| format!("{l}"));

        assert_eq!(output.next(), Some("> a".to_owned()));
        assert_eq!(output.next(), Some("* 1".to_owned()));
        assert_eq!(output.next(), Some("* 2".to_owned()));
        assert_eq!(output.next(), Some("* 3".to_owned()));
        assert_eq!(output.next(), Some("> b".to_owned()));
        assert_eq!(output.next(), Some("* 1".to_owned()));
        assert_eq!(output.next(), Some("> a".to_owned()));
        assert_eq!(output.next(), Some("* 2".to_owned()));
        assert_eq!(output.next(), Some("> c".to_owned()));
        assert_eq!(output.next(), Some("* 3".to_owned()));
    }
}
