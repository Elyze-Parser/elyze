use elyze::bytes::matchers::match_pattern;
use elyze::matcher::Match;
use rand::prelude::Distribution;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;

#[derive(Eq, PartialEq)]
enum BaseAdn {
    A,
    C,
    G,
    T,
}

fn match_patterns(patterns: &[&[u8]], data: &[u8]) -> (bool, usize) {
    for pattern in patterns {
        let (is_matching, size) = match_pattern(pattern, data);
        if is_matching {
            return (true, size);
        }
    }
    (false, 0)
}

impl Match<u8> for BaseAdn {
    fn is_matching(&self, data: &[u8]) -> (bool, usize) {
        match self {
            BaseAdn::A => match_patterns(&[b"A", b"a"], data),
            BaseAdn::C => match_patterns(&[b"C", b"c"], data),
            BaseAdn::G => match_patterns(&[b"G", b"g"], data),
            BaseAdn::T => match_patterns(&[b"T", b"t"], data),
        }
    }

    fn size(&self) -> usize {
        match self {
            BaseAdn::A => 1,
            BaseAdn::C => 1,
            BaseAdn::G => 1,
            BaseAdn::T => 1,
        }
    }
}

#[derive(Eq, PartialEq)]
enum BaseArn {
    A,
    C,
    G,
    U,
}

// There are 20 amino acids used by proteins
const AMINO_ACID_SIZE: usize = 20;

#[derive(Debug)]
enum AminoAcid {
    Alanine,
    Cysteine,
    Glutamine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
    Asparagine,
    AsparticAcid,
    GlutamicAcid,
    Glycine,
    Proline,
    Arginine,
}

impl Distribution<AminoAcid> for AminoAcid {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AminoAcid {
        let index: u8 = rng.random_range(0..AMINO_ACID_SIZE) as u8;
        unsafe { std::mem::transmute(index) }
    }
}

const CODON_SIZE: usize = 3;

fn match_codon(codon: &[BaseArn; 3], data: &[BaseArn]) -> (bool, usize) {
    if data.len() < CODON_SIZE {
        return (false, 0);
    }

    if &data[..CODON_SIZE] == codon {
        return (true, CODON_SIZE);
    }
    (false, 0)
}

fn match_codons(codons: &[[BaseArn; 3]], data: &[BaseArn]) -> (bool, usize) {
    for codon in codons {
        let (is_matching, size) = match_codon(codon, data);
        if is_matching {
            return (true, size);
        }
    }
    (false, 0)
}

