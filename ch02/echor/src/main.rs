use clap::{command, crate_authors, Arg, ArgAction, ArgMatches};

fn main() {
  let args: ArgMatches = command!()
    .author(crate_authors!("\n"))
    .arg(
      Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .action(ArgAction::Append) // accept multiple values
        .required(true),
    )
    .arg(
      Arg::new("omit_newline")
        .value_name("")
        .short('n')
        .help("Do not print newline")
        .action(clap::ArgAction::SetTrue),
    )
    .get_matches();

  let text: Vec<String> = args
    .get_many::<String>("text")
    .unwrap_or_default()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();

  let ending: &str = if args.get_one::<bool>("omit_newline").copied().unwrap() {
    ""
  } else {
    "\n"
  };

  print!("{}{}", text.join(" "), ending);
}
