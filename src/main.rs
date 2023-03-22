use clap::{value_parser, Arg, ArgAction, Command};
use crossterm::{
    execute,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::{env, io::stdout, path::PathBuf};

struct Config {
    dir: PathBuf,
    list: bool,
    all: bool,
    depth: usize,
}

enum TextFormat {
    List,
    NoList,
}

fn format_text(text: String, format: &TextFormat) {
    match format {
        TextFormat::List => println!("{} ", text),
        TextFormat::NoList => print!("{} ", text),
    }
}

fn parse_config(_args: &[String]) -> Config {
    let matches = Command::new("rls")
        .version("1.0")
        .author("Your Name")
        .about("List files and directories.")
        .arg(Arg::new("path").value_parser(value_parser!(PathBuf)))
        .arg(
            Arg::new("long")
                .long("long")
                .short('l')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("depth")
            .long("depth")
            .short('d')
            .value_parser(value_parser!(usize))
            .default_value("0")
        )
        .get_matches();

    let current_dir = env::current_dir().ok().unwrap();
    let mut list = matches.get_flag("long");
    let all = matches.get_flag("all");
    if all {
        // turn on list flag if all flag ist active
        list = true
    };

    // das muss schöner gehen
    let given_path = matches.try_get_one::<PathBuf>("path").ok().unwrap();
    let mut dir = current_dir;

    // keine ahnung was hier abgeht
    if let Some(t) = given_path {
        dir = t.to_path_buf()
    };

    let depth = *(matches.try_get_one::<usize>("depth").ok().unwrap().unwrap());

    /*if t.is_some() {
        dir = t.unwrap().to_path_buf();
    }*/

    Config { dir, list, all, depth }
}

fn print_file_tree(files: &Vec<rls::Entry>, config: &Config, intend: Option<usize>) {
    let mut formati = TextFormat::NoList;

    if config.list {
        formati = TextFormat::List
    }

    // das muss schöner gehen
    for file in files {
        let mut filename = file.filename.clone();
        if file.is_dir {
            execute!(stdout(), SetForegroundColor(Color::Blue)).ok();
            filename.push('/');
        }

        for _i in 0..intend.unwrap_or(0) {
            print!("  ");
        }

        if config.all {
            print!("{} \t", rls::get_file_permission_string(&file));
        }

        format_text(filename, &formati);
        execute!(stdout(), ResetColor).ok();
        if file.entries.is_some() {
            print_file_tree(
                file.entries.as_ref().unwrap(),
                &config,
                Some(intend.unwrap_or(0) + 1),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut files = rls::read_dir(&config.dir, Some(config.depth)).ok().unwrap();
    files.sort();

    print_file_tree(&files, &config, None);
}
