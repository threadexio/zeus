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
'--color=[Colorize the output]: :(auto always never)' \
'--builddir=[Package build directory]: : ' \
'--aur=[AUR host]: : ' \
'-h[Print help information]' \
'--help[Print help information]' \
'-V[Print version information]' \
'--version[Print version information]' \
'-d[Show debug logs]' \
'--debug[Show debug logs]' \
'--force[Ignore all warnings]' \
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
'--buildargs=[Extra arguments for makepkg]: : ' \
'--image=[Builder image name]: : ' \
'--name=[Builder container name]: : ' \
'-u[Upgrade packages]' \
'--upgrade[Upgrade packages]' \
'-h[Print help information]' \
'--help[Print help information]' \
'*::packages -- Packages to sync:' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'-h[Print help information]' \
'--help[Print help information]' \
'*::packages -- Packages to remove:' \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" \
'--archive=[Builder image archive]: : ' \
'--dockerfile=[Builder image dockerfile in archive]: : ' \
'--image=[Builder image name]: : ' \
'--name=[Builder container name]: : ' \
'-h[Print help information]' \
'--help[Print help information]' \
&& ret=0
;;
(query)
_arguments "${_arguments_options[@]}" \
'(-i --info)--by=[Query AUR packages by]: :(name description maintainer depends makedepends optdepends checkdepends)' \
'--output=[Output format]: :(pretty json)' \
'(--by)-i[Display additional information on results]' \
'(--by)--info[Display additional information on results]' \
'-h[Print help information]' \
'--help[Print help information]' \
'*::keywords -- Keywords to use:' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'--shell=[Specify shell to generate completions for]: :(bash elvish fish powershell zsh)' \
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
'build:Build/Update builder image' \
'query:Query the AUR' \
'completions:Generate shell completions & others' \
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
(( $+functions[_zeus__sync_commands] )) ||
_zeus__sync_commands() {
    local commands; commands=()
    _describe -t commands 'zeus sync commands' commands "$@"
}

_zeus "$@"
