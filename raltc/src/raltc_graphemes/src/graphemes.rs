// Get first grapheme in 'file'
pub fn get_grapheme(file: &mut String) -> String {
    let character: char = file.remove(0);
    let mut grapheme: String = String::from(character);

    match character {

        // Graphemes //

        '\r' => {
            if !file.is_empty() && file.chars().next().unwrap() == '\n' {
                grapheme.push(file.remove(0));
            }
        },

        _ => {
            // Hindi graphemes
            if !file.is_empty() && file.chars().next().unwrap() == '्' &&
                is_character_hindi(character) {
                grapheme.push(file.remove(0));
            }

            // Other: unicode character
        },
    }
    
    grapheme
}

fn is_character_hindi(character: char) -> bool {
    match character {
        // Characters without support of virama ( ੍ ): 'ॐ'
        'अ' | 'आ' | 'ए' | 'ई' | 'ऍ' | 'ऎ' | 'ऐ' | 'इ' | 'ओ' | 'ऑ' |
        'ऒ' | 'ऊ' | 'औ' | 'उ' | 'ब' | 'भ' | 'च' | 'छ' | 'ड' | 'ढ' |
        'फ' | 'फ़' | 'ग' | 'घ' | 'ग़' | 'ह' | 'ज' | 'झ' | 'क' | 'ख' |
        'ख़' | 'ल' | 'ळ' | 'ऌ' | 'ऴ' | 'ॡ' | 'म' | 'न' | 'ङ' | 'ञ' |
        'ण' | 'ऩ' | 'प' | 'क़' | 'र' | 'ऋ' | 'ॠ' | 'ऱ' | 'स' | 'श' |
        'ष' | 'ट' | 'त' | 'ठ' | 'द' | 'थ' | 'ध' | 'ड़' | 'ढ़' | 'व' |
        'य' | 'य़' | 'ज़' => { true },

        _ => { false },
    }
}
