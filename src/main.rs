use boolean_expression::{BDDFunc, BDD_ONE, BDD_ZERO};

type BDD = boolean_expression::BDD<u32>; // BDD with terminals labeled by bit indices

#[derive(Copy, Clone)]
struct Encoding {
    bits: u32,
    mask: u32,
}

impl Encoding {
    /// Create a BDD function that is true when the bit-terminals
    /// (variables) for the given bits have the appropriate values. `mask`
    /// has a 1 where we care about the bit in `bits`, and a 0 otherwise
    /// (for "fields" in the encoding).
    fn to_bdd(&self, bdd: &mut BDD) -> BDDFunc {
        let mut func = BDD_ONE;
        for i in 0..32 {
            let bit = 1u32 << i;
            if self.mask & bit != 0 {
                let bit_term = bdd.terminal(i);
                let bit_is_correct = if self.bits & bit != 0 {
                    bit_term
                } else {
                    bdd.not(bit_term)
                };
                func = bdd.and(func, bit_is_correct);
            }
        }
        func
    }
}

/// Check whether any overlaps exist between a set of encodings. If so, find an assignment of bits
/// that hits an overlap.
fn find_overlapping_encoding(encodings: &[Encoding]) -> Option<u32> {
    let mut bdd = BDD::new();
    let mut any_encoding_func = BDD_ZERO;

    for encoding in encodings {
        // Translate the encoding to a BDD function that is true exactly when the value
        // (represented as terminal bit variables) hits the encoding.
        let encoding_func = encoding.to_bdd(&mut bdd);

        // Now, see if this new encoding overlaps with any existing encoding. We check the AND
        // (intersection) of all already-processed encodings and this one; if not constant zero,
        // there is some assignment of bits that satisfies existing encodings and this new one.
        let intersection = bdd.and(encoding_func, any_encoding_func);
        if let Some(satisfying_assignment) = bdd.sat_one(intersection) {
            let mut result = 0u32;
            for (bit, value) in satisfying_assignment {
                if value {
                    result |= 1u32 << bit;
                }
            }
            return Some(result);
        }

        // Finally, update the "union of all encodings so far" function.
        any_encoding_func = bdd.or(any_encoding_func, encoding_func);
    }

    None
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_some_overlaps() {
        assert_eq!(
            find_overlapping_encoding(&[
                Encoding {
                    bits: 0b0101_0000,
                    mask: 0b1111_0000,
                },
                Encoding {
                    bits: 0b0100_0000,
                    mask: 0b1100_0000,
                }
            ])
            .unwrap()
                & 0b1100_0000,
            0b0100_0000
        );
        assert_eq!(
            find_overlapping_encoding(&[
                Encoding {
                    bits: 0b0101_0000,
                    mask: 0b1111_0000,
                },
                Encoding {
                    bits: 0b1100_0000,
                    mask: 0b1100_0000,
                }
            ]),
            None,
        );
    }
}
