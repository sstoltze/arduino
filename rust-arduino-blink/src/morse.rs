#![no_std]
#![no_main]

pub enum Morse {
    Short,
    Long,
    Null,
    LetterBreak,
    WordBreak,
}

impl Morse {
    pub fn timing(&self) -> u16 {
        morse_timing(self)
    }

    pub fn pause(&self) -> bool {
        use Morse::*;
        match self {
            Null | LetterBreak | WordBreak => true,
            _ => false,
        }
    }
}

pub type MorseSequence = [Morse; 6];

pub fn char_to_morse(c: char) -> MorseSequence {
    use Morse::*;
    match c {
        'a' => [Short, Long, Null, Null, Null, LetterBreak],
        'b' => [Long, Short, Short, Short, Null, LetterBreak],
        'c' => [Long, Short, Long, Short, Null, LetterBreak],
        'd' => [Long, Short, Short, Null, Null, LetterBreak],
        'e' => [Short, Null, Null, Null, Null, LetterBreak],
        'f' => [Short, Short, Long, Short, Null, LetterBreak],
        'g' => [Long, Long, Short, Null, Null, LetterBreak],
        'h' => [Short, Short, Short, Short, Null, LetterBreak],
        'i' => [Short, Short, Null, Null, Null, LetterBreak],
        'j' => [Short, Long, Long, Long, Null, LetterBreak],
        'k' => [Long, Short, Long, Null, Null, LetterBreak],
        'l' => [Short, Long, Short, Short, Null, LetterBreak],
        'm' => [Long, Long, Null, Null, Null, LetterBreak],
        'n' => [Long, Short, Null, Null, Null, LetterBreak],
        'o' => [Long, Long, Long, Null, Null, LetterBreak],
        'p' => [Short, Long, Long, Short, Null, LetterBreak],
        'q' => [Long, Long, Short, Long, Null, LetterBreak],
        'r' => [Short, Long, Short, Null, Null, LetterBreak],
        's' => [Short, Short, Short, Null, Null, LetterBreak],
        't' => [Long, Null, Null, Null, Null, LetterBreak],
        'u' => [Short, Short, Long, Null, Null, LetterBreak],
        'v' => [Short, Short, Short, Long, Null, LetterBreak],
        'w' => [Short, Long, Long, Null, Null, LetterBreak],
        'x' => [Long, Short, Short, Long, Null, LetterBreak],
        'y' => [Long, Short, Long, Long, Null, LetterBreak],
        'z' => [Long, Long, Short, Short, Null, LetterBreak],
        '1' => [Short, Long, Long, Long, Long, LetterBreak],
        '2' => [Short, Short, Long, Long, Long, LetterBreak],
        '3' => [Short, Short, Short, Long, Long, LetterBreak],
        '4' => [Short, Short, Short, Short, Long, LetterBreak],
        '5' => [Short, Short, Short, Short, Short, LetterBreak],
        '6' => [Long, Short, Short, Short, Short, LetterBreak],
        '7' => [Long, Long, Short, Short, Short, LetterBreak],
        '8' => [Long, Long, Long, Short, Short, LetterBreak],
        '9' => [Long, Long, Long, Long, Short, LetterBreak],
        '0' => [Long, Long, Long, Long, Long, LetterBreak],
        ' ' => [WordBreak, Null, Null, Null, Null, LetterBreak],
        _ => [Null, Null, Null, Null, Null, Null],
    }
}

fn morse_timing(m: &Morse) -> u16 {
    use Morse::*;
    match m {
        Null => 0,
        Short => 1,
        Long => 3,
        LetterBreak => 3,
        WordBreak => 7,
    }
}