impl Match<BaseArn> for AminoAcid {
    fn is_matching(&self, data: &[BaseArn]) -> (bool, usize) {
        match self {
            AminoAcid::Alanine => match_codons(
                &[
                    [BaseArn::G, BaseArn::C, BaseArn::U],
                    [BaseArn::G, BaseArn::C, BaseArn::C],
                    [BaseArn::G, BaseArn::C, BaseArn::A],
                    [BaseArn::G, BaseArn::C, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Cysteine => match_codons(
                &[
                    [BaseArn::U, BaseArn::G, BaseArn::U],
                    [BaseArn::U, BaseArn::G, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::Glutamine => match_codons(
                &[
                    [BaseArn::C, BaseArn::A, BaseArn::A],
                    [BaseArn::C, BaseArn::A, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Histidine => match_codons(
                &[
                    [BaseArn::C, BaseArn::A, BaseArn::U],
                    [BaseArn::C, BaseArn::A, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::Isoleucine => match_codons(
                &[
                    [BaseArn::A, BaseArn::U, BaseArn::U],
                    [BaseArn::A, BaseArn::U, BaseArn::C],
                    [BaseArn::A, BaseArn::U, BaseArn::A],
                ],
                data,
            ),
            AminoAcid::Leucine => match_codons(
                &[
                    [BaseArn::U, BaseArn::U, BaseArn::A],
                    [BaseArn::U, BaseArn::U, BaseArn::G],
                    [BaseArn::C, BaseArn::U, BaseArn::U],
                    [BaseArn::C, BaseArn::U, BaseArn::C],
                    [BaseArn::C, BaseArn::U, BaseArn::A],
                    [BaseArn::C, BaseArn::U, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Lysine => match_codons(
                &[
                    [BaseArn::A, BaseArn::A, BaseArn::A],
                    [BaseArn::A, BaseArn::A, BaseArn::G],
                ],
                data,
            ),
            // codon start
            AminoAcid::Methionine => match_codons(&[[BaseArn::A, BaseArn::U, BaseArn::G]], data),
            AminoAcid::Phenylalanine => match_codons(
                &[
                    [BaseArn::U, BaseArn::U, BaseArn::U],
                    [BaseArn::U, BaseArn::U, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::Serine => match_codons(
                &[
                    [BaseArn::U, BaseArn::C, BaseArn::U],
                    [BaseArn::U, BaseArn::C, BaseArn::C],
                    [BaseArn::U, BaseArn::C, BaseArn::A],
                    [BaseArn::U, BaseArn::C, BaseArn::G],
                    [BaseArn::A, BaseArn::G, BaseArn::U],
                    [BaseArn::A, BaseArn::G, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::Threonine => match_codons(
                &[
                    [BaseArn::A, BaseArn::C, BaseArn::U],
                    [BaseArn::A, BaseArn::C, BaseArn::C],
                    [BaseArn::A, BaseArn::C, BaseArn::A],
                    [BaseArn::A, BaseArn::C, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Tryptophan => match_codons(&[[BaseArn::U, BaseArn::G, BaseArn::G]], data),
            AminoAcid::Tyrosine => match_codons(
                &[
                    [BaseArn::U, BaseArn::A, BaseArn::U],
                    [BaseArn::U, BaseArn::A, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::Valine => match_codons(
                &[
                    [BaseArn::G, BaseArn::U, BaseArn::U],
                    [BaseArn::G, BaseArn::U, BaseArn::C],
                    [BaseArn::G, BaseArn::U, BaseArn::A],
                    [BaseArn::G, BaseArn::U, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Asparagine => match_codons(
                &[
                    [BaseArn::A, BaseArn::A, BaseArn::U],
                    [BaseArn::A, BaseArn::A, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::AsparticAcid => match_codons(
                &[
                    [BaseArn::G, BaseArn::A, BaseArn::U],
                    [BaseArn::G, BaseArn::A, BaseArn::C],
                ],
                data,
            ),
            AminoAcid::GlutamicAcid => match_codons(
                &[
                    [BaseArn::G, BaseArn::A, BaseArn::A],
                    [BaseArn::G, BaseArn::A, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Glycine => match_codons(
                &[
                    [BaseArn::G, BaseArn::G, BaseArn::U],
                    [BaseArn::G, BaseArn::G, BaseArn::C],
                    [BaseArn::G, BaseArn::G, BaseArn::A],
                    [BaseArn::G, BaseArn::G, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Proline => match_codons(
                &[
                    [BaseArn::C, BaseArn::C, BaseArn::U],
                    [BaseArn::C, BaseArn::C, BaseArn::C],
                    [BaseArn::C, BaseArn::C, BaseArn::A],
                    [BaseArn::C, BaseArn::C, BaseArn::G],
                ],
                data,
            ),
            AminoAcid::Arginine => match_codons(
                &[
                    [BaseArn::C, BaseArn::G, BaseArn::U],
                    [BaseArn::C, BaseArn::G, BaseArn::C],
                    [BaseArn::C, BaseArn::G, BaseArn::A],
                    [BaseArn::C, BaseArn::G, BaseArn::G],
                    [BaseArn::A, BaseArn::G, BaseArn::A],
                    [BaseArn::A, BaseArn::G, BaseArn::G],
                ],
                data,
            ),
        }
    }

    fn size(&self) -> usize {
        3
    }
}

fn main() {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let protein = AminoAcid::Alanine
        .sample_iter(&mut rng)
        .take(16)
        .collect::<Vec<AminoAcid>>();
    let animo_acid = dbg!(protein);
}
