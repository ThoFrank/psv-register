service-name: service-pkg:
{ lib, config, pkgs, ... }:
with lib;
let
  cfg = config.services."${service-name}";
in
{
  options = {
    enable = mkEnableOption "${service-name} service";
    database-location = mkOption {
      type = types.str;
      default = "/var/psv-register/${service-name}.sqlite";
    };
    settings = mkOption {
      type = pkgs.formats.toml.type;
      default = { };
      example = literalExpression ''
        {
          port = 3000;
          mail_server = {
            smtp_server = "smtp.mymail.com";
            smtp_username = "myuser";
            smtp_password = "t0p_secret";
          };
          mail_message = {
            sender_name = "Sender";
            sender_address = "me@mymail.com";
            subject = "Registration accepted";
          };
        }
      '';
      description = ''
        config.toml uses for ${service-name}
      '';
    };
  };
  config = {
    systemd.services."${service-name}" = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig.ExecStart = ''
        ${service-pkg}/bin/backend \
        --config-file ${pkgs.formats.toml.generate "${service-name} cfg.settings"} \
        --mail-template_file ${../backend/user_mail.tpl} \
        --database-file ${cfg.database-location}
      '';
    };
  };
}
