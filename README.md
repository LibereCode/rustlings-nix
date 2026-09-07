# lets get rusty 🦀

My solutions to rustlings-exercises.
I am on _NixOS_, and use [devenv.sh](https://devenv.sh/languages/rust/) (_btw_).

## setup

```sh
devenv init
# edit devenv,
# make sure to `pkgs.rustlings` to packages.

devenv inputs add git-hooks github:cachix/git-hooks.nix
devenv inputs add treefmt-nix github:numtide/treefmt-nix

devenv allow
z - ; z -

rustlings init
mv rustlings/{*,.*} ./

rustlings
```

## LICENSE

Copyleft (🄯) 2026 LibereCode. All Rights Reserved.\
Licensed under the **EUPL-1.2**. See [the LICENSE](./LICENSE) for details.

> [!NOTE]
> My license (_EUPL_) only cover my changes.
> For original license, see [rustling's license](./licenses/rustlings.LICENSE) (MIT).
