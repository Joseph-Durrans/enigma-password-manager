package main

import (
	"fmt"
	"strings"
)

type rotor struct {
	wiring    []byte
	notch     byte
	position  int
	increment int
}

type rotorSet struct {
	rotors    []*rotor
	reflector []byte
	repeat    int
}

var alpha = []byte{'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'}

func (r *rotor) forward(pos int) int {
	// Get the position of the character in the alphabet:
	for i := 0; i < len(alpha); i++ {
		fmt.Println(byte('A'))
		if alpha[i] == r.wiring[pos] {
			return i
		}
	}

	// If the character is not found, return 0:
	return 0
}

func (r *rotor) backward(pos int) int {
	// Get the position of the character in the wiring:
	for i := 0; i < len(r.wiring); i++ {
		if r.wiring[i] == alpha[pos] {
			return i
		}
	}

	// If the character is not found, return 0:
	return 0
}

func (r *rotor) incrementRotor() {
	// Increment the position of the rotor:
	r.position = (r.position + r.increment) % len(r.wiring)
}

func (r *rotor) reachedNotch() bool {
	// Check if the rotor has reached its notch:
	return r.wiring[r.position] == r.notch
}

func (rs *rotorSet) rotate() {
	// Get length of rotors:
	rsLen := len(rs.rotors) - 1

	// Rotate the rightmost rotor:
	rs.rotors[rsLen].incrementRotor()

	// Check if double rotation is needed on any rotor:
	for i := rsLen - 1; i > 0; i-- {
		if rs.rotors[i].reachedNotch() {
			rs.rotors[i].incrementRotor()
		}
	}

	// Rotate the next rotors if this rotor has reached its notch:
	for i := rsLen; i > 0; i-- {
		if rs.rotors[i].reachedNotch() {
			rs.rotors[i-1].incrementRotor()
		}
	}
}

func (rs *rotorSet) traverseForward(pos int) int {
	for i := len(rs.rotors) - 1; i >= 0; i-- {
		pos = rs.rotors[i].forward(pos)

		// Adjust for next rotor position:
		if i != 0 {
			pos = posMod(pos-rs.rotors[i].position+rs.rotors[i-1].position, len(rs.rotors[i].wiring))
		}
	}
	return pos
}

func (rs *rotorSet) traverseBackward(pos int) int {
	for i := 0; i < len(rs.rotors); i++ {
		pos = rs.rotors[i].backward(pos)
		
		// Adjust for next rotor position:
		if i != len(rs.rotors)-1 {
			pos = posMod(pos-rs.rotors[i].position+rs.rotors[i+1].position, len(rs.rotors[i].wiring))
		}
	}

	return pos
}

func (rs *rotorSet) reflect(pos int) int {
	// Get the position of the character in the alphabet:
	for i := 0; i < len(alpha); i++ {
		if alpha[i] == rs.reflector[pos] {
			return i
		}
	}

	// If the character is not found, return 0:
	return 0
}

func (rs *rotorSet) encode(message string) (string, error) {
	var encodedMessage []byte

	for i := 0; i < len(message); i++ {
		// Rotate the rotors:
		rs.rotate()

		// If the character is a space, add a space to the encoded message and continue:
		if message[i] == ' ' {
			encodedMessage = append(encodedMessage, ' ')
			continue
		}

		// Get the position of the character in the alphabet adjusted for the next rotor position:
		var pos int
		for j := 0; j < len(alpha); j++ {
			if alpha[j] == message[i] {
				pos = posMod(j+rs.rotors[len(rs.rotors)-1].position, len(alpha))
			}
		}

		// Go forward through rotors adjusting for position:
		for j := 0; j < rs.repeat; j++ {
			pos = rs.traverseForward(pos)
		
		}

		// Reflect the signal:
		pos = rs.reflect(pos)

		// Go backward through rotors:
		for j := 0; j < rs.repeat; j++ {
			pos = rs.traverseBackward(pos)
		}

		// Get the position of the character in the alphabet:
		pos = posMod(pos-rs.rotors[len(rs.rotors)-1].position, len(alpha))

		// Get the character from the alphabet:
		encodedMessage = append(encodedMessage, alpha[pos])
	}

	return string(encodedMessage), nil
}

func posMod(a, b int) int {
	// Modulo operator that always returns a positive number:
	return ((a % b) + b) % b
}

func main() {
	// Create rotors:
	r1 := &rotor{
		wiring:    []byte{'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'},
		notch:     'Q',
	}

	r2 := &rotor{
		wiring:    []byte{'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'},
		notch:     'E',
	}

	r3 := &rotor{
		wiring:    []byte{'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O'},
		notch:     'V',
	}

	// Create rotor set and reflector:
	rs := &rotorSet{
		rotors:    []*rotor{r1, r2, r3},
		reflector: []byte{'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T'},
	}

	// For each rotor in rotor set, set position and increment with user input:
	for i := 0; i < len(rs.rotors); i++ {
		var rPos, rInc int
		fmt.Printf("Enter rotor %d position and increment (0 - 25): ", i)
		if _, err := fmt.Scanln(&rPos, &rInc); err != nil {
			fmt.Println("Error:", err, "Setting rotor position and increment to 0 and 1")
			rPos, rInc = 0, 1

		}

		if rPos > 25 || rPos < 0 {
			fmt.Println("Error: rotor position must be between 0 and 25")
			return
		}

		if rInc > 25 || rInc < 0 {
			fmt.Println("Error: rotor increment must be between 0 and 25")
			return
		}

		rs.rotors[i].position = rPos
		rs.rotors[i].increment = rInc
	}

	// Set repeat with user input:
	fmt.Print("Enter repeat: ")
	if _, err := fmt.Scanln(&rs.repeat); err != nil {
		fmt.Println("Error:", err, "Setting repeat to 1")
		rs.repeat = 1
	}

	// Get message from user:
	var message string
	fmt.Print("Enter message: ")
	fmt.Scanln(&message)

	// Convert message to uppercase:
	message = strings.ToUpper(message)

	// check if message contains any characters not in alpha
	for i := 0; i < len(message); i++ {
		if !strings.Contains(string(alpha), string(message[i])) {
			fmt.Println("Error: message contains invalid characters")
			return
		}
	}

	// Print the encoded message:
	fmt.Print("Encoded message: ")
	fmt.Println(rs.encode(message))
}
