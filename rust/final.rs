//https://kerkour.com/rust-functional-programming

use std::io;

#[derive(Clone)]
struct Rotor {
    wiring: [char; 26],
    notch: char,
    position: usize,
    increment: usize,
}

#[derive(Clone)]
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

    fn increment_rotor(&self) -> Rotor {
        let new_position = (self.position + self.increment) % self.wiring.len();
        Rotor { position: new_position, ..*self }
    }

    fn reached_notch(&self) -> bool {
        self.wiring[self.position] == self.notch
    }
}

impl RotorSet {
    fn rotate(&self) -> RotorSet {
        let rs_len = self.rotors.len() - 1;

        let double_rotated_rotors: Vec<Rotor> = self.rotors.iter().rev().map(|rotor| {
            if rotor.reached_notch() {
                rotor.increment_rotor()
            } else {
               rotor.clone()
            }            
        }).collect();

        let rotated_rotors: Vec<Rotor> = double_rotated_rotors.iter().rev().enumerate().map(|(i, rotor)| {
            if i == rs_len || rotor.reached_notch() {
                rotor.increment_rotor()
            } else {
               rotor.clone()
            }            
        }).collect();

        RotorSet { rotors: rotated_rotors, ..*self }
    }

    fn traverse_forward(&self, pos: usize) -> usize {
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
        self.rotors.iter().enumerate().fold(pos, |acc, (i, rotor)| {
            let b_pos = rotor.backward(acc);

            if i < self.rotors.len() - 1 {
                (b_pos as i32 - rotor.position as i32 + self.rotors[i + 1].position as i32) as usize % rotor.wiring.len()
            } else {
                b_pos
            }
        })
    }

    fn reflect(&self, pos: usize) -> usize {
        self.reflector.iter().position(|&c| c == ALPHA[pos]).unwrap_or(0)
    }

    

    fn encode(&self, message: &str) -> String {        
        let (encoded_message, _) = message.chars().fold((Vec::new(), self.clone()), |(mut acc, rs), ch| {
            let rs = rs.rotate();
            println!("{:?}", rs.rotors.iter().map(|rotor| rotor.position).collect::<Vec<usize>>());
            
            if ch == ' ' {
                acc.push(' ');
            } else {
                let pos = ALPHA.iter().position(|&c| c == ch).unwrap_or(0);
                let w_pos = (pos + rs.rotors.last().unwrap().position) % ALPHA.len();
                
                let f_pos = (0..rs.repeat).fold(w_pos, |acc, _| rs.traverse_forward(acc));
                
                let r_pos = rs.reflect(f_pos);
                
                let b_pos = (0..rs.repeat).fold(r_pos, |acc, _| rs.traverse_backward(acc));
                
                let a_pos = (b_pos + ALPHA.len() - rs.rotors.last().unwrap().position) % ALPHA.len();
                
                acc.push(ALPHA[a_pos]);
            }

            (acc, rs)
        });
    
        encoded_message.into_iter().collect()
    }
}

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