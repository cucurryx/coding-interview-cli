extern crate serde;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: cinterview list
       cinterview submit [--exam | --test] <num>...
       
Options:
    -h, --help         Show this message.
    --version          Show the version of cinterview.
    --exam             Submit in exam mode, no debug message.
    --test             Submit in test mode, have debug message.
";

#[derive(Debug, Deserialize)]
pub struct CinterviewOptions {
    pub arg_num: Vec<u32>,
    pub flag_exam: bool,
    pub flag_test: bool,
    pub flag_help: bool,
    pub flag_version: bool,
    pub cmd_list: bool,
    pub cmd_submit: bool
}

pub fn parse_args() -> CinterviewOptions {
    let args: CinterviewOptions = Docopt::new(USAGE)
                                .and_then(|d| d.deserialize())
                                .unwrap_or_else(|e| e.exit());
    args 
}

#[test]
fn test_basic() {

}

#[test]
fn test_submit() {
    let argv = || vec!["cinterview", "submit", "--test", "12", "1"];
    let args: CinterviewOptions = Docopt::new(USAGE)
                                .and_then(|d| d.argv(argv().into_iter()).deserialize())
                                .unwrap_or_else(|e| e.exit());

    assert_eq!(args.flag_exam, false);
    assert_eq!(args.flag_test, true);
    assert_eq!(args.arg_num, vec![12, 1]);
}
