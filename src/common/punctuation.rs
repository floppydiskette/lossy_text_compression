use rand::Rng;

pub fn compress(input: impl AsRef<str>) -> String {
    let mut output = input.as_ref().to_string();
    output = output.replace('.', "");
    output = output.replace(',', "");
    output = output.replace('!', "");
    output = output.replace('?', "");
    output = output.replace(';', "");
    output = output.replace(':', "");
    output = output.replace('\'', "");
    output = output.replace('"', "");
    output = output.replace('(', "");
    output = output.replace(')', "");
    output = output.replace('[', "");
    output = output.replace(']', "");
    output = output.replace('{', "");
    output = output.replace('}', "");
    output = output.replace('<', "");
    output = output.replace('>', "");
    output = output.replace('/', "");
    output = output.replace('\\', "");
    output = output.replace('|', "");
    output = output.replace('@', "");
    output = output.replace('#', "");
    output = output.replace('$', "");
    output = output.replace('%', "");
    output = output.replace('^', "");
    output = output.replace('&', "");
    output = output.replace('*', "");
    output = output.replace('-', "");
    output = output.replace('_', "");
    output = output.replace('=', "");
    output = output.replace('+', "");
    output = output.replace('~', "");
    output = output.replace('`', "");
    output
}

pub fn decompress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    let mut random = rand::thread_rng();
    let mut word_counter = 15;
    // for each word...
    for word in input.as_ref().split_whitespace() {
        let mut word = word.to_string();
        // if the word is a number...
        if let Ok(_number) = word.parse::<i32>() {
            // randomly add a dollar sign
            if random.gen_bool(0.5) {
                word = format!("${}", word);
            }
        }

        // if word counter is 0, add a period; if word counter is 15 / 2, add a comma
        if word_counter == 0 {
            word = format!("{}.", word);
            word_counter = 15;
        } else if word_counter == 15 / 2 {
            word = format!("{},", word);
            word_counter -= 1;
        } else {
            word_counter -= 1;
        }

        // randomly add (or replace the period with) a question mark
        if random.gen_bool(0.05) {
            if word.ends_with('.') {
                word = word.replace('.', "?");
            } else {
                word = format!("{}?", word);
            }
        }

        // randomly replace comma with semicolon
        if random.gen_bool(0.05) {
            word = word.replace(',', ";");
        }

        output.push_str(&format!("{} ", word));
    }
    output.trim().to_string()
}