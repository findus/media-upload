{
  config,
  lib,
  pkgs,
  ...
}:
let
  service = "media-upload";
  cfg = config.homelab.services.${service};
  hl = config.homelab;

  # Runs as root (+ prefix) to write /etc/media-upload.toml which is hardcoded in the app.
  writeConfig = pkgs.writeShellScript "media-upload-write-config" ''
    TOKEN=$(cat ${cfg.tokenFile})
    cat > /etc/media-upload.toml <<EOF
    path = "${cfg.uploadPath}"
    token = "$TOKEN"
    url = "https://${cfg.domain}"
    ip = "127.0.0.1"
    port = "${toString cfg.port}"
    EOF
    chmod 640 /etc/media-upload.toml
    chown ${service}:${service} /etc/media-upload.toml
  '';
in
{
  options.homelab.services.${service} = {
    enable = lib.mkEnableOption "media-uploader — multipart HTTP upload with EXIF stripping";

    domain = lib.mkOption {
      type = lib.types.str;
      default = "img.${hl.baseDomain}";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 3001;
    };

    uploadPath = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/media-upload";
      description = "Directory where uploaded files are stored and served from.";
    };

    dirGroup = lib.mkOption {
      type = lib.types.str;
      default = service;
      description = "Group owning uploadPath — set to a share group to grant extra access.";
    };

    dirMode = lib.mkOption {
      type = lib.types.str;
      default = "0755";
      description = "Mode of uploadPath. Use 2775 (setgid) so a share group can read/write and new files inherit the group; world r-x keeps Caddy's file_server working.";
    };

    tokenFile = lib.mkOption {
      type = lib.types.path;
      description = "Path to a file containing the bearer token (agenix secret).";
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.${service} = {
      isSystemUser = true;
      group = service;
    };
    users.groups.${service} = { };

    # Caddy serves uploadPath directly; some files end up without an "other"
    # read bit (e.g. exiv2's metadata-stripping rewrite drops it), so Caddy
    # needs group access via dirGroup rather than relying on world-readable.
    users.users.${config.services.caddy.user}.extraGroups = [ cfg.dirGroup ];

    systemd.tmpfiles.rules = [
      "d ${cfg.uploadPath} ${cfg.dirMode} ${service} ${cfg.dirGroup} - -"
      # Recursively reset ownership on every boot — corrects files that
      # land with the wrong owner (e.g. manually copied in as another user)
      # back to the service default. Mode ("-") is left untouched.
      "Z ${cfg.uploadPath} - ${service} ${cfg.dirGroup} - -"
      # Pre-create so ProtectSystem=strict + ReadWritePaths below can target it.
      "f /etc/media-upload.toml 0640 ${service} ${service} - -"
    ];

    systemd.services.${service} = {
      description = "Media upload service";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      path = [ pkgs.ffmpeg ];
      serviceConfig = {
        ExecStartPre = "+${writeConfig}";
        ExecStart = "${pkgs.media-uploader}/bin/media-uploader";
        Type = "simple";
        Restart = "on-failure";
        User = service;
        Group = service;

        # writeConfig runs as root (the "+" prefix) only to write the
        # app-hardcoded /etc path; that file is carved out of the
        # otherwise read-only filesystem below.
        ReadWritePaths = [
          cfg.uploadPath
          "/etc/media-upload.toml"
        ];

        NoNewPrivileges = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        ProtectClock = true;
        ProtectHostname = true;
        ProtectProc = "invisible";
        ProcSubset = "pid";
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        RemoveIPC = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [ "@system-service" ];
      };
    };

    services.caddy.virtualHosts."*.${hl.baseDomain}" = {
      extraConfig = ''
        @mediaupload host ${cfg.domain}
        handle @mediaupload {
          @upload method POST
          handle @upload {
            # The app serves the upload handler at POST / ; map the public
            # /upload path onto it.
            rewrite * /
            reverse_proxy 127.0.0.1:${toString cfg.port}
          }
          root * ${cfg.uploadPath}
          file_server
        }
      '';
    };
  };
}
