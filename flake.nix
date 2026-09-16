# SPDX-License-Identifier: Unlicense
{
    inputs = {
        ## Foundation
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        flake-parts.url = "github:hercules-ci/flake-parts";

    };

    outputs =
        inputs@{
            flake-parts,
            ...
        }:
        flake-parts.lib.mkFlake
            {
                inherit inputs;
            }
            {
                imports = [
                    ./nix/flake.nix
                ];
            };
}
