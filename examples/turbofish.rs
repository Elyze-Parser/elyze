use elyze::matcher::Match;

/// Pattern to match.
const TURBOFISH: [char; 4] = [':', ':', '<', '>'];

/// Handle turbofish operator.
struct Turbofish;

/// Match turbofish operator.
impl Match<char> for Turbofish {
    fn is_matching(&self, data: &[char]) -> (bool, usize) {
        let pattern = &TURBOFISH;
        if data.len() < pattern.len() {
            return (false, 0);
        }
        if &data[..pattern.len()] == pattern {
            return (true, pattern.len());
        }
        (false, 0)
    }

    fn size(&self) -> usize {
        TURBOFISH.len()
    }
}

fn main() {
    let data = [':', ':', '<', '>', 'b'];
    let scanner = elyze::scanner::Scanner::new(&data);
    let result = Turbofish.is_matching(&scanner);
    println!("{:?}", result); // ( true, 4 ) because the turbofish operator is 4 char

    let data = ['a', ':', ':', '<', '>', 'b'];
    let scanner = elyze::scanner::Scanner::new(&data);
    let result = Turbofish.is_matching(&scanner);
    println!("{:?}", result); // ( false, 0 ) because doesn't match the turbofish operator
}
