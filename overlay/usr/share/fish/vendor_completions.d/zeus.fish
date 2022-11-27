complete -c zeus -n "__fish_use_subcommand" -l color -d 'Colorize the output' -r -f -a "{always	,never	,auto	}"
complete -c zeus -n "__fish_use_subcommand" -s l -l level -d 'Set log level' -r -f -a "{fatal	,error	,warn	,info	,debug	,trace	}"
complete -c zeus -n "__fish_use_subcommand" -l build-dir -d 'Package build directory' -r -f -a "(__fish_complete_directories)"
complete -c zeus -n "__fish_use_subcommand" -l aur -d 'AUR URL' -r -f
complete -c zeus -n "__fish_use_subcommand" -l rt -d 'Specify runtime to use' -r -f
complete -c zeus -n "__fish_use_subcommand" -l name -d 'Builder machine name' -r -f
complete -c zeus -n "__fish_use_subcommand" -l image -d 'Builder machine image' -r -f
complete -c zeus -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c zeus -n "__fish_use_subcommand" -f -a "sync" -d 'Sync packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "remove" -d 'Remove packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "build" -d 'Build/Update builder'
complete -c zeus -n "__fish_use_subcommand" -f -a "query" -d 'Query the AUR'
complete -c zeus -n "__fish_use_subcommand" -f -a "runtime" -d 'Various runtime operations'
complete -c zeus -n "__fish_use_subcommand" -f -a "completions" -d 'Generate shell completions'
complete -c zeus -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c zeus -n "__fish_seen_subcommand_from sync" -l build-args -d 'Extra arguments for makepkg' -r -f
complete -c zeus -n "__fish_seen_subcommand_from sync" -s u -l upgrade -d 'Upgrade packages'
complete -c zeus -n "__fish_seen_subcommand_from sync" -l install -d 'Install packages after build'
complete -c zeus -n "__fish_seen_subcommand_from sync" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from remove" -l uninstall -d 'Uninstall packages after remove'
complete -c zeus -n "__fish_seen_subcommand_from remove" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from build" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from query" -s b -l by -d 'Query AUR packages by' -r -f -a "{name	,name-desc	,maintainer	,depends	,makedepends	,optdepends	,checkdepends	}"
complete -c zeus -n "__fish_seen_subcommand_from query" -l output -d 'Output format' -r -f -a "{pretty	,json	}"
complete -c zeus -n "__fish_seen_subcommand_from query" -s i -l info -d 'Display additional information on results'
complete -c zeus -n "__fish_seen_subcommand_from query" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s l -l lost -d 'List all available runtimes'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from completions" -s s -l shell -d 'Specify shell to generate completions for' -r -f -a "{bash	,zsh	,fish	}"
complete -c zeus -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Print help information'
