#!/bin/bash

echo "Testing line editing enhancements..."

echo ""
echo "Testing prompt with different validators:"
echo ""

echo "1. Testing file exists validator:"
echo "   Try entering a path and use arrow keys to edit"
echo "   Use Ctrl+A to jump to beginning, Ctrl+E to end"
echo "   Use Ctrl+K to delete to end, Ctrl+U to delete to beginning"
echo ""

echo "2. Testing choice validator:"
echo "   Valid choices: red, green, blue"
echo "   Try typing partial matches and using arrow keys"
echo ""

echo "3. Testing date validator:"
echo "   Enter date in YYYY-MM-DD format"
echo "   Use arrow keys to move cursor and edit"
echo ""

echo "Press Enter to continue..."
read

echo "Test 1: File exists validator"
./target/debug/prompt --file-exists "Enter a file path:"
echo "Result: $?"
echo ""

echo "Test 2: Choice validator" 
./target/debug/prompt --choices "red,green,blue" "Pick a color:"
echo "Result: $?"
echo ""

echo "Test 3: Date validator"
./target/debug/prompt --date "Enter date (YYYY-MM-DD):"
echo "Result: $?"
echo ""

echo "Line editing test complete!"