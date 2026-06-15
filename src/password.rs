use rand::prelude::*;

fn build_spec_char_vec() -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    let mut counter: u32 = 33;
    while counter < 127 {
        if counter == 34 {
            counter += 1;
            continue;
        }
        vector.push(counter);
        counter += 1;
    }

    return vector;
}

fn build_char_vec() -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    let mut counter: u32 = 48;
    while counter < 123 {
        if counter == 58
            || counter == 59
            || counter == 60
            || counter == 61
            || counter == 62
            || counter == 63
            || counter == 64
            || counter == 91
            || counter == 92
            || counter == 93
            || counter == 94
            || counter == 95
            || counter == 96 {
            counter += 1;
            continue;
        }
        vector.push(counter);
        counter += 1;
    }
    return vector;
}

fn build_password(password_length: u8, character_vector: Vec<u32>, vector_length: u8) -> String {
    let mut rng = rand::rng();
    let mut character: char;
    let mut counter: u8 = 0;
    let mut password_string: String = String::with_capacity(password_length as usize);

    while counter < password_length {
        let index = rng.random_range(..vector_length) as u32;
        character = char::from_u32(character_vector[index as usize]).unwrap();
        (&mut password_string).push(character);
        counter += 1;
    }

    return password_string
}

pub fn create_password(use_spec_char: bool, length: u8) -> String {
    let pass_length: u8 = length.clamp(8,24);
    let password_string: String;

    if use_spec_char {
        let special_character_vec: Vec<u32> = build_spec_char_vec();
        let vec_length: u8 = special_character_vec.len() as u8;

        password_string = build_password(pass_length, special_character_vec, vec_length);
    }
    else {
        let character_vec: Vec<u32> = build_char_vec();
        let vec_length: u8 = character_vec.len() as u8;

        password_string = build_password(pass_length, character_vec, vec_length);
    }

    return password_string;
}