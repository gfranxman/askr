#!/bin/bash
# Basic usage examples for askr

echo "=== Basic askr Examples ==="

echo
echo "1. Simple text input:"
echo "askr \"Enter your name:\""

echo
echo "2. Required input with length validation:"
echo "askr \"Username:\" --required --min-length 3 --max-length 20"

echo
echo "3. Email validation:"
echo "askr \"Email address:\" --validate-email --required"

echo
echo "4. Number with range:"
echo "askr \"Port number:\" --integer --range 1024-65535"

echo
echo "5. Password with masking:"
echo "askr \"Password:\" --required --min-length 8 --mask"

echo
echo "6. Single choice selection:"
echo "askr \"Environment:\" --choices \"dev,staging,prod\""

echo
echo "7. Multiple choice selection:"
echo "askr \"Select features:\" --choices \"auth,db,cache,api\" --max-choices 3"

echo
echo "8. JSON output:"
echo "askr \"Email:\" --validate-email --output json"

echo
echo "Run any of these commands to see askr in action!"