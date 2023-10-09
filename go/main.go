package main

import "fmt"

type rotor struct {
	wiring      []byte
	notch       byte
	position    int
}

type rotorSet struct {
	rotors []*rotor
}

func (rs *rotorSet) rotate() {
	// Get length of rotors:
	rsLen := len(rs.rotors) - 1

	// Rotate the rightmost rotor:
	rs.rotors[rsLen].position = (rs.rotors[rsLen].position + 1) % len(rs.rotors[rsLen].wiring)

	// Check if double rotation is needed:
	for i := rsLen - 1; i > 0; i-- {
		if rs.rotors[i].wiring[rs.rotors[i].position] == rs.rotors[i].notch {
			rs.rotors[i].position = (rs.rotors[i].position + 1) % len(rs.rotors[i].wiring)
		}
	}
	
	// Rotate the other rotors if the previous rotor reached its notch:
	for i := rsLen; i > 0; i-- {
		if rs.rotors[i].wiring[rs.rotors[i].position] == rs.rotors[i].notch {
			rs.rotors[i - 1].position = (rs.rotors[i - 1].position + 1) % len(rs.rotors[i - 1].wiring)
		} 
	}
}

func main() {
	// Example usage:
	r1 := &rotor{
		wiring: []byte{'E','K','M','F','L','G','D','Q','V','Z','N','T','O','W','Y','H','X','U','S','P','A','I','B','R','C','J'},
		notch: 'Q',
		position: 0,
	}
	r2 := &rotor{
		wiring: []byte{'A','J','D','K','S','I','R','U','X','B','L','H','W','T','M','C','Q','G','Z','N','P','Y','F','V','O','E'},
		notch: 'E',
		position: 0,
	}
	r3 := &rotor{
		wiring: []byte{'B','D','F','H','J','L','C','P','R','T','X','V','Z','N','Y','E','I','W','G','A','K','M','U','S','Q','O'},
		notch: 'V',
		position: 0,
	}
	
	rs := &rotorSet{
		rotors: []*rotor{r1, r2, r3},
	}

	for i := 0; i < 20000; i++ {
		fmt.Println(r1.position, r2.position, r3.position)

		rs.rotate()
	}
}