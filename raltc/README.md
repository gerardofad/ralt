# Guide to raltc development in Rust

## Script (core: 'raltc_script')
File handler by tokens ('Item's) with support for graphemic characters ('स्' of characters: 'स' & ' ्') -> (lexer style)

**Script:** *stores the contents of 1 file in tokens with custom patterns*

---

**Char:** *graphemic character manager of the files (graphemic character: 'स्' of characters: 'स' & ' ्')*

```Ralt
let character: Char = Char::new();
```
> start a new script manager (empty)

```Ralt
character.from_str("I");
```
> assign a value by means of a string literal ( ".." )

```Ralt
character.from_string(String::from("I"));
```
> assignment of a value by means of a standard string module ( String::from("..") | "..".to_string() )

```Ralt
let transferred_character: Char = character.give(); // char: '', transfer: 'I'
```
> transfer its value, eliminating its own

```Ralt
let cloned_character: &str = character.clone(); // char: 'I', clone: 'I'
```
> transfer its value, keeping yours intact

```Ralt
character.remove(); // char: ''
```
> only remove its value (to empty)

```Ralt
let character_as_str: &str = character.as_str(); // I
```
> obtain value as a 'basic' string ( &str )

```Ralt
let character_as_string: String = character.as_string(); // I
```
> get the value as a standard string module ( String )
