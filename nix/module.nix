self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.repo-builder;
in
{
  options.programs."canary" = {
    enable = lib.mkEnableOption "Rust firmware turning an ESP32-S3 touch LCD into an always-on desk widget for environment and status info.";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "canary.packages.\${system}.default";
      description = "Rust firmware turning an ESP32-S3 touch LCD into an always-on desk widget for environment and status info.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
