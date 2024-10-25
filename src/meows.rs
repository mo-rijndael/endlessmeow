use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::grammaw::MeowToken;

pub fn generate_meow() -> String {
    let mut buf = String::with_capacity(20);
    let pattern = MEOWS.choose(&mut thread_rng()).unwrap();
    for token in *pattern {
        token.resolve(&mut buf)
    }
    if thread_rng().gen_bool(0.1) {
        buf.push('\n');
    } else {
        buf.push(' ');
    }
    buf
}

// Edit as needed
// 'a' — always resolves to 'a'
// ('a', 10) — resolves to 'a' repeated from 1 to 10 times
// ('a',) — resolves to 'a' with probability 0.5
// ['a', 'b', 'c'] — choose one of
// Sowwy for weadabiwity
static MEOWS: &[&[MeowToken]] = &[
    meow!('m', ('e', 2), (('o', 2),), ('w', 6), (['!', '~'],)),
    meow!('m', ('r', 10), ['a', 'o', 'e'], ('w', 2)),
    meow!('m', ('r', 10)),
    meow!('p', ('u',), ('r', 10), ('~',)),
    meow!('m', 'o', ('w', 5)),
    meow!(('>',), ':', '3', ('c',)),
    meow!('>', ('/', 5), '<'),
];
