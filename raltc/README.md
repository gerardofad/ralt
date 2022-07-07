# Guide to raltc development in Rust

### Remove

raltc_cleaner
raltc_core_error
raltc_error
raltc_language_error
raltc_script
raltc_tokenizer
raltc_tokens

---

## Cleaner (core: 'raltc_cleaner')
Remove comments (one-line ( //... ) or multi-line ( /\*...\*/ | /'...'/ ))

```Ralt
// Hi!
/* Hi! */
/' Hi! '/
```

> Needs and returns a 'Script' struct, which itself contains all the text of the file in 'Item's by characters with their respective file position

> Handles errors, replaces whitespaces joined into 1 normal space (including whitespaces between comments) and does not remove 'comments', nor whitespaces within strings
