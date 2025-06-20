# UI Behavior Specification

## Interactive Mode Layout

### Smart Prompt Placement

The prompt intelligently reserves screen space to provide a stable UI experience, especially when used in scripts with existing terminal output.

#### Space Reservation Algorithm
1. **Calculate required space**: Analyze all validators to determine maximum possible error messages
2. **Account for text wrapping**: Calculate line wrapping based on terminal width
3. **Reserve terminal lines**: Print blank lines to claim screen real estate
4. **Position prompt**: Move cursor back up to reserved prompt position
5. **Stable display**: All validation content appears within reserved space

#### Benefits
- **Script-friendly**: Previous terminal content remains visible
- **No scrolling**: All errors visible simultaneously without pushing content off-screen
- **Predictable**: Consistent UI behavior regardless of terminal scroll position
- **Professional**: Stable, polished interaction suitable for production scripts

### Basic Layout Structure
```
[Previous script output remains visible]
[Reserved space - calculated based on validators]
↑ Cursor positioned here ↑
[PROMPT_TEXT] [USER_INPUT]█
[ERROR_AREA - Dynamic Height within reserved space]
[HELP_TEXT - Optional]
```

### Example Display
```
Enter hostname: my-server..example.com█
❌ Must not contain consecutive dots
❌ Maximum length is 253 characters (currently 264)
✅ Contains only valid characters

Press Ctrl+C to cancel, Enter to submit
```

## Real-time Input Feedback

### Text Input Coloring

#### Color Scheme
- **Valid text**: Default terminal color (usually white/black)
- **Invalid text**: Red foreground
- **Cursor**: Normal cursor behavior
- **Selection**: Standard terminal selection colors

#### Coloring Logic
1. **Partial validation** runs on each keystroke
2. **First error position** determined by validators
3. **Text coloring** applied from first error position to end
4. **Multiple errors**: Color from earliest failure point

#### Examples
```
# Valid input (all default color)
Enter email: user@example.com█

# Invalid from position 4 (red from @)
Enter email: user@█

# Invalid from position 12 (red from second dot)
Enter hostname: my-server..com█
```

### Dynamic Error Display

#### Error Area Behavior
- **Height**: Grows/shrinks based on number of errors to display
- **Clearing**: Previous errors cleared before redrawing
- **Scrolling**: If errors push past terminal bottom, scroll appropriately
- **Wrapping**: Long error messages wrap to fit terminal width

#### Error Message Format
```
[ICON] [PRIORITY] [MESSAGE]

Icons:
❌ - Critical/High errors (blocking)
⚠️  - Medium errors (warnings)
💡 - Low errors (suggestions)
✅ - Passed validations (optional, for complex forms)
```

#### Priority-based Display Rules
1. **Always show**: Critical and High priority errors
2. **Show up to 3**: Medium priority errors
3. **Show up to 2**: Low priority errors (only if no higher priority)
4. **Truncation**: "... and N more errors" if too many

### Terminal Control

#### Cursor Management
```rust
// Pseudo-code for cursor handling
struct CursorState {
    input_line: usize,
    input_column: usize,
    error_area_start: usize,
    error_area_height: usize,
}

impl TerminalUI {
    fn save_cursor(&mut self);
    fn restore_cursor(&mut self);
    fn clear_error_area(&mut self);
    fn redraw_errors(&mut self, errors: &[ValidationError]);
}
```

#### Screen Updates
1. **On keystroke**:
   - Update input line with new character + coloring
   - Run partial validation
   - Clear and redraw error area if needed
   - Restore cursor to input position

2. **On backspace/delete**:
   - Update input line
   - Revalidate from changed position
   - Update error display
   - Restore cursor

3. **On terminal resize**:
   - Recalculate available width
   - Rewrap error messages
   - Redraw entire display

## Keyboard Handling

### Text Input Mode

#### Character Input
- **Printable characters**: Add to input buffer, trigger validation
- **UTF-8 support**: Handle multi-byte characters correctly
- **Control characters**: Handle appropriately (tab, etc.)

#### Special Keys
- **Enter**: Submit input (validate and return/continue)
- **Ctrl+C**: Cancel input, exit with code 130
- **Ctrl+D**: EOF, exit with code 1 (only when input is empty)
- **Backspace**: Remove character before cursor, revalidate
- **Delete**: Remove character at cursor, revalidate
- **Arrow keys**: Left/Right cursor movement within input
- **Home/End**: Jump to beginning/end of input
- **Tab**: Auto-completion for choice validators (if applicable)

### Choice Menu Mode

#### Navigation Keys
- **Up/Down arrows**: Navigate between choices
- **Enter**: Select current choice (single mode) or submit selections (multi mode)
- **Space**: Toggle selection (multiple choice mode only)
- **Ctrl+C**: Cancel choice menu, exit with code 130
- **Ctrl+D**: EOF, exit with code 1

