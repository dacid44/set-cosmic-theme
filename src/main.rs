use anyhow::{Context, Result};
use cosmic_config::{self, CosmicConfigEntry};
use cosmic_theme::{Theme, ThemeBuilder};
use std::{fs, path::PathBuf};

const HELP: &str = "\
set-cosmic-theme

USAGE:
  set-cosmic-theme [MODES] {THEME | default}

FLAGS:
  -h, --help    Print help information.

MODES:
  --dark        Set the COSMIC dark theme.
  --light       Set the COSMIC light theme.
                If neither --dark or --light are specified, which theme to set
                is determined based on the theme file's palette, or if setting
                the default theme, both themes are set.
  --gtk4        Set the GTK4 user CSS based on the theme file.

ARGS:
  <THEME>       The theme file to set. Should be a RON theme file exported from
                cosmic-settings or in the same format.
  default       Set the theme(s) back to the default for the given mode.
                Incompatible with --gtk4.
";

struct Args {
    set_dark: bool,
    set_light: bool,
    set_gtk4: bool,
    theme_file: Option<PathBuf>,
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
    let set_gtk4 = args
        .iter()
        .position(|arg| arg == "--gtk4")
        .inspect(|i| {
            args.swap_remove(*i);
        })
        .is_some();
    let theme_file = match args.as_slice() {
        [arg] if arg == "default" => {
            if set_gtk4 {
                eprintln!("You may not set the default theme when --gtk4 is specified.");
                std::process::exit(1);
            }
            None
        }
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
            Some(path)
        }
        [] => {
            eprintln!(
                "You must provide a path to a theme file, or specify 'default'. \
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

    Args {
        set_dark,
        set_light,
        set_gtk4,
        theme_file,
    }
}

fn set_theme(theme_builder: Option<&ThemeBuilder>, dark: bool) -> Result<()> {
    let theme_builder = match (theme_builder, dark) {
        (Some(t), _) => t,
        (None, true) => &ThemeBuilder::dark(),
        (None, false) => &ThemeBuilder::light(),
    };

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
    let mut args = parse_args();

    // Read file
    let theme_builder = match args.theme_file {
        Some(file) => {
            let theme_str = fs::read_to_string(file).context("Failed to read theme file")?;
            Some(
                ron::de::from_str::<ThemeBuilder>(&theme_str)
                    .context("Failed to parse theme file")?,
            )
        }
        None => None,
    };

    // If theme mode was unspecified, set it based on the given theme file
    if !args.set_dark && !args.set_light {
        match theme_builder.as_ref().map(|t| t.palette.is_dark()) {
            Some(true) => args.set_dark = true,
            Some(false) => args.set_light = true,
            None => {
                args.set_dark = true;
                args.set_light = true;
            }
        }
    }

    // Set Cosmic theme(s)
    if args.set_dark {
        set_theme(theme_builder.as_ref(), true).context("Failed to set Cosmic dark theme")?;
    }
    if args.set_light {
        set_theme(theme_builder.as_ref(), false).context("Failed to set Cosmic light theme")?;
    }

    // Set gtk4 theme
    if let Some(theme_builder) = theme_builder.filter(|_| args.set_gtk4) {
        theme_builder
            .build()
            .write_gtk4()
            .context("Failed to set GTK4 user CSS")?;
    }

    Ok(())
}
