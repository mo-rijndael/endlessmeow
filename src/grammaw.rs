use rand::{seq::SliceRandom, thread_rng, Rng};

pub enum MeowToken {
    Letter(char),
    Repeatable(char, u32),
    Alternative(&'static [char]),
    Optional(&'static Self),
}
impl MeowToken {
    fn maybe_capitalized(c: char) -> char {
        if thread_rng().gen_bool(0.1) {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
    pub fn resolve(&self, buffer: &mut String) {
        match *self {
            MeowToken::Letter(c) => buffer.push(Self::maybe_capitalized(c)),
            MeowToken::Repeatable(c, n) => {
                let n = thread_rng().gen_range(1..=n);
                for _ in 0..n {
                    buffer.push(Self::maybe_capitalized(c));
                }
            }
            MeowToken::Alternative(list) => {
                let c = list.choose(&mut thread_rng()).unwrap();
                buffer.push(Self::maybe_capitalized(*c));
            }
            MeowToken::Optional(c) => {
                if thread_rng().gen_bool(0.5) {
                    c.resolve(buffer);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! meow {
    ($($token:tt),* $(,)?) => {
        &[
            $(mew!($token)),*
        ]
    };
}

#[macro_export]
macro_rules! mew {
    ( ($char:literal, $limit:literal ) ) => {
        MeowToken::Repeatable($char, $limit)
    };
    ( ($el:tt,) ) => {
        MeowToken::Optional(&mew!($el))
    };
    ( [$($el:literal),+] ) => {
        MeowToken::Alternative(&[$($el),+])
    };
    ( $char:literal ) => {
        MeowToken::Letter($char)
    };
}
