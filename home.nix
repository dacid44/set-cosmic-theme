{ config, pkgs, lib, ... }:
{
  options.set-cosmic-theme = {
    dark = lib.mkOption {
      description = "Path to dark mode COSMIC theme file, or `\"default\" (set the default dark theme).`";
      type = lib.types.nullOr (lib.types.either (lib.types.enum [ "default" ]) lib.types.path);
      default = null;
    };
    light = lib.mkOption {
      description = "Path to light mode COSMIC theme file, or `\"default\"` (set the default light theme).";
      type = lib.types.nullOr (lib.types.either (lib.types.enum [ "default" ]) lib.types.path);
      default = null;
    };
    gtk4 = lib.mkOption {
      description = "Which theme mode to apply to the GTK4 stylesheet. If `null`, the GTK4 stylesheet will not be set. If it is set, the corresponding mode must have a path set. EXPERIMENTAL!";
      type = lib.types.nullOr (lib.types.enum [ "left" "right" ]);
      default = null;
    };
  };
  config = let
    cfg = config.set-cosmic-theme;
    set-cosmic-theme = import ./default.nix { pkgs = pkgs; };
  in
  lib.mkIf (cfg.dark != null || cfg.light != null) {
    assertions = [
      {
        assertion = cfg.gtk4 == null || cfg.${cfg.gtk4} != null;
        message = "If the gtk4 option is set, the corresponding mode must not be null";
      }
    ];

    home.activation = {
      setCosmicThemeDark = lib.mkIf (cfg.dark != null) (
        lib.hm.dag.entryAfter [ "writeBoundary" ] ''
          run ${set-cosmic-theme.outPath}/bin/set-cosmic-theme --dark \
              ${lib.strings.escapeShellArg cfg.dark} ${if cfg.gtk4 == "dark" then "--gtk4" else ""}
        ''
      );
      setCosmicThemeLight = lib.mkIf (cfg.light != null) (
        lib.hm.dag.entryAfter [ "writeBoundary" ] ''
          run ${set-cosmic-theme.outPath}/bin/set-cosmic-theme --light \
              ${lib.strings.escapeShellArg cfg.light} ${if cfg.gtk4 == "light" then "--gtk4" else ""}
        ''
      );
    };
  };
}
