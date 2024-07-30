use phf::phf_map;
use bimap::BiMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnigmaSettings {
    plaintext: String,
    reflector: String,
    rotor1: String,
    #[serde(alias = "rotor1-pos")]
    rotor1_pos: u8,
    rotor2: String,
    #[serde(alias = "rotor2-pos")]
    rotor2_pos: u8,
    rotor3: String,
    #[serde(alias = "rotor3-pos")]
    rotor3_pos: u8,
    plugboard: String,
    #[serde(alias = "case")]
    preserve_case: bool,
    #[serde(alias = "punc")]
    preserve_punctuation: bool,
    #[serde(alias = "space")]
    preserve_spacing: bool,
}

// key: rotor_name
// value: scrambled alphabet, notch position
static ROTOR_SETTINGS: phf::Map<&str, (&str, u8)> = phf_map! {
    "I" => ("ekmflgdqvzntowyhxuspaibrcj", 25),
    "II" => ("ajdksiruxblhwtmcqgznpyfvoe", 13),
    "III" => ("bdfhjlcprtxvznyeiwgakmusqo", 4),
    "IV" => ("esovpzjayquirhxlnftgkdcmwb", 18),
    "V" => ("vzbrgityupsdnhlxawmjqofeck", 8),
};

// key: reflector name
// value: reflector pairs (e -> a, a -> e, j -> b, b -> j, etc...)
static REFLECTOR_SETTINGS: phf::Map<&str, &str> = phf_map! {
    "UKWA" => "eajbmczdlfygxhviwkrnqoupts",
    "UKWB" => "yarbuchdqesflgpixjnkomztwv",
    "UKWC" => "favbpcjdieogyhrkzlxmwntqus",
};

// pairs[0] mapped to pairs[1], pairs[2] to pairs[3], etc... 
fn map_by_pair(pairs: &str) -> BiMap<u8, u8> {
    let mut map = BiMap::new();
    let mut chars = pairs.chars();
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
        map.insert(c1 as u8, c2 as u8);
    }
    map
}

// scrambled_alpha[0] mapped to 'a', scrambled_alpha[1] to 'b', etc...
fn map_against_alpha(scrambled_alpha: &str) -> BiMap<u8, u8> {
    let mut map = BiMap::new();
    let mut alpha_c = b'a';
    for c in scrambled_alpha.chars() {
        map.insert(c as u8, alpha_c);
        alpha_c += 1;
    }
    map
}

struct Rotor {
    settings: BiMap<u8, u8>,
    notch_pos: u8,
    curr_pos: u8,
}

impl Rotor {
    pub fn new(rotor_name: &str, rotor_pos: u8) -> Self {
        let (scrambled_alpha, notch_pos) = match ROTOR_SETTINGS.get(&rotor_name) {
            Some(r) => *r,
            None => *ROTOR_SETTINGS.get("I").unwrap(),
        };
        Self {
            settings: map_against_alpha(scrambled_alpha),
            notch_pos,
            curr_pos: rotor_pos,
        }
    }

    pub fn through_forwards(&self, c: &mut u8) {
        if let Some(new_c) = self.settings.get_by_left(&self.shift(c)) {
            *c = self.shift(new_c);
        }
    }

    pub fn through_backwards(&self, c: &mut u8) {
        if let Some(new_c) = self.settings.get_by_right(&self.unshift(c)) {
            *c = self.unshift(new_c);
        }
    }

    pub fn turn(&mut self) {
        self.curr_pos = (self.curr_pos + 1) % 26;
    }

    pub fn at_notch(&self) -> bool {
        self.curr_pos == self.notch_pos
    }

    fn shift(&self, c: &u8) -> u8 {
        ((*c as i8 - 'a' as i8 + self.curr_pos as i8) % 26) as u8 + 'a' as u8
    }

    fn unshift(&self, c: &u8) -> u8 {
        let offset = (*c as i8 - 'a' as i8) - self.curr_pos as i8;
        let new_c = ((offset + 26) % 26) as u8;
        'a' as u8 + new_c
    }
}

struct Reflector {
    settings: BiMap<u8, u8>,
}

impl Reflector {
    pub fn new(reflector_name: &str) -> Self {
        let pairs = match REFLECTOR_SETTINGS.get(&reflector_name) {
            Some(s) => s,
            None => REFLECTOR_SETTINGS.get("UKWA").unwrap(),
        };
        Self {
            settings: map_by_pair(pairs),
        }
    }

    pub fn through(&self, c: &mut u8) {
        if let Some(new_c) = self.settings.get_by_left(&c)
            .or_else(|| self.settings.get_by_right(&c)) {
            *c = *new_c;
        }
    }
}

struct Plugboard {
    settings: BiMap<u8, u8>,
}

impl Plugboard {
    pub fn new(pairs: &str) -> Self {
        Self {
            settings: map_by_pair(pairs),
        }
    }

    pub fn through(&self, c: &mut u8) {
        if let Some(new_c) = self.settings.get_by_left(&c)
            .or_else(|| self.settings.get_by_right(&c)) {
            *c = *new_c;
        }
    }
}

pub struct Enigma {
    plugboard: Plugboard,
    reflector: Reflector,
    rotors: [Rotor; 3], 
}

impl Enigma {
    pub fn new(settings: &EnigmaSettings) -> Self {
        Enigma {
            plugboard: Plugboard::new(&settings.plugboard),
            reflector: Reflector::new(&settings.reflector),
            rotors: [
                Rotor::new(&settings.rotor1, settings.rotor1_pos),
                Rotor::new(&settings.rotor2, settings.rotor2_pos),
                Rotor::new(&settings.rotor3, settings.rotor3_pos),
            ],
        }
    }

    pub fn encode(&mut self, settings: &EnigmaSettings) -> String {
        let mut ciphertext = String::new();
        let mut index: u8 = 1;
        for c in settings.plaintext.chars() {
            let is_upper = c.is_ascii_uppercase();
            let mut c_byte = c.to_ascii_lowercase() as u8;

            if c.is_alphabetic() {
                self.through(&mut c_byte);
            } else if (c.is_whitespace() && !settings.preserve_spacing) || (!c.is_whitespace() && !settings.preserve_punctuation) {
                continue;
            }

            if is_upper && settings.preserve_case {
                c_byte.make_ascii_uppercase();
            }
            ciphertext.push(c_byte as char);

            if !settings.preserve_spacing {
                index += 1;
                if  index % 6 == 0 {
                    ciphertext.push(' ');
                    index = 1;
                }
            }
        }
        ciphertext
    }

    fn through(&mut self, c: &mut u8) {
        self.turn_rotors();
        self.plugboard.through(c);
        self.through_rotors_forwards(c);
        self.reflector.through(c);
        self.through_rotors_backwards(c);
        self.plugboard.through(c);
    }

    fn turn_rotors(&mut self) {
        let turn_rot2 = self.rotors[0].at_notch();
        let turn_rot3 = self.rotors[1].at_notch();
        if turn_rot3 {
            self.rotors[2].turn();
        }
        if turn_rot2 {
            self.rotors[1].turn();
        }
        self.rotors[0].turn();
    }

    fn through_rotors_forwards(&self, c: &mut u8) {
        for rotor in self.rotors.iter() {
            rotor.through_forwards(c);
        }
    }

    fn through_rotors_backwards(&self, c: &mut u8) {
        for rotor in self.rotors.iter().rev() {
            rotor.through_backwards(c);
        }
    }
}