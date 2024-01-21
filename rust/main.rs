use std::io;
use std::io::Write;

// implements Clone trait to allow for copying of structs:
#[derive(Clone)]
struct Rotor {
    wiring: [char; 26],
    notch: char,
    position: usize,
    increment: usize,
}

// implements Clone trait:
#[derive(Clone)]
struct RotorSet {
    rotors: Vec<Rotor>,
    reflector: [char; 26],
    repeat: usize,
}

// Alphabet:
static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Rotor {
    fn forward(&self, pos: usize) -> usize {
        // Iterated through the alphabet and returns the index of the character that matches the character at the given position in the wiring array or 0 if no match is found:
        ALPHA.iter().position(|&c| c == self.wiring[pos]).unwrap_or(0)
    }

    fn backward(&self, pos: usize) -> usize {
        // Iterated through the alphabet and returns the index of the character that matches the character at the given position in the wiring array or 0 if no match is found:
        self.wiring.iter().position(|&c| c == ALPHA[pos]).unwrap_or(0)
    }

    fn increment_rotor(&self) -> Rotor {
        // Creates a new rotor with the same wiring and notch as the current rotor but with a new position and increment:
        let new_position = (self.position + self.increment) % self.wiring.len();
        Rotor { position: new_position, ..*self }
    }

    fn reached_notch(&self) -> bool {
        // Check if the rotor has reached its notch:
        self.wiring[self.position] == self.notch
    }
}

impl RotorSet {
    fn rotate(&self) -> RotorSet {
        // Get length of rotors:
        let rs_len = self.rotors.len() - 1;

        // Check if double rotation is needed on any rotor:
        let double_rotated_rotors: Vec<Rotor> = self.rotors.iter().rev().map(|rotor| {
            if rotor.reached_notch() {
                rotor.increment_rotor()
            } else {
               rotor.clone()
            }            
        }).collect();

        // Rotate the next rotors if this rotor has reached its notch or rotate the rightmost rotor:
        let rotated_rotors: Vec<Rotor> = double_rotated_rotors.iter().rev().enumerate().map(|(i, rotor)| {
            if i == rs_len || rotor.reached_notch() {
                rotor.increment_rotor()
            } else {
               rotor.clone()
            }            
        }).collect();

        // Return a new RotorSet with the rotated rotors:
        RotorSet { rotors: rotated_rotors, ..*self }
    }

    fn traverse_forward(&self, pos: usize) -> usize {
        // Fold over the rotors in reverse order and traverse forward through each rotor:
        self.rotors.iter().enumerate().rev().fold(pos, |acc, (i, rotor)| {
            let f_pos = rotor.forward(acc);

            if i != 0 {
                (f_pos as i32 - rotor.position as i32 + self.rotors[i - 1].position as i32).rem_euclid(rotor.wiring.len() as i32) as usize
            } else {
                f_pos
            }
        })
    }

    fn traverse_backward(&self, pos: usize) -> usize {
        // Fold over the rotors in order and traverse backward through each rotor:
        self.rotors.iter().enumerate().fold(pos, |acc, (i, rotor)| {
            let b_pos = rotor.backward(acc);

            if i < self.rotors.len() - 1 {
                (b_pos as i32 - rotor.position as i32 + self.rotors[i + 1].position as i32).rem_euclid(rotor.wiring.len() as i32) as usize
            } else {
                b_pos
            }
        })
    }

    fn reflect(&self, pos: usize) -> usize {
        // Iterate through the reflector and return the index of the character that matches the character at the given position in the alphabet or 0 if no match is found:
        self.reflector.iter().position(|&c| c == ALPHA[pos]).unwrap_or(0)
    }

