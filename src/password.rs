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

pub fn create_password(use_spec_char: bool, length: u8) -> String {
    let mut rng = rand::rng();
    let mut character: char;
    let mut password_tmp_string: String = String::new();
    let mut counter: u8 = 0;

    if use_spec_char {
        let special_character_vec: Vec<u32> = build_spec_char_vec();
        let vec_length: u8 = special_character_vec.len() as u8;

        while counter < length {
            let index:u32 = (&mut rng).random_range(..vec_length) as u32;
            character = char::from_u32(special_character_vec[index as usize]).unwrap();
            password_tmp_string = password_tmp_string + character.to_string().as_str();
            counter += 1;
        }
    }
    else {
        let character_vec: Vec<u32> = build_char_vec();
        let vec_length: u8 = character_vec.len() as u8;

        while counter < length {
            let index = rng.random_range(..vec_length) as u32;
            character = char::from_u32(character_vec[index as usize]).unwrap();
            password_tmp_string = password_tmp_string + character.to_string().as_str();
            counter += 1;
        }
    }

    return password_tmp_string;
}