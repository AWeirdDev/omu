To make all fields public, first hit <kbd>⌘</kbd> <kbd>F</kbd>, apply "Regex" and "match case."

Then, add this to the find box:
```re
\s[a-z_]+: 
```

Use <kbd>⌘</kbd> <kbd>Shift</kbd> <kbd>L</kbd> to use multi-line selection, and then add your `pub` using the multi-cursors!
