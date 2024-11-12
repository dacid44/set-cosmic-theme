use anyhow::{Context, Result};
use cosmic::{
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme::Theme,
    cosmic_theme::ThemeBuilder,
};
use std::{fs, path::PathBuf};

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

fn set_theme(theme_builder: &ThemeBuilder, dark: bool) -> Result<()> {
    let builder_config = (if dark {
        ThemeBuilder::dark_config()
    } else {
        ThemeBuilder::light_config()
    })
    .context("Could not get Cosmic theme builder config")?;
    let theme_config = (if dark {
        Theme::dark_config()
    } else {
        Theme::light_config()
    })
    .context("Could not get Cosmic theme config")?;

    theme_builder
        .clone()
        .build()
        .write_entry(&theme_config)
        .context("Failed to write theme")?;

    theme_builder
        .write_entry(&builder_config)
        .context("Failed to write theme builder")?;

    Ok(())
}

fn main() -> Result<()> {
    // Parse args
    let args = parse_args();

    // Read file
    let theme_str = fs::read_to_string(args.theme_file).context("Failed to read theme file")?;
    let theme_builder: ThemeBuilder =
        ron::de::from_str(&theme_str).context("Failed to parse theme file")?;

    // Set Cosmic theme(s)
    if args.set_dark {
        set_theme(&theme_builder, true).context("Failed to set Cosmic dark theme")?;
    }
    if args.set_light {
        set_theme(&theme_builder, false).context("Failed to set Cosmic light theme")?;
    }

    Ok(())
}
