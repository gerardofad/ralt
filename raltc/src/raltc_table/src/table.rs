enum Table { // Table of tokens
    Keyword,     // fnt | main
    Name,        // !name-name | _name_1_
    String,      // "Hi!" | ¨Hi!¨ | 'Hi!'
    StringChars, // "Hi!" -> "स्"
    StringUnits, // ¨Hi!¨ -> ¨स¨ ¨ ्¨
    StringBytes, // 'Hi!' -> '!' -> 0x33
    Number,      // 100 000.000 000 | 100_000.000_000
    NumberInt,   // 100 000 | 100_000
    NumberFloat, // 1.000 000 | 1.000_000
    Byte,        // 0x3D | 0xA3
    Space,
    Illegal,
}
