pub fn compress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    for word in input.as_ref().split_whitespace() {
        output.push_str(word);
    }
    output
}

pub fn decompress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    let mut max_word_length = 20;
    let mut word_index = 0;
    let mut plural_buffer = String::new();
    let mut ing_buffer = String::new();
    let mut possessive_buffer = String::new();
    for c in input.as_ref().chars() {
        // if we've reached the max word length, next word
        if max_word_length == 0 {
            output.push(' ');
            max_word_length = 20;
            word_index = 0;
            plural_buffer.clear();
            ing_buffer.clear();
            possessive_buffer.clear();
            continue;
        }
        max_word_length -= 1;

        // if word index is greater than 2 and the character is a 'd', next word
        if word_index > 2 && c == 'd' {
            output.push(' ');
            max_word_length = 20;
            word_index = 0;
            plural_buffer.clear();
            ing_buffer.clear();
            possessive_buffer.clear();
            continue;
        }

        // if the last three letters were 'ing', next word
        if ing_buffer.len() == 3 {
            ing_buffer.remove(0);
            ing_buffer.push(c);
            if ing_buffer == "ing" {
                output.push(' ');
                max_word_length = 20;
                word_index = 0;
                plural_buffer.clear();
                ing_buffer.clear();
                possessive_buffer.clear();
                continue;
            }
        } else {
            ing_buffer.push(c);
        }

        // if the last three letters were ies, next word
        if plural_buffer.len() == 3 {
            plural_buffer.remove(0);
            plural_buffer.push(c);
            if plural_buffer == "ies" {
                output.push(' ');
                max_word_length = 20;
                word_index = 0;
                plural_buffer.clear();
                ing_buffer.clear();
                possessive_buffer.clear();
                continue;
            }
        } else {
            plural_buffer.push(c);
        }

        // if the last two letters were '\'s', next word
        if possessive_buffer.len() == 2 {
            possessive_buffer.remove(0);
            possessive_buffer.push(c);
            if possessive_buffer == "'s" {
                output.push(' ');
                max_word_length = 20;
                word_index = 0;
                plural_buffer.clear();
                ing_buffer.clear();
                possessive_buffer.clear();
                continue;
            }
        } else {
            possessive_buffer.push(c);
        }

        output.push(c);
    }
    output
}