pub fn gen_id(dest: &mut String) {
    let mut letters_remaining = 4;
    let mut digits_remaining = 2;
    for _ in 0..(letters_remaining + digits_remaining) {
        let gen_letter = if letters_remaining > 0 && digits_remaining > 0 {
            rand::random()
        } else {
            letters_remaining > 0
        };

        if gen_letter {
            let letter = rand::random_range('a'..='z');
            dest.push(letter);
            letters_remaining -= 1;
        } else {
            let digit = rand::random_range('0'..='9');
            dest.push(digit);
            digits_remaining -= 1;
        }
    }
}

pub fn gen_email(dest: &mut String, prefix: &str, host: &str) {
    dest.push_str(prefix);
    dest.push('.');
    gen_id(dest);
    dest.push('@');
    dest.push_str(host);
}
