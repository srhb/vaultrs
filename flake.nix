{
  description = "vaultrs";
  inputs = {
    ## Nixpkgs ##
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs@{nixpkgs, ...}: {
    devShells.x86_64-linux.default = let pkgs = import nixpkgs { system = "x86_64-linux"; }; in pkgs.mkShell {
      packages = with pkgs; [
        cargo
        rustc
        rust-analyzer
      ];
    };
  };
}
