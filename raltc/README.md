# Guide to raltc development in Rust

## Cleaner (core: 'raltc_cleaner')
Remove comments (one-line ('//...') or multi-line ('/\*...\*/' or "/'...'/"))

```Ralt
// Hi!
/* Hi! */
/' Hi! '/
```

> Needs and returns a 'Script' struct, which itself contains all the text of the file in 'Item's by characters with their respective file position
