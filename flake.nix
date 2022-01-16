{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/554d2d8aa25b6e583575459c297ec23750adb6cb";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
      application = naersk-lib.buildPackage {
        pname = "borg-exporter";
        root = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
        buildInputs = with pkgs; [
          openssl
          pkg-config
        ];
      };
    in rec {
      # `nix build`
      packages.my-project = pkgs.dockerTools.buildImage {
        name = "k0ral/borg-exporter";
        tag = "latest";
        contents = [ pkgs.bash pkgs.borgbackup pkgs.coreutils application ./docker ];
        runAsRoot = ''
          #!${pkgs.bash}
          ${pkgs.dockerTools.shadowSetup}
        '';
        config = {
          Entrypoint = [ "mount-ssh.sh" ];
          Command = [ "borg-exporter" ];
          ExposedPorts = { "9884" = { }; };
        };
      };
      defaultPackage = packages.my-project;

      # `nix run`
      apps.my-project = utils.lib.mkApp {
        drv = packages.my-project;
      };
      defaultApp = apps.my-project;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo cargo-watch openssl pkg-config rustfmt ];
      };
    });
}
