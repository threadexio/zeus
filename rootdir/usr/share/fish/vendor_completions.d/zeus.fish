complete -c zeus -n "__fish_use_subcommand" -l name -d 'Builder machine name' -r
complete -c zeus -n "__fish_use_subcommand" -l image -d 'Builder machine image' -r
complete -c zeus -n "__fish_use_subcommand" -l color -d 'Colorize the output' -r -f -a "{auto	,never	,always	}"
complete -c zeus -n "__fish_use_subcommand" -s l -l level -d 'Set log level' -r -f -a "{error	,warn	,info	,debug	}"
complete -c zeus -n "__fish_use_subcommand" -l builddir -d 'Package build directory' -r -F
complete -c zeus -n "__fish_use_subcommand" -l aur -d 'AUR URL' -r
complete -c zeus -n "__fish_use_subcommand" -l rt -d 'Specify runtime to use' -r
complete -c zeus -n "__fish_use_subcommand" -l rtdir -d 'Specify directory to search for runtimes' -r -F
complete -c zeus -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c zeus -n "__fish_use_subcommand" -f -a "sync" -d 'Sync packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "remove" -d 'Remove packages'
complete -c zeus -n "__fish_use_subcommand" -f -a "build" -d 'Build/Update builder'
complete -c zeus -n "__fish_use_subcommand" -f -a "query" -d 'Query the AUR'
complete -c zeus -n "__fish_use_subcommand" -f -a "completions" -d 'Generate shell completions'
complete -c zeus -n "__fish_use_subcommand" -f -a "runtime" -d 'Various runtime operations'
complete -c zeus -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c zeus -n "__fish_seen_subcommand_from sync" -l build-args -d 'Extra arguments for makepkg' -r
complete -c zeus -n "__fish_seen_subcommand_from sync" -s u -l upgrade -d 'Upgrade packages'
complete -c zeus -n "__fish_seen_subcommand_from sync" -l install -d 'Install packages after build (needs root)'
complete -c zeus -n "__fish_seen_subcommand_from sync" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from remove" -l uninstall -d 'Uninstall packages after remove (needs root)'
complete -c zeus -n "__fish_seen_subcommand_from remove" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from build" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from query" -l by -d 'Query AUR packages by' -r -f -a "{name	Search by package name,name-desc	Search by package name and description,maintainer	Search by maintainer,depends	Search by dependencies,make-depends	Search by dev dependencies,opt-depends	Search by optional dependencies,check-depends	Search by testing dependencies}"
complete -c zeus -n "__fish_seen_subcommand_from query" -s o -l output -d 'Output format' -r -f -a "{pretty	,json	}"
complete -c zeus -n "__fish_seen_subcommand_from query" -s i -l info -d 'Display additional information on results'
complete -c zeus -n "__fish_seen_subcommand_from query" -s h -l help -d 'Print help information (use `--help` for more detail)'
complete -c zeus -n "__fish_seen_subcommand_from completions" -s s -l shell -d 'Specify shell to generate completions for' -r -f -a "{bash	,fish	,zsh	}"
complete -c zeus -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s l -l list -d 'List all available runtimes'
complete -c zeus -n "__fish_seen_subcommand_from runtime" -s h -l help -d 'Print help information'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "sync" -d 'Sync packages'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove packages'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "build" -d 'Build/Update builder'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "query" -d 'Query the AUR'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "completions" -d 'Generate shell completions'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "runtime" -d 'Various runtime operations'
complete -c zeus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sync; and not __fish_seen_subcommand_from remove; and not __fish_seen_subcommand_from build; and not __fish_seen_subcommand_from query; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from runtime; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
