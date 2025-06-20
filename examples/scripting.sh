#!/bin/bash
# Scripting examples using askr

echo "=== Scripting with askr ==="

echo
echo "Example 1: Configuration script"
cat << 'EOF'
#!/bin/bash
# setup.sh - Interactive configuration script

echo "🔧 Setting up your application..."

# Get basic configuration
app_name=$(askr "Application name:" --required --min-length 1)
environment=$(askr "Environment:" --choices "development,staging,production")
debug_enabled=$(askr "Enable debug mode?" --choices "yes,no" --default "no")

# Get database configuration
db_type=$(askr "Database type:" --choices "postgresql,mysql,sqlite")
if [ "$db_type" != "sqlite" ]; then
    db_host=$(askr "Database host:" --validate-hostname --default "localhost")
    db_port=$(askr "Database port:" --integer --range 1-65535 --default "5432")
    db_name=$(askr "Database name:" --required)
    db_user=$(askr "Database user:" --required)
    db_password=$(askr "Database password:" --required --mask)
fi

# Generate configuration file
cat > config.json << EOCONFIG
{
    "app_name": "$app_name",
    "environment": "$environment",
    "debug": $([ "$debug_enabled" = "yes" ] && echo "true" || echo "false"),
    "database": {
        "type": "$db_type",
        "host": "${db_host:-}",
        "port": ${db_port:-0},
        "name": "${db_name:-}",
        "user": "${db_user:-}",
        "password": "${db_password:-}"
    }
}
EOCONFIG

echo "✅ Configuration saved to config.json"
EOF

echo
echo "Example 2: User registration script"
cat << 'EOF'
#!/bin/bash
# register.sh - User registration with validation

echo "👤 User Registration"

# Get user details with validation
username=$(askr "Username:" \
    --required \
    --min-length 3 --max-length 20 \
    --pattern "^[a-zA-Z0-9_]+$" \
    --pattern-message "Only letters, numbers, and underscores allowed")

email=$(askr "Email:" --validate-email --required)

password=$(askr "Password:" \
    --required \
    --min-length 8 \
    --pattern ".*[A-Z].*" --pattern-message "Must contain uppercase letter" \
    --pattern ".*[a-z].*" --pattern-message "Must contain lowercase letter" \
    --pattern ".*[0-9].*" --pattern-message "Must contain number" \
    --mask)

# Get user preferences
roles=$(askr "User roles:" \
    --choices "admin,user,moderator,viewer" \
    --max-choices 3 \
    --min-choices 1)

notifications=$(askr "Enable notifications?" \
    --choices "email,sms,push,none" \
    --max-choices 3 \
    --min-choices 0)

# Create user account (placeholder)
echo "Creating user account..."
echo "Username: $username"
echo "Email: $email"
echo "Roles: $roles"
echo "Notifications: $notifications"
echo "✅ User registered successfully!"
EOF

echo
echo "Example 3: Deployment script"
cat << 'EOF'
#!/bin/bash
# deploy.sh - Deployment configuration

echo "🚀 Deployment Configuration"

# Get deployment details
target=$(askr "Deployment target:" --choices "aws,gcp,azure,local")
region=$(askr "Region:" --required --default "us-east-1")

if [ "$target" != "local" ]; then
    instance_type=$(askr "Instance type:" \
        --choices "t3.micro,t3.small,t3.medium,t3.large" \
        --default "t3.small")

    scaling=$(askr "Auto-scaling enabled?" --choices "yes,no" --default "yes")

    if [ "$scaling" = "yes" ]; then
        min_instances=$(askr "Minimum instances:" --integer --range 1-10 --default "1")
        max_instances=$(askr "Maximum instances:" --integer --range 1-100 --default "5")
    fi
fi

# Get application settings
health_check_path=$(askr "Health check path:" --default "/health")
port=$(askr "Application port:" --integer --range 1000-65535 --default "8080")

echo "🔧 Deploying with configuration:"
echo "Target: $target"
echo "Region: $region"
echo "Instance Type: ${instance_type:-N/A}"
echo "Port: $port"
echo "Health Check: $health_check_path"
if [ "$scaling" = "yes" ]; then
    echo "Scaling: $min_instances-$max_instances instances"
fi
EOF

echo
echo "Example 4: JSON output processing"
cat << 'EOF'
#!/bin/bash
# process_json.sh - Using JSON output for complex processing

# Get user input with JSON output
result=$(askr "Email address:" --validate-email --output json)

# Parse JSON response
email=$(echo "$result" | jq -r '.value')
valid=$(echo "$result" | jq -r '.valid')
errors=$(echo "$result" | jq -r '.validation_results[].message // empty')

if [ "$valid" = "true" ]; then
    echo "✅ Valid email: $email"
    # Process valid email...
else
    echo "❌ Invalid email address:"
    echo "$errors"
    exit 1
fi
EOF

echo
echo "Run these examples to see askr in real-world scripting scenarios!"
