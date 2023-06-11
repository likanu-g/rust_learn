
use clap::{App,Arg};
fn main() {
    let args = App::new("grep-lite")
    .version("0.1")
    .about("search for patterns")
    .arg(Arg::with_name("patterns"))
      .help("the patterns to search for")
      //.takes_value(true)
      //.required(true)
    .get_matches();
    

}
