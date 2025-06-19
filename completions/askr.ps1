
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'askr' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'askr'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'askr' {
            [CompletionResult]::new('--output', '--output', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--max-length', '--max-length', [CompletionResultType]::ParameterName, 'Maximum character length')
            [CompletionResult]::new('--min-length', '--min-length', [CompletionResultType]::ParameterName, 'Minimum character length')
            [CompletionResult]::new('--pattern', '--pattern', [CompletionResultType]::ParameterName, 'Custom regex pattern (can be used multiple times)')
            [CompletionResult]::new('--pattern-message', '--pattern-message', [CompletionResultType]::ParameterName, 'Custom error message for pattern validation (applies to most recent --pattern)')
            [CompletionResult]::new('--range', '--range', [CompletionResultType]::ParameterName, 'Numeric range (e.g., 1-100)')
            [CompletionResult]::new('--date-format', '--date-format', [CompletionResultType]::ParameterName, 'Expected date format (default: %Y-%m-%d)')
            [CompletionResult]::new('--time-format', '--time-format', [CompletionResultType]::ParameterName, 'Expected time format (default: %H:%M:%S)')
            [CompletionResult]::new('--datetime-format', '--datetime-format', [CompletionResultType]::ParameterName, 'Expected datetime format')
            [CompletionResult]::new('--choices', '--choices', [CompletionResultType]::ParameterName, 'Comma-separated list of valid choices')
            [CompletionResult]::new('--min-choices', '--min-choices', [CompletionResultType]::ParameterName, 'Minimum number of choices required (default: 1)')
            [CompletionResult]::new('--max-choices', '--max-choices', [CompletionResultType]::ParameterName, 'Maximum number of choices allowed (default: 1)')
            [CompletionResult]::new('--required-priority', '--required-priority', [CompletionResultType]::ParameterName, 'Priority for required validation (default: critical)')
            [CompletionResult]::new('--length-priority', '--length-priority', [CompletionResultType]::ParameterName, 'Priority for length validation (default: medium)')
            [CompletionResult]::new('--pattern-priority', '--pattern-priority', [CompletionResultType]::ParameterName, 'Priority for pattern validation (default: high)')
            [CompletionResult]::new('--format-priority', '--format-priority', [CompletionResultType]::ParameterName, 'Priority for format validation (default: high)')
            [CompletionResult]::new('--max-attempts', '--max-attempts', [CompletionResultType]::ParameterName, 'Maximum validation attempts (default: unlimited)')
            [CompletionResult]::new('--timeout', '--timeout', [CompletionResultType]::ParameterName, 'Input timeout in seconds')
            [CompletionResult]::new('--default', '--default', [CompletionResultType]::ParameterName, 'Default value if user presses Enter')
            [CompletionResult]::new('--width', '--width', [CompletionResultType]::ParameterName, 'Maximum display width')
            [CompletionResult]::new('--help-text', '--help-text', [CompletionResultType]::ParameterName, 'Additional help text displayed below prompt')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Non-interactive mode, read from stdin')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show detailed validation messages to stderr')
            [CompletionResult]::new('--required', '--required', [CompletionResultType]::ParameterName, 'Input cannot be empty')
            [CompletionResult]::new('--validate-email', '--validate-email', [CompletionResultType]::ParameterName, 'Email address validation')
            [CompletionResult]::new('--validate-hostname', '--validate-hostname', [CompletionResultType]::ParameterName, 'Hostname/domain validation')
            [CompletionResult]::new('--validate-url', '--validate-url', [CompletionResultType]::ParameterName, 'URL validation')
            [CompletionResult]::new('--validate-ipv4', '--validate-ipv4', [CompletionResultType]::ParameterName, 'IPv4 address validation')
            [CompletionResult]::new('--validate-ipv6', '--validate-ipv6', [CompletionResultType]::ParameterName, 'IPv6 address validation')
            [CompletionResult]::new('--number', '--number', [CompletionResultType]::ParameterName, 'Accept only numeric input')
            [CompletionResult]::new('--integer', '--integer', [CompletionResultType]::ParameterName, 'Accept only integer input')
            [CompletionResult]::new('--float', '--float', [CompletionResultType]::ParameterName, 'Accept only floating-point input')
            [CompletionResult]::new('--positive', '--positive', [CompletionResultType]::ParameterName, 'Only positive numbers')
            [CompletionResult]::new('--negative', '--negative', [CompletionResultType]::ParameterName, 'Only negative numbers')
            [CompletionResult]::new('--date', '--date', [CompletionResultType]::ParameterName, 'Accept date input')
            [CompletionResult]::new('--time', '--time', [CompletionResultType]::ParameterName, 'Accept time input')
            [CompletionResult]::new('--datetime', '--datetime', [CompletionResultType]::ParameterName, 'Accept datetime input')
            [CompletionResult]::new('--choices-case-sensitive', '--choices-case-sensitive', [CompletionResultType]::ParameterName, 'Make choice matching case-sensitive')
            [CompletionResult]::new('--file-exists', '--file-exists', [CompletionResultType]::ParameterName, 'File must exist')
            [CompletionResult]::new('--dir-exists', '--dir-exists', [CompletionResultType]::ParameterName, 'Directory must exist')
            [CompletionResult]::new('--path-exists', '--path-exists', [CompletionResultType]::ParameterName, 'File or directory must exist')
            [CompletionResult]::new('--readable', '--readable', [CompletionResultType]::ParameterName, 'Path must be readable')
            [CompletionResult]::new('--writable', '--writable', [CompletionResultType]::ParameterName, 'Path must be writable')
            [CompletionResult]::new('--executable', '--executable', [CompletionResultType]::ParameterName, 'File must be executable')
            [CompletionResult]::new('--mask', '--mask', [CompletionResultType]::ParameterName, 'Mask input (for passwords)')
            [CompletionResult]::new('--confirm', '--confirm', [CompletionResultType]::ParameterName, 'Require confirmation input')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion scripts')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'askr;completion' {
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'askr;help' {
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion scripts')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'askr;help;completion' {
            break
        }
        'askr;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
