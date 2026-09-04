{
  description = "skwd-wall";

  inputs.release.url = "github:liixini/skwd-wall/nix";

  nixConfig = {
    extra-substituters = [
      "https://github.com/liixini/skwd-wall/releases/download/nix-cache"
    ];
    extra-trusted-public-keys = [
      "skwd-wall-v2-1:oDTVsejeUw3NmND4qKUwek0muuUnsolw/5qM8n/Gl54="
    ];
  };

  outputs = { release, ... }: {
    packages = release.packages;
    checks = release.checks;
    nixosModules = release.nixosModules;
  };
}
