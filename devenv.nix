{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  packages = with pkgs; [
    git
    rustlings
  ];

  files.".editorconfig" = {
    ini = {
      "*" = {
        indent_size = 2;
        indent_type = "space";
      };
    };
    copyMode = "copy";
  };

  languages.rust = {
    enable = true;
  };

  # https://devenv.sh/processes/
  processes = {
    dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";
  };

  env.GREET = "devenv";
  scripts.hello.exec = ''
    echo hello from $GREET
  '';
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';

  # https://devenv.sh/tasks/
  tasks = {
    "checks:cargo".exec = "cargo check"; # TEST
  };

  /*
    devenv inputs add git-hooks github:cachix/git-hooks.nix
    devenv inputs add treefmt-nix github:numtide/treefmt-nix
  */
  treefmt = {
    enable = true;
    config = {
      programs = {
        nixfmt = {
          enable = true;
          indent = 2;
        };
        rustfmt.enable = true;
      };
    };
  };
  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    treefmt.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
