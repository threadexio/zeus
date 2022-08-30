#compdef zeus

autoload -U is-at-least

_zeus() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'*--color=[Colorize the output]:COLOR:(auto never always)' \
'*-l+[Set log level]:LOG_LEVEL:(error warn info debug)' \
'*--level=[Set log level]:LOG_LEVEL:(error warn info debug)' \
'*--builddir=[Package build directory]:BUILD_DIR: ' \
'*--aur=[AUR URL]:AUR: ' \
'*--rt=[Specify runtime to use]:RUNTIME: ' \
'*--rtdir=[Specify directory to search for runtimes]:RUNTIME_DIR: ' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
":: :_zeus_commands" \
"*::: :->zeus" \
&& ret=0
    case $state in
    (zeus)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:zeus-command-$line[1]:"
        case $line[1] in
            (sync)
_arguments "${_arguments_options[@]}" \
'*--build-args=[Extra arguments for makepkg]:BUILD_ARGS: ' \
'*--name=[Builder machine name]:MACHINE_NAME: ' \
'-u[Upgrade packages]' \
'--upgrade[Upgrade packages]' \
'--install[Install packages after build]' \
'-h[Print help information]' \
'--help[Print help information]' \
'*::packages:' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'*--name=[Builder machine name]:MACHINE_NAME: ' \
'--uninstall[Uninstall packages after remove]' \
'-h[Print help information]' \
'--help[Print help information]' \
'*::packages:' \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" \
'*--name=[Builder machine name]:MACHINE_NAME: ' \
'*--image=[Builder machine image]:MACHINE_IMAGE: ' \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(query)
_arguments "${_arguments_options[@]}" \
'(-i --info)*--by=[Query AUR packages by]:BY:((name\:"Search by package name"
name-desc\:"Search by package name and description"
maintainer\:"Search by maintainer"
depends\:"Search by dependencies"
make-depends\:"Search by dev dependencies"
opt-depends\:"Search by optional dependencies"
check-depends\:"Search by testing dependencies"))' \
'*-o+[Output format]:OUTPUT:(pretty json)' \
'*--output=[Output format]:OUTPUT:(pretty json)' \
'(--by)-i[Display additional information on results]' \
'(--by)--info[Display additional information on results]' \
'-h[Print help information]' \
'--help[Print help information]' \
'*::keywords:' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'*-s+[Specify shell to generate completions for]:SHELL:(bash fish zsh)' \
'*--shell=[Specify shell to generate completions for]:SHELL:(bash fish zsh)' \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(runtime)
_arguments "${_arguments_options[@]}" \
'-l[List all available runtimes]' \
'--list[List all available runtimes]' \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'*::subcommand -- The subcommand whose help message to display:' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_zeus_commands] )) ||
_zeus_commands() {
    local commands; commands=(
'sync:Sync packages' \
'remove:Remove packages' \
'build:Build/Update builder' \
'query:Query the AUR' \
'completions:Generate shell completions' \
'runtime:Various runtime operations' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'zeus commands' commands "$@"
}
(( $+functions[_zeus__build_commands] )) ||
_zeus__build_commands() {
    local commands; commands=()
    _describe -t commands 'zeus build commands' commands "$@"
}
(( $+functions[_zeus__completions_commands] )) ||
_zeus__completions_commands() {
    local commands; commands=()
    _describe -t commands 'zeus completions commands' commands "$@"
}
(( $+functions[_zeus__help_commands] )) ||
_zeus__help_commands() {
    local commands; commands=()
    _describe -t commands 'zeus help commands' commands "$@"
}
(( $+functions[_zeus__query_commands] )) ||
_zeus__query_commands() {
    local commands; commands=()
    _describe -t commands 'zeus query commands' commands "$@"
}
(( $+functions[_zeus__remove_commands] )) ||
_zeus__remove_commands() {
    local commands; commands=()
    _describe -t commands 'zeus remove commands' commands "$@"
}
(( $+functions[_zeus__runtime_commands] )) ||
_zeus__runtime_commands() {
    local commands; commands=()
    _describe -t commands 'zeus runtime commands' commands "$@"
}
(( $+functions[_zeus__sync_commands] )) ||
_zeus__sync_commands() {
    local commands; commands=()
    _describe -t commands 'zeus sync commands' commands "$@"
}

_zeus "$@"