    fn encode_char(&self, ch: char, rs: &RotorSet) -> char {
        // Check if character is a space:
        if ch == ' ' {
            return ' ';
        }
    
        // Get the position of the character in the alphabet and ajust for the position of the rightmost rotor:
        let pos = ALPHA.iter().position(|&c| c == ch).unwrap_or(0);
        let w_pos = (pos + rs.rotors.last().unwrap().position) % ALPHA.len();
    
        // Traverse forward through the rotors, reflect, and traverse backward through the rotors:
        let f_pos = (0..rs.repeat).fold(w_pos, |acc, _| rs.traverse_forward(acc));
        let r_pos = rs.reflect(f_pos);
        let b_pos = (0..rs.repeat).fold(r_pos, |acc, _| rs.traverse_backward(acc));
        
        // Adjust for the position of the rightmost rotor and return the character at the given position in the alphabet:
        let a_pos = (b_pos + ALPHA.len() - rs.rotors.last().unwrap().position) % ALPHA.len();
        ALPHA[a_pos]
    }
    
    fn encode(&self, message: &str) -> String {        
        // Fold over the characters in the message and encode each character:
        let (encoded_message, _) = message.chars().fold((Vec::new(), self.clone()), |(acc, rs), ch| {
            // Rotate the rotors:
            let rs = rs.rotate();
    
            // Encode the character:
            let new_char = self.encode_char(ch, &rs);
            let new_acc = [acc.as_slice(), &[new_char]].concat();
            
            // Return the new encoded message and rotor set:
            (new_acc, rs)
        });
    
        // Return the encoded message:
        encoded_message.into_iter().collect()
    }
}

fn main() {
    // Create rotors:
    let r1 = Rotor {
        wiring: [ 'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J' ],
        notch: 'Q',
        position: 0,
        increment: 1,
    };

    let r2 = Rotor {
        wiring: [ 'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E' ],
        notch: 'E',
        position: 0,
        increment: 1,
    };

    let r3 = Rotor {
        wiring: [ 'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O' ],
        notch: 'V',
        position: 0,
        increment: 1,
    };

    // Create rotor set:
    let mut rs = RotorSet {
        rotors: vec![r1, r2, r3],
        reflector: [ 'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T' ],
        repeat: 1,
    };

    // Get rotor positions and increments from user:
    for (i, rotor) in rs.rotors.iter_mut().enumerate() {
        let mut rotor_input = String::new();

        print!("Enter rotor {} position and increment (0 - 25): ", i);
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut rotor_input).unwrap();
        let mut parts = rotor_input.trim().split_whitespace();

        let error = "Error: rotor position and increment must be between 0 and 25";

        let r_pos: usize = parts.next().unwrap_or_else(|| {
            eprintln!("{}", error);
            std::process::exit(1);
        }).parse().unwrap_or_else(|_| {
            eprintln!("{}", error);
            std::process::exit(1);
        });
        let r_inc: usize = parts.next().unwrap_or_else(|| {
            eprintln!("{}", error);
            std::process::exit(1);
        }).parse().unwrap_or_else(|_| {
            eprintln!("{}", error);
            std::process::exit(1);
        });

        if r_pos > 25 || r_inc > 25 {
            eprintln!("{}", error);
            std::process::exit(1);
        }
        rotor.position = r_pos;
        rotor.increment = r_inc;
    }

    // Get repeat from user:
    print!("Enter repeat: ");
    io::stdout().flush().unwrap();
    let mut repeat = String::new();
    io::stdin().read_line(&mut repeat).unwrap();
    rs.repeat = repeat.trim().parse().unwrap_or_else(|_| {
        eprintln!("Error: repeat must be an integer");
        std::process::exit(1);
    });

    // Get message from user:
    print!("Enter message: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    // Sanitize message:
    let message = message.to_uppercase().trim().to_string();

    // Encode message:
    if message.chars().all(|ch| ALPHA.contains(&ch)) {
        let encoded_message = rs.encode(&message);
        println!("Encoded message: {}", encoded_message);
    } else {
        println!("Error: message contains invalid characters");
    }
}