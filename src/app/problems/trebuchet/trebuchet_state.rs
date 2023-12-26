// Just some handwritten state machine
enum TrebuchetState {
    Empty,
    E,
    Ei,
    Eig,
    Eigh,
    F,
    Fi,
    Fiv,
    Fo,
    Fou,
    N,
    Ni,
    Nin,
    O,
    On,
    S,
    Se,
    Sev,
    Seve,
    Si,
    T,
    Th,
    Thr,
    Thre,
    Tw,
}

enum TrebuchetMatchResult {
    None(TrebuchetState),
    Digit(TrebuchetState, usize)
}

impl TrebuchetState {
    fn match_char(self, c: char) -> TrebuchetMatchResult {
        match c {
            '1' => TrebuchetMatchResult::Digit(Self::Empty, 1),
            '2' => TrebuchetMatchResult::Digit(Self::Empty, 2),
            '3' => TrebuchetMatchResult::Digit(Self::Empty, 3),
            '4' => TrebuchetMatchResult::Digit(Self::Empty, 4),
            '5' => TrebuchetMatchResult::Digit(Self::Empty, 5),
            '6' => TrebuchetMatchResult::Digit(Self::Empty, 6),
            '7' => TrebuchetMatchResult::Digit(Self::Empty, 7),
            '8' => TrebuchetMatchResult::Digit(Self::Empty, 8),
            '9' => TrebuchetMatchResult::Digit(Self::Empty, 9),
            'e' => match self {
                Self::Fiv => TrebuchetMatchResult::Digit(Self::E, 5),
                Self::Nin => TrebuchetMatchResult::Digit(Self::E, 9),
                Self::On => TrebuchetMatchResult::Digit(Self::E, 1),
                Self::S => TrebuchetMatchResult::None(Self::Se),
                Self::Sev => TrebuchetMatchResult::None(Self::Seve),
                Self::Thr => TrebuchetMatchResult::None(Self::Thre),
                Self::Thre => TrebuchetMatchResult::Digit(Self::E, 3),
                _ => TrebuchetMatchResult::None(Self::E),
            },
            'f' => TrebuchetMatchResult::None(Self::F),
            'g' => match self {
                Self::Ei => TrebuchetMatchResult::None(Self::Eig),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'h' => match self {
                Self::Eig => TrebuchetMatchResult::None(Self::Eigh),
                Self::T => TrebuchetMatchResult::None(Self::Th),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'i' => match self {
                Self::E => TrebuchetMatchResult::None(Self::Ei),
                Self::F => TrebuchetMatchResult::None(Self::Fi),
                Self::N => TrebuchetMatchResult::None(Self::Ni),
                Self::S => TrebuchetMatchResult::None(Self::Si),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'n' => match self {
                Self::Ni => TrebuchetMatchResult::None(Self::Nin),
                Self::O => TrebuchetMatchResult::None(Self::On),
                Self::Seve => TrebuchetMatchResult::Digit(Self::N, 7),
                _ => TrebuchetMatchResult::None(Self::N),
            },
            'o' => match self {
                Self::F => TrebuchetMatchResult::None(Self::Fo),
                Self::Tw => TrebuchetMatchResult::Digit(Self::O, 2),
                _ => TrebuchetMatchResult::None(Self::O),
            },
            'r' => match self {
                Self::Fou => TrebuchetMatchResult::Digit(Self::Empty, 4),
                Self::Th => TrebuchetMatchResult::None(Self::Thr),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            's' => TrebuchetMatchResult::None(Self::S),
            't' => match self {
                Self::Eigh => TrebuchetMatchResult::Digit(Self::T, 8),
                _ => TrebuchetMatchResult::None(Self::T),
            },
            'u' => match self {
                Self::Fo => TrebuchetMatchResult::None(Self::Fou),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'v' => match self {
                Self::Fi => TrebuchetMatchResult::None(Self::Fiv),
                Self::Se => TrebuchetMatchResult::None(Self::Sev),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'w' => match self {
                Self::T => TrebuchetMatchResult::None(Self::Tw),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            'x' => match self {
                Self::Si => TrebuchetMatchResult::Digit(Self::Empty, 6),
                _ => TrebuchetMatchResult::None(Self::Empty),
            },
            _ => TrebuchetMatchResult::None(Self::Empty),
        }
    }
}

pub fn read_line_value(s: &str) -> usize {
    let mut first_value: Option<usize> = None;
    let mut last_value: Option<usize> = None;
    let mut state = TrebuchetState::Empty;

    for c in s.chars() {
        match state.match_char(c) {
            TrebuchetMatchResult::None(s) => {
                state = s;
            },
            TrebuchetMatchResult::Digit(s, value) => {
                if first_value == None {
                    first_value = Some(value);
                }
                last_value = Some(value);
                state = s;
            }
        }
    }

    first_value.unwrap() * 10 + last_value.unwrap()
}
