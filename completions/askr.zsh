#compdef askr

autoload -U is-at-least

_askr() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--output=[Output format]:OUTPUT:(default json raw)' \
'--max-length=[Maximum character length]:MAX_LENGTH:_default' \
'--min-length=[Minimum character length]:MIN_LENGTH:_default' \
'*--pattern=[Custom regex pattern (can be used multiple times)]:PATTERN:' \
'*--pattern-message=[Custom error message for pattern validation (applies to most recent --pattern)]:PATTERN_MESSAGE:_default' \
'--range=[Numeric range (e.g., 1-100)]:RANGE:_default' \
'--date-format=[Expected date format (default\: %Y-%m-%d)]:DATE_FORMAT:' \
'--time-format=[Expected time format (default\: %H\:%M\:%S)]:TIME_FORMAT:' \
'--datetime-format=[Expected datetime format]:DATETIME_FORMAT:' \
'--choices=[Comma-separated list of valid choices]:CHOICES:_default' \
'--min-choices=[Minimum number of choices required (default\: 1)]:MIN_CHOICES:_default' \
'--max-choices=[Maximum number of choices allowed (default\: 1)]:MAX_CHOICES:_default' \
'--required-priority=[Priority for required validation (default\: critical)]:REQUIRED_PRIORITY:(critical high medium low)' \
'--length-priority=[Priority for length validation (default\: medium)]:LENGTH_PRIORITY:(critical high medium low)' \
'--pattern-priority=[Priority for pattern validation (default\: high)]:PATTERN_PRIORITY:(critical high medium low)' \
'--format-priority=[Priority for format validation (default\: high)]:FORMAT_PRIORITY:(critical high medium low)' \
'--max-attempts=[Maximum validation attempts (default\: unlimited)]:MAX_ATTEMPTS:_default' \
'--timeout=[Input timeout in seconds]:TIMEOUT:_default' \
'--default=[Default value if user presses Enter]:DEFAULT:_default' \
'--width=[Maximum display width]:WIDTH:_default' \
'--help-text=[Additional help text displayed below prompt]:HELP_TEXT:_default' \
'--quiet[Non-interactive mode, read from stdin]' \
'--verbose[Show detailed validation messages to stderr]' \
'--required[Input cannot be empty]' \
'--validate-email[Email address validation]' \
'--validate-hostname[Hostname/domain validation]' \
'--validate-url[URL validation]' \
'--validate-ipv4[IPv4 address validation]' \
'--validate-ipv6[IPv6 address validation]' \
'--number[Accept only numeric input]' \
'--integer[Accept only integer input]' \
'--float[Accept only floating-point input]' \
'--positive[Only positive numbers]' \
'--negative[Only negative numbers]' \
'--date[Accept date input]' \
'--time[Accept time input]' \
'--datetime[Accept datetime input]' \
'--choices-case-sensitive[Make choice matching case-sensitive]' \
'--file-exists[File must exist]' \
'--dir-exists[Directory must exist]' \
'--path-exists[File or directory must exist]' \
'--readable[Path must be readable]' \
'--writable[Path must be writable]' \
'--executable[File must be executable]' \
'--mask[Mask input (for passwords)]' \
'--confirm[Require confirmation input]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'::prompt_text -- The text to display as the prompt:_default' \
":: :_askr_commands" \
"*::: :->askr" \
&& ret=0
    case $state in
    (askr)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:askr-command-$line[2]:"
        case $line[2] in
            (completion)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':shell -- The shell to generate completions for:(bash zsh fish power-shell)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_askr__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:askr-help-command-$line[1]:"
        case $line[1] in
            (completion)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_askr_commands] )) ||
_askr_commands() {
    local commands; commands=(
'completion:Generate shell completion scripts' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'askr commands' commands "$@"
}
(( $+functions[_askr__completion_commands] )) ||
_askr__completion_commands() {
    local commands; commands=()
    _describe -t commands 'askr completion commands' commands "$@"
}
(( $+functions[_askr__help_commands] )) ||
_askr__help_commands() {
    local commands; commands=(
'completion:Generate shell completion scripts' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'askr help commands' commands "$@"
}
(( $+functions[_askr__help__completion_commands] )) ||
_askr__help__completion_commands() {
    local commands; commands=()
    _describe -t commands 'askr help completion commands' commands "$@"
}
(( $+functions[_askr__help__help_commands] )) ||
_askr__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'askr help help commands' commands "$@"
}

if [ "$funcstack[1]" = "_askr" ]; then
    _askr "$@"
else
    compdef _askr askr
fi
