let
  pkgs = import <nixpkgs> {};

  mathlibtools = import (pkgs.fetchzip {
    url = "https://github.com/leanprover-community/mathlib-tools/archive/77e36fa1c84b87cea81407d5acd08389b9777790.zip";
    sha256 = "1859phpa6d814ii2f0g5czcqsq394xzpfkc64q240xs81l7kb3hk";
  }) { inherit pkgs; };

  # pkgs_lean_3_21 = import (builtins.fetchGit {
  #   # Descriptive name to make the store path easier to identify                
  #   name = "pkgs_lean_3_21_revisiion";                                                 
  #   url = "https://github.com/nixos/nixpkgs/";                       
  #   ref = "refs/heads/master";                     
  #   rev = "855b84d8b013a88804093cb8782c748441b999a6";
  # }) {};                                                                           

  pkgs_lean_3_21 = import (pkgs.fetchzip {
    name = "pkgs_lean_3_21_revision";
    url = "https://github.com/NixOS/nixpkgs/archive/855b84d8b013a88804093cb8782c748441b999a6.zip";
    sha256 = "0lhg7xik6qw30bjv9mb9cg49qrwb110mj9irzkbsxrijq0nhhls4";
  }) {};

  lean = pkgs_lean_3_21.lean;
  elan = pkgs_lean_3_21.elan;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    lean
    elan

    mathlibtools
  ];
}
