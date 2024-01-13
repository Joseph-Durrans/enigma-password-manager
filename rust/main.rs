//https://kerkour.com/rust-functional-programming

use std::io;

struct Rotor {
    wiring: [char; 26],
    notch: char,
    position: usize,
    increment: usize,
}

struct RotorSet {
    rotors: Vec<Rotor>,
    reflector: [char; 26],
    repeat: usize,
}

static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Rotor {
    fn forward(&self, pos: usize) -> usize {
        ALPHA.iter().position(|&c| c == self.wiring[pos]).unwrap_or(0)
    }

    fn backward(&self, pos: usize) -> usize {
        self.wiring.iter().position(|&c| c == ALPHA[pos]).unwrap_or(0)
    }

    fn increment_rotor(&mut self) {
        self.position = (self.position + self.increment) % self.wiring.len();
    }

    fn reached_notch(&self) -> bool {
        self.wiring[self.position] == self.notch
    }
}

impl RotorSet {
    fn rotate(&mut self) {
        let rs_len = self.rotors.len() - 1;

        self.rotors[rs_len].increment_rotor();

        for i in (0..rs_len).rev() {
            if self.rotors[i].reached_notch() {
                self.rotors[i].increment_rotor();
            }
        }

        for i in (0..rs_len).rev() {
            if self.rotors[i].reached_notch() {
                self.rotors[i - 1].increment_rotor();
            }
        }
    }

    fn traverse_forward(&self, mut pos: usize) -> usize {
        for (i, rotor) in self.rotors.iter().enumerate().rev() {
            pos = rotor.forward(pos);

            if i != 0 {
                pos = (pos as i32 - rotor.position as i32 + self.rotors[i - 1].position as i32).rem_euclid(rotor.wiring.len() as i32) as usize;
            }
        }

        pos
    }

    fn traverse_backward(&self, mut pos: usize) -> usize {
        for (i, rotor) in self.rotors.iter().enumerate() {
            pos = rotor.backward(pos);

            if i < self.rotors.len() - 1 {
                pos = (pos as i32 - rotor.position as i32 + self.rotors[i + 1].position as i32) as usize % rotor.wiring.len();
            }
        }

        pos
    }

    fn reflect(&self, pos: usize) -> usize {
        self.reflector.iter().position(|&c| c == ALPHA[pos]).unwrap_or(0)
    }

    fn encode(&mut self, message: &str) -> String {
        let mut encoded_message = Vec::new();

        for ch in message.chars() {
            println!("NEW CHARACTER");

            self.rotate();

            if ch == ' ' {
                encoded_message.push(' ');
                continue;
            }

            let pos = ALPHA.iter().position(|&c| c == ch).unwrap_or(0);

            let mut pos = (pos + self.rotors.last().unwrap().position) % ALPHA.len();

            for _ in 0..self.repeat {
                pos = self.traverse_forward(pos);
            }

            pos = self.reflect(pos);

            for _ in 0..self.repeat {
                pos = self.traverse_backward(pos);
            }

            pos = (pos + ALPHA.len() - self.rotors.last().unwrap().position) % ALPHA.len();

            encoded_message.push(ALPHA[pos]);
        }

        encoded_message.into_iter().collect()
    }
}

// positive modulous function
// fn pos_mod(x: usize, y: usize) -> usize {
//     (x % y + y) % y
// }

fn main() {
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

    let rs = RotorSet {
        rotors: vec![r1, r2, r3],
        reflector: [ 'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T' ],
        repeat: 1,
    };

    let mut rs = rs;

    for (i, rotor) in rs.rotors.iter_mut().enumerate() {
        let mut r_pos = String::new();
        let mut r_inc = String::new();

        println!("Enter rotor {} position and increment (0 - 25): ", i);
        io::stdin().read_line(&mut r_pos).unwrap();
        io::stdin().read_line(&mut r_inc).unwrap();

        let r_pos: usize = r_pos.trim().parse().unwrap();
        let r_inc: usize = r_inc.trim().parse().unwrap();

        rotor.position = r_pos;
        rotor.increment = r_inc;
    }

    println!("Enter repeat: ");
    let mut repeat = String::new();
    io::stdin().read_line(&mut repeat).unwrap();

    rs.repeat = repeat.trim().parse().unwrap();

    println!("Enter message: ");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    let message = message.to_uppercase().trim().to_string();

    if message.chars().all(|ch| ALPHA.contains(&ch)) {
        let encoded_message = rs.encode(&message);
        println!("Encoded message: {}", encoded_message);
    } else {
        println!("Error: message contains invalid characters");
    }
}