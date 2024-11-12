use std::path::PathBuf;

const HELP: &str = "\
set-cosmic-theme

USAGE:
  set-cosmic-theme [--dark] [--light] THEME

FLAGS:
  -h, --help        Print help information.
  --dark, --light   Set dark or light theme. If neither or both are specified,
                    sets both themes with the same file.

ARGS:
  <THEME>           The theme file to set. Should be a RON theme file exported
                    from cosmic-settings or in the same format.
";

struct Args {
    set_dark: bool,
    set_light: bool,
    theme_file: PathBuf,
}

fn parse_args() -> Args {
    let mut args = std::env::args_os().skip(1).collect::<Vec<_>>();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let set_dark = args
        .iter()
        .position(|arg| arg == "--dark")
        .inspect(|i| {
            args.swap_remove(*i);
        })
        .is_some();
    let set_light = args
        .iter()
        .position(|arg| arg == "--light")
        .inspect(|i| {
            args.swap_remove(*i);
        })
        .is_some();
    let theme_file = match args.as_slice() {
        [path] => {
            let path = PathBuf::from(path);
            if !path.is_file() {
                eprintln!(
                    "The path {} does not exist, is not a file, \
                    or could not be accessed.",
                    path.display()
                );
                std::process::exit(1);
            }
            path
        }
        [] => {
            eprintln!(
                "You mut provide a path to a theme file. \
                Run 'set-cosmic-theme --help' for more information."
            );
            std::process::exit(1);
        }
        _ => {
            eprintln!(
                "Too many arguments given. \
                Run 'set-cosmic-theme --help' for more information."
            );
            std::process::exit(1);
        }
    };

    if set_dark || set_light {
        Args {
            set_dark,
            set_light,
            theme_file,
        }
    } else {
        Args {
            set_dark: true,
            set_light: true,
            theme_file,
        }
    }
}

fn main() {
    let args = parse_args();
}
