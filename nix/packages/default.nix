{
  self,
  inputs,
  ...
}:
{
  perSystem =
    {
      pkgs,
      self',
      lib,
      ...
    }:
    let
      naersk' = pkgs.callPackage inputs.naersk { };
    in
    {
      packages = {
        ## NOTE Using default packageWrapping
        basic = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "PREFERRED_SETUP_RUST";
          version = "0.0.1";
          src = "${self}";
          cargoHash = "sha256-i1rsRhWA3NHz+6oKq6Gd61/2WKXdQPxm32QZgj/sIlk=";
          meta = with lib; {
            description = "This is my preferred rust-development setup.";
            license = licenses.unlicense;
            maintainers = [ ];
          };
        });

        #INFO: This one is better
        naersk = naersk'.buildPackage {
          src = self;
        };

        default = self'.packages.naersk;
      };
    };
}
