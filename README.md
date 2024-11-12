# set-cosmic-theme

This is a small utility to programmatically set a COSMIC Desktop theme from a file.

## Usage
```
> set-cosmic-theme --help
set-cosmic-theme

USAGE:
  set-cosmic-theme [MODES] THEME

FLAGS:
  -h, --help    Print help information.

MODES:
  --dark        Set the COSMIC dark theme.
  --light       Set the COSMIC light theme.
                If neither --dark or --light are specified, which theme to set
                is determined based on the theme file's palette.
  --gtk4        Set the GTK4 user CSS based on the theme file.

ARGS:
  <THEME>           The theme file to set. Should be a RON theme file exported
                    from cosmic-settings or in the same format.
```
