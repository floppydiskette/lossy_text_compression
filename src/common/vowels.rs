use rand::Rng;

fn is_vowel(c: char) -> bool {
    let mut random = rand::thread_rng();
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => {
            if c == 'y' || c == 'Y' {
                random.gen_bool(0.5) // sometimes y is a vowel
            } else {
                false
            }
        }
    }
}

pub fn compress(input: impl AsRef<str>, intense: bool) -> String {
    let mut output = String::new();
    for word in input.as_ref().split_whitespace() {
        // does this word have a vowel?
        let mut has_vowel = false;
        let mut vowel_indices = Vec::new();
        for (i, c) in word.chars().enumerate() {
            if is_vowel(c) {
                has_vowel = true;
                vowel_indices.push(i);
            }
        }

        // for each vowel that isn't the first or last letter...
        if has_vowel {
            let mut word = word.to_string();
            for i in vowel_indices {
                if (i != 0 && i != word.len() - 1) || intense {
                    word = format!("{}{}", &word[..i], &word[i + 1..]);
                }
            }
            output = format!("{} {}", output, word);
        } else {
            output = format!("{} {}", output, word);
        }
    }
    output.trim().to_string()
}

pub fn decompress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    let mut consonant_counter = 0;
    for word in input.as_ref().split_whitespace() {
        // if this word ends with an 'e', add an 'i' before it
        let mut word = word.to_string();
        if word.ends_with('e') || word.ends_with('E') {
            word = format!("{}ie", &word[..word.len() - 1]);
        }

        // if this word ends with an i, add an 'e' before it
        if word.ends_with('i') || word.ends_with('I') {
            word = format!("{}e", &word[..word.len() - 1]);
        }

        // for each consonant...
        for c in word.clone().chars() {
            if !is_vowel(c) {
                consonant_counter += 1;
                if consonant_counter == 5 {
                    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
                    let mut random = rand::thread_rng();
                    let vowel = VOWELS[random.gen_range(0..VOWELS.len())];
                    word = format!("{}{}{}", &word[..word.len() - 1], vowel, &word[word.len() - 1..]);
                    consonant_counter = 0;
                }
            }
        }

        output = format!("{} {}", output, word);
    }
    output.trim().to_string()
}