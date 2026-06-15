use rand::prelude::*;

fn build_ascii_character_vector(special_characters: bool) -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    let mut _character_code: u32 = 33;

    while _character_code < 127 {

        if _character_code == 34 {
            _character_code += 1;
            continue;
        }

        if special_characters {
            if char::from_u32(_character_code).unwrap().is_ascii_graphic()
                && !char::from_u32(_character_code).unwrap().is_ascii_alphanumeric() {
                (&mut vector).push(_character_code);
            }
        }
        else {
            if char::from_u32(_character_code).unwrap().is_ascii_alphanumeric() {
                (&mut vector).push(_character_code);
            }
        }

        _character_code += 1;
    }

    return vector;
}

fn build_password(use_special_characters: bool, password_length: u8) -> String {
    let mut rng = rand::rng();
    let mut character: char;
    let mut counter: u8 = 0;
    let mut password_string: String = String::with_capacity(password_length as usize);
    let character_vector: Vec<u32> = build_ascii_character_vector(false);

    while counter < password_length {
        let index = rng.random_range(..character_vector.len()) as u32;
        character = char::from_u32(character_vector[index as usize]).unwrap();
        (&mut password_string).push(character);
        counter += 1;
    }

    if use_special_characters {
        password_string = add_special_characters(password_length, password_string);
    }

    return password_string
}

fn add_special_characters(password_length: u8, password_string: String) -> String {
    let mut rng = rand::rng();
    let mut _rand_index: u32;
    let mut _password_vector: Vec<char> = password_string.chars().collect();
    let mut special_character_count: u8 = 0;
    let character_vector: Vec<u32> = build_ascii_character_vector(true);
    let mut _character: char;

    // Set special character limit to a value between 1 half of the password's length
    let special_character_limit: u8 = (&mut rng).random_range(1..(password_length / 2));

    // Replace alphanumeric characters with special characters until the limit is reached
    if special_character_count < special_character_limit {
        while special_character_count < special_character_limit {
            _rand_index = (&mut rng).random_range(.._password_vector.len()) as u32;
            _character = _password_vector[_rand_index as usize];
            if _character.is_ascii_graphic() && !_character.is_ascii_alphanumeric() {
                continue;
            }
            else {
                let graphic_index = (&mut rng).random_range(..character_vector.len()) as u32;
                _password_vector[_rand_index as usize] = char::from_u32(character_vector[graphic_index as usize]).unwrap();
                special_character_count += 1;
            }
        }
    }

    return _password_vector.into_iter().collect();
}

pub fn create_password(use_spec_char: bool, length: u8) -> String {
    let pass_length: u8 = length.clamp(8,24);
    return build_password(use_spec_char, pass_length);
}