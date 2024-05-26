use phf::phf_map;
use bimap::BiMap;

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

fn map_by_pair(pairs: &str) -> BiMap<u8, u8> {
    let mut map = BiMap::new();
    let mut chars = pairs.chars();
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
        map.insert(c1 as u8, c2 as u8);
    }
    map
}

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
        if let Some(new_c) = self.settings.get_by_left(&c) {
            *c = *new_c;
        }
    }

    pub fn through_backwards(&self, c: &mut u8) {
        if let Some(new_c) = self.settings.get_by_right(&c) {
            *c = *new_c;
        }
    }

    pub fn turn(&mut self) {
        self.curr_pos = (self.curr_pos + 1) % 26;
    }

    pub fn at_notch(&self) -> bool {
        self.curr_pos == self.notch_pos
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

struct Enigma {
    plugboard: Plugboard,
    reflector: Reflector,
    rotors: [Rotor; 3], 
}

impl Enigma {
    pub fn new(plug_pairs: &str, reflector_name: &str, rot1_name: &str, rot1_pos: u8, 
        rot2_name: &str, rot2_pos: u8, rot3_name: &str, rot3_pos: u8) -> Self {
        Enigma {
            plugboard: Plugboard::new(plug_pairs),
            reflector: Reflector::new(reflector_name),
            rotors: [
                Rotor::new(rot1_name, rot1_pos), 
                Rotor::new(rot2_name, rot2_pos), 
                Rotor::new(rot3_name, rot3_pos)
            ],
        }
    }

    pub fn encode(&mut self, plaintext: &str) -> String{
        let mut ciphertext = String::new();
        for c in plaintext.chars() {
            let mut c_byte = c as u8;
            if c.is_alphabetic() {
                self.through(&mut c_byte);
            }
            ciphertext.push(c_byte as char);
        }
        ciphertext
    }

    pub fn through(&mut self, c: &mut u8) {
        // self.turn_rotors();
        self.plugboard.through(c);
        // self.through_rotors_forwards(c);
        self.reflector.through(c);
        // self.through_rotors_backwards(c);
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

fn main() {
    let mut enigma = Enigma::new(
        "abghfi",
        "UKWA",
        "II", 0,
        "I", 24,
        "V", 9,
    );
    let plaintext = "I love cheese";
    let ciphertext = enigma.encode(plaintext);
    println!("{}", ciphertext);
}