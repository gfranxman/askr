_askr() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="askr"
                ;;
            askr,completion)
                cmd="askr__completion"
                ;;
            askr,help)
                cmd="askr__help"
                ;;
            askr__help,completion)
                cmd="askr__help__completion"
                ;;
            askr__help,help)
                cmd="askr__help__help"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        askr)
            opts="-h -V --output --quiet --verbose --required --max-length --min-length --pattern --pattern-message --validate-email --validate-hostname --validate-url --validate-ipv4 --validate-ipv6 --number --integer --float --range --positive --negative --date --date-format --time --time-format --datetime --datetime-format --choices --choices-case-sensitive --min-choices --max-choices --file-exists --dir-exists --path-exists --readable --writable --executable --required-priority --length-priority --pattern-priority --format-priority --max-attempts --timeout --default --mask --confirm --no-color --width --help-text --help --version [PROMPT_TEXT] completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -W "default json raw" -- "${cur}"))
                    return 0
                    ;;
                --max-length)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-length)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pattern)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --pattern-message)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --range)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --date-format)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --time-format)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --datetime-format)
                    COMPREPLY=("${cur}")
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --choices)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-choices)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-choices)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --required-priority)
                    COMPREPLY=($(compgen -W "critical high medium low" -- "${cur}"))
                    return 0
                    ;;
                --length-priority)
                    COMPREPLY=($(compgen -W "critical high medium low" -- "${cur}"))
                    return 0
                    ;;
                --pattern-priority)
                    COMPREPLY=($(compgen -W "critical high medium low" -- "${cur}"))
                    return 0
                    ;;
                --format-priority)
                    COMPREPLY=($(compgen -W "critical high medium low" -- "${cur}"))
                    return 0
                    ;;
                --max-attempts)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --default)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --width)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --help-text)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        askr__completion)
            opts="-h --help bash zsh fish power-shell"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        askr__help)
            opts="completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        askr__help__completion)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        askr__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _askr -o nosort -o bashdefault -o default askr
else
    complete -F _askr -o bashdefault -o default askr
fi
