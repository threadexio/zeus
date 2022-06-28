complete -c zeus -n "__fish_use_subcommand" -l color -d 'Colorize the output' -r -f -a "{auto	,always	,never	}"
complete -c zeus -n "__fish_use_subcommand" -l builddir -d 'Package build directory' -r
complete -c zeus -n "__fish_use_subcommand" -l aur -d 'AUR host' -r
complete -c zeus -n "__fish_use_subcommand" -l rt -d 'Specify runtime to use' -r
complete -c zeus -n "__fish_use_subcommand" -l rtdir -d 'Specify directory to search for runtimes' -r
complete -c zeus -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c zeus -n "__fish_use_subcommand" -s d -l debug -d 'Show debug logs'
complete -c zeus -n "__fish_use_subcommand" -l force -d 'Ignore all warnings'
complete -c zeus -n "__fish_use_subcommand" -f -a "sync" -d 'Sync packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "remove" -d 'Remove packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "build" -d 'Build/Update builder image'
complete -c zeus -n "__fish_use_subcommand" -f -a "query" -d 'Query the AUR'
complete -c zeus -n "__fish_use_subcommand" -f -a "completions" -d 'Generate shell completions & others'
complete -c zeus -n "__fish_use_subcommand" -f -a "runtime" -d 'Various runtime operations'
complete -c zeus -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c zeus -n "__fish_seen_subcommand_from sync" -l buildargs -d 'Extra arguments for makepkg' -r
complete -c zeus -n "__fish_seen_subcommand_from sync" -l name -d 'Builder container name' -r
complete -c zeus -n "__fish_seen_subcommand_from sync" -s u -l upgrade -d 'Upgrade packages'
complete -c zeus -n "__fish_seen_subcommand_from sync" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from remove" -l name -d 'Builder container name' -r
complete -c zeus -n "__fish_seen_subcommand_from remove" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from build" -l image -d 'Builder image name' -r
complete -c zeus -n "__fish_seen_subcommand_from build" -l name -d 'Builder container name' -r
complete -c zeus -n "__fish_seen_subcommand_from build" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from query" -l by -d 'Query AUR packages by' -r -f -a "{name	,description	,maintainer	,depends	,makedepends	,optdepends	,checkdepends	}"
complete -c zeus -n "__fish_seen_subcommand_from query" -l output -d 'Output format' -r -f -a "{pretty	,json	}"
complete -c zeus -n "__fish_seen_subcommand_from query" -s i -l info -d 'Display additional information on results'
complete -c zeus -n "__fish_seen_subcommand_from query" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from completions" -l shell -d 'Specify shell to generate completions for' -r -f -a "{bash	,elvish	,fish	,powershell	,zsh	}"
complete -c zeus -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s l -l list -d 'List all available runtimes'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s c -l c -d 'Check that the specified runtime works'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s h -l help -d 'Print help information'
