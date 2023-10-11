package main

import "fmt"

type rotor struct {
	wiring      []byte
	notch       byte
	position    int
}

type rotorSet struct {
	rotors []*rotor
	reflector []byte
}

var alpha = []byte{'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'}

func (r *rotor) forward(pos int) (int, error) {
	// Get the position of the character in the alphabet:
	for i := 0; i < len(alpha); i++ {
		if alpha[i] == r.wiring[pos] {
			return i, nil
		}
	}

	// If the character is not found, return an error:
	return 0, fmt.Errorf("character not found")
}

func (r *rotor) backward(pos int) (int, error) {
	// Get the position of the character in the wiring:
	for i := 0; i < len(r.wiring); i++ {
		if r.wiring[i] == alpha[pos] {
			return i, nil
		}
	}

	// If the character is not found, return an error:
	return 0, fmt.Errorf("character not found")
}

func (rs *rotorSet) rotate() {
	// Get length of rotors:
	rsLen := len(rs.rotors) - 1

	// Rotate the rightmost rotor:
	rs.rotors[rsLen].position = (rs.rotors[rsLen].position + 1) % len(rs.rotors[rsLen].wiring)

	// Check if double rotation is needed on any rotor:
	for i := rsLen - 1; i > 0; i-- {
		if rs.rotors[i].wiring[rs.rotors[i].position] == rs.rotors[i].notch {
			rs.rotors[i].position = (rs.rotors[i].position + 1) % len(rs.rotors[i].wiring)
		}
	}
	
	// Rotate the next rotors if this rotor has reached its notch:
	for i := rsLen; i > 0; i-- {
		if rs.rotors[i].wiring[rs.rotors[i].position] == rs.rotors[i].notch {
			rs.rotors[i - 1].position = (rs.rotors[i - 1].position + 1) % len(rs.rotors[i - 1].wiring)
		} 
	}
}

func (rs *rotorSet) reflect(pos int) (int, error) {
	// Get the position of the character in the alphabet:
	for i := 0; i < len(alpha); i++ {
		if alpha[i] == rs.reflector[pos] {
			return i, nil
		}
	}
	
	// If the character is not found, return an error:
	return 0, fmt.Errorf("character not found")
}

func posMod(a, b int) int {
	// Modulo operator that always returns a positive number:
	return ((a % b) + b) % b
}

func main() {
	// Create rotors and reflector:
	r1 := &rotor{
		wiring: []byte{'E','K','M','F','L','G','D','Q','V','Z','N','T','O','W','Y','H','X','U','S','P','A','I','B','R','C','J'},
		notch: 'Q',
		position: 25,
	}
	r2 := &rotor{
		wiring: []byte{'A','J','D','K','S','I','R','U','X','B','L','H','W','T','M','C','Q','G','Z','N','P','Y','F','V','O','E'},
		notch: 'E',
		position: 25,
	}
	r3 := &rotor{
		wiring: []byte{'B','D','F','H','J','L','C','P','R','T','X','V','Z','N','Y','E','I','W','G','A','K','M','U','S','Q','O'},
		notch: 'V',
		position: 24,
	}
	
	// Create rotor set:
	rs := &rotorSet{
		rotors: []*rotor{r1, r2, r3},
		reflector: []byte{'Y','R','U','H','Q','S','L','D','P','X','N','G','O','K','M','I','E','B','F','Z','C','W','V','J','A','T'},
	}


	// Encode the message:
	message := "HELLOMYNAMEISJOE"
	var encodedMessage []byte

	for i := 0; i < len(message); i++ {
		// Rotate the rotors:
		rs.rotate()

		// Get the position of the character in the alphabet adjusted for the next rotor position:
		var pos int
		for j := 0; j < len(alpha); j++ {
			if alpha[j] == message[i] {
				pos = posMod(j + rs.rotors[len(rs.rotors) - 1].position, len(alpha))
			}
		}

		// Go forward through rotors adjusting for position:
		for i := len(rs.rotors) - 1; i >= 0; i-- {
			pos, _ = rs.rotors[i].forward(pos)


			// Adjust for next rotor position:
			if i != 0 {
				pos = posMod(pos - rs.rotors[i].position + rs.rotors[i - 1].position, len(rs.rotors[i].wiring))
			}
		}

		// Reflect the signal:
		pos, _ = rs.reflect(pos)

		// Go backward through rotors:
		for i := 0; i < len(rs.rotors); i++ {
			pos, _ = rs.rotors[i].backward(pos)

			// Adjust for next rotor position:
			if i != len(rs.rotors) - 1 {
				pos = posMod(pos - rs.rotors[i].position + rs.rotors[i + 1].position, len(rs.rotors[i].wiring))
			}
		}

		// Get the position of the character in the alphabet:
		pos = posMod(pos - rs.rotors[len(rs.rotors) - 1].position, len(alpha))

		// Get the character from the alphabet:
		encodedMessage = append(encodedMessage, alpha[pos])
	}

	fmt.Println(string(encodedMessage))
}