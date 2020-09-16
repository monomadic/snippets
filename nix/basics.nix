# inspecting (printing) nixos configuration
nixos-option -I nixos-config=/etc/nixos/configuration.nix programs.gnupg.agent.enable

# or, simply: (note tab-completion works)
nixos-option programs.gnupg.agent
