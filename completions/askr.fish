# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_askr_global_optspecs
	string join \n output= quiet verbose required max-length= min-length= pattern= pattern-message= validate-email validate-hostname validate-url validate-ipv4 validate-ipv6 number integer float range= positive negative date date-format= time time-format= datetime datetime-format= choices= choices-case-sensitive min-choices= max-choices= file-exists dir-exists path-exists readable writable executable required-priority= length-priority= pattern-priority= format-priority= max-attempts= timeout= default= mask confirm no-color width= help-text= h/help V/version
end

function __fish_askr_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_askr_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_askr_using_subcommand
	set -l cmd (__fish_askr_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c askr -n "__fish_askr_needs_command" -l output -d 'Output format' -r -f -a "default\t''
json\t''
raw\t''"
complete -c askr -n "__fish_askr_needs_command" -l max-length -d 'Maximum character length' -r
complete -c askr -n "__fish_askr_needs_command" -l min-length -d 'Minimum character length' -r
complete -c askr -n "__fish_askr_needs_command" -l pattern -d 'Custom regex pattern (can be used multiple times)' -r -f
complete -c askr -n "__fish_askr_needs_command" -l pattern-message -d 'Custom error message for pattern validation (applies to most recent --pattern)' -r
complete -c askr -n "__fish_askr_needs_command" -l range -d 'Numeric range (e.g., 1-100)' -r
complete -c askr -n "__fish_askr_needs_command" -l date-format -d 'Expected date format (default: %Y-%m-%d)' -r -f
complete -c askr -n "__fish_askr_needs_command" -l time-format -d 'Expected time format (default: %H:%M:%S)' -r -f
complete -c askr -n "__fish_askr_needs_command" -l datetime-format -d 'Expected datetime format' -r -f
complete -c askr -n "__fish_askr_needs_command" -l choices -d 'Comma-separated list of valid choices' -r
complete -c askr -n "__fish_askr_needs_command" -l min-choices -d 'Minimum number of choices required (default: 1)' -r
complete -c askr -n "__fish_askr_needs_command" -l max-choices -d 'Maximum number of choices allowed (default: 1)' -r
complete -c askr -n "__fish_askr_needs_command" -l required-priority -d 'Priority for required validation (default: critical)' -r -f -a "critical\t''
high\t''
medium\t''
low\t''"
complete -c askr -n "__fish_askr_needs_command" -l length-priority -d 'Priority for length validation (default: medium)' -r -f -a "critical\t''
high\t''
medium\t''
low\t''"
complete -c askr -n "__fish_askr_needs_command" -l pattern-priority -d 'Priority for pattern validation (default: high)' -r -f -a "critical\t''
high\t''
medium\t''
low\t''"
complete -c askr -n "__fish_askr_needs_command" -l format-priority -d 'Priority for format validation (default: high)' -r -f -a "critical\t''
high\t''
medium\t''
low\t''"
complete -c askr -n "__fish_askr_needs_command" -l max-attempts -d 'Maximum validation attempts (default: unlimited)' -r
complete -c askr -n "__fish_askr_needs_command" -l timeout -d 'Input timeout in seconds' -r
complete -c askr -n "__fish_askr_needs_command" -l default -d 'Default value if user presses Enter' -r
complete -c askr -n "__fish_askr_needs_command" -l width -d 'Maximum display width' -r
complete -c askr -n "__fish_askr_needs_command" -l help-text -d 'Additional help text displayed below prompt' -r
complete -c askr -n "__fish_askr_needs_command" -l quiet -d 'Non-interactive mode, read from stdin'
complete -c askr -n "__fish_askr_needs_command" -l verbose -d 'Show detailed validation messages to stderr'
complete -c askr -n "__fish_askr_needs_command" -l required -d 'Input cannot be empty'
complete -c askr -n "__fish_askr_needs_command" -l validate-email -d 'Email address validation'
complete -c askr -n "__fish_askr_needs_command" -l validate-hostname -d 'Hostname/domain validation'
complete -c askr -n "__fish_askr_needs_command" -l validate-url -d 'URL validation'
complete -c askr -n "__fish_askr_needs_command" -l validate-ipv4 -d 'IPv4 address validation'
complete -c askr -n "__fish_askr_needs_command" -l validate-ipv6 -d 'IPv6 address validation'
complete -c askr -n "__fish_askr_needs_command" -l number -d 'Accept only numeric input'
complete -c askr -n "__fish_askr_needs_command" -l integer -d 'Accept only integer input'
complete -c askr -n "__fish_askr_needs_command" -l float -d 'Accept only floating-point input'
complete -c askr -n "__fish_askr_needs_command" -l positive -d 'Only positive numbers'
complete -c askr -n "__fish_askr_needs_command" -l negative -d 'Only negative numbers'
complete -c askr -n "__fish_askr_needs_command" -l date -d 'Accept date input'
complete -c askr -n "__fish_askr_needs_command" -l time -d 'Accept time input'
complete -c askr -n "__fish_askr_needs_command" -l datetime -d 'Accept datetime input'
complete -c askr -n "__fish_askr_needs_command" -l choices-case-sensitive -d 'Make choice matching case-sensitive'
complete -c askr -n "__fish_askr_needs_command" -l file-exists -d 'File must exist'
complete -c askr -n "__fish_askr_needs_command" -l dir-exists -d 'Directory must exist'
complete -c askr -n "__fish_askr_needs_command" -l path-exists -d 'File or directory must exist'
complete -c askr -n "__fish_askr_needs_command" -l readable -d 'Path must be readable'
complete -c askr -n "__fish_askr_needs_command" -l writable -d 'Path must be writable'
complete -c askr -n "__fish_askr_needs_command" -l executable -d 'File must be executable'
complete -c askr -n "__fish_askr_needs_command" -l mask -d 'Mask input (for passwords)'
complete -c askr -n "__fish_askr_needs_command" -l confirm -d 'Require confirmation input'
complete -c askr -n "__fish_askr_needs_command" -l no-color -d 'Disable colored output'
complete -c askr -n "__fish_askr_needs_command" -s h -l help -d 'Print help'
complete -c askr -n "__fish_askr_needs_command" -s V -l version -d 'Print version'
complete -c askr -n "__fish_askr_needs_command" -a "completion" -d 'Generate shell completion scripts'
complete -c askr -n "__fish_askr_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c askr -n "__fish_askr_using_subcommand completion" -s h -l help -d 'Print help'
complete -c askr -n "__fish_askr_using_subcommand help; and not __fish_seen_subcommand_from completion help" -f -a "completion" -d 'Generate shell completion scripts'
complete -c askr -n "__fish_askr_using_subcommand help; and not __fish_seen_subcommand_from completion help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
