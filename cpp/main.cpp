#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
#include <array>

class Rotor {
public:
    Rotor(std::array<char, 26> wiring, char notch, int position = 0, int increment = 1) : wiring(wiring), notch(notch), position(position), increment(increment) {}

    int forward(int pos);
    int backward(int pos);
    void incrementRotor();
    bool reachedNotch();

    int getPosition() const {
        return this->position;
    }

    void setPosition(int position) {
        this->position = position;
    }

    int getIncrement() const {
        return this->increment;
    }

    void setIncrement(int increment) {
        this->increment = increment;
    }

    std::array<char, 26> getWiring() const {
		return this->wiring;
	}
private:
    const std::array<char, 26> wiring;
    int position;
    int increment;
    int notch;
};

class RotorSet {
public:
    RotorSet(std::vector<Rotor*> rotors, std::array<char, 26> reflector, int repeat = 1) : rotors(rotors), reflector(reflector), repeat(repeat) {}

    void rotate();
    int traverseForward(int pos);
    int traverseBackward(int pos);
    int reflect(int pos);
    std::string encode(std::string msg);

    std::vector<Rotor*> getRotors() const {
		return this->rotors;
	}

    std::array<char, 26> getReflector() const {
		return this->reflector;
	}

    int getRepeat() const {
        return this->repeat;
    }

    void setRepeat(int repeat) {
        this->repeat = repeat;
    }
private:
    std::vector<Rotor*> rotors;
    const std::array<char, 26> reflector;
    int repeat;

    static int posMod(int a, int b) {
        return (a % b + b) % b;
    }
};

const std::array<char, 26> alpha = { 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z' };

int Rotor::forward(int pos)
{
    // Get the position of the character in the alphabet:
    for (int i = 0; i < alpha.size(); i++) {
        if (alpha[i] == this->wiring[pos]) {
            return i;
        }
    }

    // If the character is not found, return 0:
    return 0;
}

int Rotor::backward(int pos)
{
    // Get the position of the character in the wiring:
    for (int i = 0; i < this->wiring.size(); i++) {
        if (this->wiring[i] == alpha[pos]) {
            return i;
        }
    }

    // If the character is not found, return 0:
    return 0;
}

void Rotor::incrementRotor() {
    // Increment the position of the rotor:
    this->position = (this->position + this->increment) % this->wiring.size();
}

bool Rotor::reachedNotch() {
    // Check if the rotor has reached its notch:
    return this->wiring[this->position] == this->notch;
}

void RotorSet::rotate() {
    // Get length of rotors:
    int rsLen = this->rotors.size() - 1;

    // Rotate the rightmost rotor:
    this->rotors[rsLen]->incrementRotor();

    // Check if double rotation is needed on any rotor:
    for (int i = rsLen - 1; i > 0; i--) {
        if (this->rotors[i]->reachedNotch()) {
            this->rotors[i]->incrementRotor();
        }
    }

    // Rotate the next rotors if this rotor has reached its notch:
    for (int i = rsLen; i > 0; i--) {
        if (this->rotors[i]->reachedNotch()) {
            this->rotors[i - 1]->incrementRotor();
        }
    }
}

int RotorSet::traverseForward(int pos) {
    for (int i = this->rotors.size() - 1; i >= 0; i--) {
        pos = this->rotors[i]->forward(pos);

        // Adjust for next rotor position:
        if (i != 0) {
            pos = posMod(pos - this->rotors[i]->getPosition() + this->rotors[i - 1]->getPosition(), this->rotors[i]->getWiring().size());
        }
    }

    return pos;
}

int RotorSet::traverseBackward(int pos) {
    for (int i = 0; i < this->rotors.size(); i++) {
        pos = this->rotors[i]->backward(pos);

        // Adjust for next rotor position:
        if (i != this->rotors.size() - 1) {
            pos = posMod(pos - this->rotors[i]->getPosition() + this->rotors[i + 1]->getPosition(), this->rotors[i]->getWiring().size());
        }
    }

    return pos;

}

int RotorSet::reflect(int pos) {
    // Get the position of the character in the alphabet:
    for (int i = 0; i < alpha.size(); i++) {
        if (alpha[i] == this->reflector[pos]) {
            return i;
        }
    }

    // If the character is not found, return 0:
    return 0;
}

std::string RotorSet::encode(std::string msg) {
    std::string encodedMsg;

    for (int i = 0; i < msg.size(); i++) {


        // Rotate the rotors:
        this->rotate();

        // If the character is a space, add a space to the encoded message and continue:
        if (msg[i] == ' ') {
            encodedMsg += ' ';
            continue;
        }

        // Get the position of the character in the alphabet adjusted for the next rotor position:
        int pos;
        for (int j = 0; j < alpha.size(); j++) {
            if (alpha[j] == msg[i]) {
                pos = posMod(j + this->rotors[this->rotors.size() - 1]->getPosition(), alpha.size());
            }
        }

        // Go forward through rotors adjusting for position:
        for (int j = 0; j < this->repeat; j++) {
            pos = this->traverseForward(pos);
        }

        // Reflect the signal:
        pos = this->reflect(pos);

        // Go backward through rotors:
        for (int j = 0; j < this->repeat; j++) {
            pos = this->traverseBackward(pos);
        }

        // Get the position of the character in the alphabet:
        pos = posMod(pos - this->rotors[this->rotors.size() - 1]->getPosition(), alpha.size());

        // Get the character from the alphabet:
        encodedMsg += alpha[pos];


    }

    return encodedMsg;
}



int main() {
    // Create rotors:
    RotorSet rs(
        {
            new Rotor({ 'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J' }, 'Q', 0, 1),
            new Rotor({ 'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E' }, 'E', 0, 1),
            new Rotor({ 'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O' }, 'V', 0, 1)
        },
        { 'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T' },
        1
    );

    // For each rotor in rotor set, set position and increment with user input:
    for (Rotor* r : rs.getRotors()) {
        int rPos, rInc;

        try {
            std::cout << "Enter rotor position and increment (0 - 25): ";
            std::cin >> rPos >> rInc;
        }
        catch (std::exception e) {
            std::cout << "Error: " << e.what() << " rotor position and increment must be between 0 and 25";
            return 0;
        }

        if (rPos > 25 || rPos < 0) {
            std::cout << "Error: rotor position must be between 0 and 25";
            return 0;
        }

        if (rInc > 25 || rInc < 0) {
            std::cout << "Error: rotor increment must be between 0 and 25";
            return 0;
        }

        r->setPosition(rPos);
        r->setIncrement(rInc);
    }

    // Set repeat with user input:
    try {
                int repeat;

        std::cout << "Enter repeat (1 - 25): ";
        std::cin >> repeat;
        rs.setRepeat(repeat);
    }
    catch (std::exception e) {
        std::cout << "Error: repeat must be an integer";
        return 0;
    }

    // Get message from user:
    std::string msg;
    std::cout << "Enter message: ";
    std::cin.ignore();
    std::getline(std::cin, msg);

    // Convert message to uppercase:
    std::transform(msg.begin(), msg.end(), msg.begin(), ::toupper);

    // check if message contains any characters not in alpha
    for (int i = 0; i < msg.size(); i++) {
        auto a = std::find(std::begin(alpha), std::end(alpha), msg[i]);
        if (a == std::end(alpha)) {
            std::cout << "Error: message contains invalid characters";
            return 0;
        }
    }

    // Print the encoded message:
    std::cout << "Encoded message: " << rs.encode(msg) << std::endl;

    return 0;
}