#### Real-time Validation
- **Selection validation**: Check min/max constraints after each toggle
- **Visual feedback**: Display validation errors below choice list
- **Submit prevention**: Block Enter submission when constraints not met
- **Error display**: Show "At least N choice(s) required" or "At most N choice(s) allowed"

#### Enhanced Line Editing (Emacs-style)
- **Ctrl+A**: Jump to beginning of line
- **Ctrl+E**: Jump to end of line
- **Ctrl+K**: Kill (delete) from cursor to end of line
- **Ctrl+U**: Kill from beginning of line to cursor
- **Ctrl+W**: Delete word before cursor
- **Insert mode**: Characters inserted at cursor position (not appended)

#### Paste Handling
- **Large pastes**: Handle efficiently without lag
- **Validation**: Run validation on complete pasted content
- **Multi-line**: Reject or convert to single line based on context

### Navigation and Editing

#### Cursor Movement
- **Left/Right arrows**: Move cursor within input (character-wise)
- **Home/End**: Jump to start/end of line
- **Ctrl+A/Ctrl+E**: Alternative jump to start/end (Emacs-style)
- **Unicode-aware**: Properly handles multi-byte characters
- **Visual feedback**: Cursor positioned exactly where expected
- **Real-time validation**: Validation updates as cursor moves

#### Text Selection
- **Shift+Arrows**: Select text
- **Ctrl+A**: Select all
- **Delete/Backspace**: Remove selected text
- **Typing**: Replace selected text

#### Undo/Redo (Future Enhancement)
- **Ctrl+Z**: Undo last change
- **Ctrl+Y**: Redo change
- **History**: Maintain edit history for session

## Visual Feedback

### Color Schemes

#### Default Color Scheme
```
Valid text:       Default (white/black)
Invalid text:     Red (bright_red)
Cursor:          Default
Error icons:     Red (❌), Yellow (⚠️), Blue (💡)
Success icons:   Green (✅)
Prompt text:     Bold
Help text:       Dim/gray
```

#### No-Color Mode
```
Valid text:       No change
Invalid text:     No change (rely on error messages)
Error markers:    [ERROR], [WARN], [INFO], [OK]
Emphasis:         UPPERCASE, *asterisks*
```

### Animation and Transitions

#### Error Message Transitions
- **Appear**: Instant display (no fade-in)
- **Disappear**: Clear immediately when fixed
- **Change**: Clear old, display new (no morphing)
- **Performance**: Minimize screen flicker

#### Text Coloring Updates
- **Real-time**: Update colors as user types
- **Smooth**: No visible flicker or lag
- **Efficient**: Only update changed portions

### Accessibility

#### Screen Reader Support
- **Error announcements**: Announce new errors as they appear
- **Status updates**: Announce validation status changes
- **Input description**: Announce input requirements upfront

#### High Contrast Mode
- **Detection**: Detect system high contrast settings
- **Colors**: Use system-appropriate high contrast colors
- **Fallback**: Bold/italic text when color unavailable

#### Keyboard-only Operation
- **No mouse required**: All functionality via keyboard
- **Standard shortcuts**: Follow common terminal conventions
- **Focus indication**: Clear focus indication for all elements

## Terminal Compatibility

### Terminal Types
- **Modern terminals**: Full color and cursor support (iTerm2, Windows Terminal, etc.)
- **Basic terminals**: Fallback to simple text (basic xterm)
- **Legacy terminals**: Minimal functionality, focus on core features

### Feature Detection
```rust
struct TerminalCapabilities {
    colors_256: bool,
    colors_truecolor: bool,
    cursor_movement: bool,
    alternate_screen: bool,
    mouse_support: bool,
}

impl TerminalCapabilities {
    fn detect() -> Self;
    fn fallback_ui(&self) -> UIMode;
}
```

### Fallback Behavior
- **No color support**: Use text markers and formatting
- **Limited cursor control**: Simpler UI with less real-time feedback
- **Minimal terminals**: Basic prompt with post-input validation

## Performance Requirements

### Response Time Targets
- **Keystroke response**: < 16ms (60fps feel)
- **Validation display**: < 100ms for simple validators
- **Complex validation**: < 500ms with loading indicator
- **Terminal resize**: < 200ms to redraw

### Resource Usage
- **Memory**: Minimal allocation during input processing
- **CPU**: Low CPU usage during idle periods
- **Terminal I/O**: Batch updates to minimize syscalls

### Optimization Strategies
- **Dirty regions**: Only redraw changed areas
- **Debounced validation**: Avoid excessive validation calls
- **Lazy error formatting**: Format messages only when displayed
- **String interning**: Reuse common error messages
