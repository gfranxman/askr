#!/bin/bash
# Advanced validation examples for askr

echo "=== Advanced Validation Examples ==="

echo
echo "1. Strong password requirements:"
echo 'askr "Password:" \'
echo '    --required \'
echo '    --min-length 12 \'
echo '    --pattern ".*[A-Z].*" --pattern-message "Must contain uppercase letter" \'
echo '    --pattern ".*[a-z].*" --pattern-message "Must contain lowercase letter" \'
echo '    --pattern ".*[0-9].*" --pattern-message "Must contain number" \'
echo '    --pattern ".*[!@#$%^&*].*" --pattern-message "Must contain special character" \'
echo '    --mask'

echo
echo "2. Username with multiple constraints:"
echo 'askr "Username:" \'
echo '    --required \'
echo '    --min-length 3 --max-length 20 \'
echo '    --pattern "^[a-zA-Z0-9_]+$" --pattern-message "Only letters, numbers, and underscores" \'
echo '    --pattern "^[a-zA-Z].*" --pattern-message "Must start with a letter"'

echo
echo "3. Email with domain restrictions:"
echo 'askr "Company email:" \'
echo '    --validate-email \'
echo '    --pattern ".*@(example\.com|company\.org)$" \'
echo '    --pattern-message "Must be @example.com or @company.org domain"'

echo
echo "4. Date with custom format:"
echo 'askr "Birthday (MM/DD/YYYY):" \'
echo '    --date --date-format "%m/%d/%Y"'

echo
echo "5. File validation:"
echo 'askr "Config file:" --file-exists --readable'

echo
echo "6. Multiple choices with constraints:"
echo 'askr "Select team members:" \'
echo '    --choices "alice,bob,charlie,diana,eve" \'
echo '    --min-choices 2 \'
echo '    --max-choices 4'

echo
echo "7. Number validation with multiple constraints:"
echo 'askr "CPU cores:" \'
echo '    --integer \'
echo '    --range 1-128 \'
echo '    --default "4"'

echo
echo "8. URL validation with custom patterns:"
echo 'askr "API endpoint:" \'
echo '    --validate-url \'
echo '    --pattern "^https://.*" --pattern-message "Must use HTTPS"'

echo
echo "Run any of these commands to see advanced validation in action!"