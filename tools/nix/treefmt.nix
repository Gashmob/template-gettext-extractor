{ pkgs, treefmt-nix }:
treefmt-nix.lib.mkWrapper pkgs {
  programs.actionlint.enable = true;
  programs.nixfmt.enable = true;
  programs.nixf-diagnose.enable = true;
  programs.rustfmt.enable = true;
  programs.shellcheck = {
    enable = true;
    severity = "warning";
  };
  programs.shfmt = {
    enable = true;
    useEditorConfig = true;
  };
  programs.typos = {
    enable = true;
    configFile = toString (
      pkgs.writeText "typos-config.toml" ''
        [default.extend-words]
        TGE = "TGE"
      ''
    );
  };
}
