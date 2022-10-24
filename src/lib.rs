//! Implementation of ISO-11649 standard
//!
//! The Creditor Reference (also called the Structured Creditor Reference)
//! is an international business standard based on ISO 11649, implemented at
//! the end of 2008.
//!
//! The Creditor Reference was first implemented within the SEPA rulebook 3.2.
//!
//! A vendor adds the Creditor Reference to its invoices. When a customer pays
//! the invoice, the company writes the Creditor Reference instead of the
//! invoice number in the message section, or places a Creditor Reference
//! field in its payment ledger.
//!
//! When the vendor receives the payment, it can automatically match the
//! remittance information to its Accounts Receivable system.
//!
//! # Links
//!
//! [ISO-11649-2009 Financial services - Core banking - Structured creditor reference to remittance information](https://cdn.standards.iteh.ai/samples/50649/a769e57fc5a34724bac3a5d18a2b8407/ISO-11649-2009.pdf)
//!
#![warn(clippy::pedantic)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![forbid(unsafe_code)]

use std::{borrow::Cow, str::FromStr};

pub use parse_error::ParseError;

pub mod parse_error;

/// The official identifier for `RfCreditorReference`
pub const IDENTIFIER: &str = "RF";

/// Prefix with identifier to use for checksum calculation
pub const GEN_PREFIX: &str = "RF00";

const DIGIT_CONVERT_LOWCASE: i8 = -('a' as i8) + 10;
const DIGIT_CONVERT_NUMBER: i8 = -('0' as i8);
const DIGIT_CONVERT_UPCASE: i8 = -('A' as i8) + 10;

/// `RfCreditorReference` provides generator and validator for
/// creditor references.
///
/// # Examples
///
/// Generate a conformant creditor reference:
///
/// ```rust
/// use iso_11649::RfCreditorReference;
///
/// let reference = "539007547034";
/// let rf = RfCreditorReference::new(reference);
///
/// assert_eq!(rf.to_electronic_string(), "RF18539007547034");
/// assert_eq!(rf.to_string(), "RF18 5390 0754 7034");
///
/// let rf = RfCreditorReference::try_new(reference)
///     .expect("need to be valid reference");
///
/// assert_eq!(rf.to_electronic_string(), "RF18539007547034");
/// assert_eq!(rf.to_string(), "RF18 5390 0754 7034");
/// ```
///
/// Parsing existing creditor references:
///
/// ```rust
/// use std::str::FromStr;
///
/// use iso_11649::RfCreditorReference;
///
/// let rf = RfCreditorReference::parse_str("RF18539007547034").unwrap();
/// let rf = RfCreditorReference::from_str("RF18539007547034").unwrap();
///
/// assert_eq!(rf.to_string(), "RF18 5390 0754 7034");
///
/// let r: &str = (&rf).into();
/// assert_eq!(r, "RF18 5390 0754 7034");
/// ```
///
/// Check the checksum validation of a creditor reference:
///
/// ```rust
/// use std::str::FromStr;
///
/// use iso_11649::parse_error::ParseError;
/// use iso_11649::RfCreditorReference;
///
/// fn is_valid_reference(reference: &str) {
///     #[allow(clippy::match_same_arms)]
///     match RfCreditorReference::from_str(reference) {
///         Ok(validated_creditor_reference) => {
///             // this RE is validated
///         },
///         // multiple kinds of errors, with cause of
///         // why validation failed...
///         Err(err) => match err {
///             ParseError::InvalidCharacter(_) => {}
///             ParseError::InvalidChecksum(_) => {}
///             ParseError::InvalidFormat(_) => {}
///             ParseError::InvalidIdentifier(_) => {}
///         },
///     }
/// }
///
/// is_valid_reference("RF18 5390 0754 7034");
///
/// ```
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RfCreditorReference<'a> {
    /// The checksum digits of reference
    checksum: u8,
    /// For print formatted creditor reference string
    creditor_reference: Cow<'a, str>,
}

impl RfCreditorReference<'_> {
    /// Generate new [`RfCreditorReference`] from specified reference
    ///
    /// See also [`Self::try_new`] and [`Self::parse_str`].
    ///
    /// # Examples
    ///
    /// See [`RfCreditorReference`].
    ///
    /// # Panics
    ///
    /// If `reference` contains invalid characters.
    /// Valid characters are 0-9, a-z and A-Z.
    ///
    #[must_use]
    pub fn new(reference: &str) -> Self {
        Self::try_new(reference).unwrap()
    }

    /// Parses a `creditor_reference`
    ///
    /// See also [`Self::from_str`].
    ///
    /// # Errors
    ///
    /// Results in [`ParseError`]s when there is some problem with
    /// parsing the `reference`.
    ///
    pub fn parse_str(reference: &str) -> Result<Self, ParseError> {
        Self::check_reference(reference)?;

        let reference = RfCreditorReference::convert_electronic(reference);
        let checksum = str::parse::<u8>(&reference[2..4]);

        if let Err(e) = checksum {
            Err(ParseError::InvalidChecksum(e.to_string()))
        } else {
            let checksum = checksum.unwrap_or_default();

            let check_digits = Self::gen_check_digits(&reference)?;

            if Self::is_valid(&check_digits) {
                let four_elemented_ref = reference[4..]
                    .chars()
                    .enumerate()
                    .flat_map(|(i, c)| {
                        if i != 0 && i % 4 == 0 {
                            vec![' ', c]
                        } else {
                            vec![c]
                        }
                    })
                    .collect::<String>();
                let creditor_reference = Cow::from(format!(
                    "{}{:02} {}",
                    IDENTIFIER, checksum, four_elemented_ref
                ));
                Ok(Self {
                    checksum,
                    creditor_reference,
                })
            } else {
                Err(ParseError::InvalidChecksum(reference))
            }
        }
    }

    /// Retrieves `creditor_reference` in electronic format without spaces
    #[must_use]
    #[inline]
    pub fn to_electronic_string(&self) -> String {
        Self::convert_electronic(&self.creditor_reference)
    }

    /// Try to generate new [`RfCreditorReference`] from specified reference
    ///
    /// See also [`Self::new`] and [`Self::parse_str`].
    ///
    /// # Examples
    ///
    /// See [`RfCreditorReference`].
    ///
    /// # Errors
    ///
    /// If `reference` contains invalid characters.
    /// Valid characters are 0-9, a-z and A-Z.
    ///
    pub fn try_new(reference: &str) -> Result<Self, ParseError> {
        let mut electronic_reference =
            if reference.len() > GEN_PREFIX.len() && reference.starts_with(GEN_PREFIX) {
                Self::convert_electronic(reference)
            } else if reference.len() > IDENTIFIER.len() && reference.starts_with(IDENTIFIER) {
                GEN_PREFIX.to_string() + &Self::convert_electronic(reference)[2..]
            } else {
                GEN_PREFIX.to_string() + &Self::convert_electronic(reference)
            };

        Self::check_reference(&electronic_reference)?;

        let checksum = Self::gen_checksum(&Self::gen_check_digits(&electronic_reference)?);

        electronic_reference.replace_range(2..4, &String::from_iter(checksum.1));

        Self::parse_str(&electronic_reference)
    }

    /// First basic validation of reference
    fn check_reference(reference: &str) -> Result<(), ParseError> {
        let reference = RfCreditorReference::convert_electronic(reference);
        if !(reference.len() > 4 && reference.len() <= 25) {
            Err(ParseError::InvalidFormat(reference))
        } else if &reference[..2] != IDENTIFIER {
            Err(ParseError::InvalidIdentifier(reference))
        } else if reference[4..]
            .find(|c| {
                !(('0'..='9').contains(&c) || ('A'..='Z').contains(&c) || ('a'..='z').contains(&c))
            })
            .is_some()
        {
            Err(ParseError::InvalidCharacter(reference))
        } else {
            Ok(())
        }
    }

    #[inline]
    fn convert_electronic(reference: &str) -> String {
        reference.replace(' ', "")
    }

    /// Try to generate a `Vec` of `electronic_reference` with digits
    ///
    /// See also [`Self::to_electronic_string`]
    /// and [`convert_electronic`].
    #[inline]
    fn gen_check_digits(electronic_reference: &str) -> Result<Vec<i8>, ParseError> {
        let map = electronic_reference[4..]
            .chars()
            .chain(electronic_reference[0..4].chars())
            .map(|c| match c {
                '0'..='9' => {
                    let n = (c as i8) + DIGIT_CONVERT_NUMBER;
                    Some(vec![n])
                }
                'A'..='Z' => {
                    let n = (c as i8) + DIGIT_CONVERT_UPCASE;
                    let t = n / 10;
                    Some(vec![t, n - t * 10])
                }
                'a'..='z' => {
                    let n = (c as i8) + DIGIT_CONVERT_LOWCASE;
                    let t = n / 10;
                    Some(vec![t, n - t * 10])
                }
                _ => None,
            });

        if map.clone().any(|o| o.is_none()) {
            return Err(ParseError::InvalidCharacter(
                electronic_reference.to_string(),
            ));
        }

        // unwrap() ok, because return ParseError above
        let digits = map.flat_map(Option::unwrap).collect();

        Ok(digits)
    }

    /// Generates the checksum
    ///
    /// Returns a tuple with checksum as `u8` and the two checksum digits as `[char; 2]`.<br>
    /// Later can be used with e.g. `String::from_iter()`.
    #[inline]
    fn gen_checksum(check_digits: &[i8]) -> (u8, [char; 2]) {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let checksum = u8::try_from(
            98u128
                .checked_sub(
                    check_digits
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, &n)| (n as u128) * 10u128.pow(i as u32))
                        .sum::<u128>()
                        % 97,
                )
                .unwrap(),
        )
        .unwrap();

        let checksum_chars = &mut ['0'; 2];
        checksum_chars[0] = (checksum / 10 + 48) as char;
        checksum_chars[1] = ((checksum - checksum / 10 * 10) + 48) as char;

        (checksum, *checksum_chars)
    }

    /// Returns true if `check_digits` contains valid data and checksum.
    #[inline]
    fn is_valid(check_digits: &[i8]) -> bool {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let check = u8::try_from(
            check_digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &n)| (n as u128) * 10u128.pow(i as u32))
                .sum::<u128>()
                % 97,
        )
        .unwrap();

        check == 1
    }
}

impl From<&RfCreditorReference<'_>> for String {
    fn from(id: &RfCreditorReference) -> Self {
        id.creditor_reference.to_string()
    }
}

impl From<RfCreditorReference<'_>> for String {
    fn from(id: RfCreditorReference) -> Self {
        id.creditor_reference.to_string()
    }
}

impl<'a> From<&'a RfCreditorReference<'a>> for &'a str {
    fn from(id: &'a RfCreditorReference) -> Self {
        id.creditor_reference.as_ref()
    }
}

impl<'a> From<&'a RfCreditorReference<'a>> for &Cow<'a, str> {
    fn from(id: &'a RfCreditorReference<'a>) -> Self {
        &id.creditor_reference
    }
}

impl<'a> From<RfCreditorReference<'a>> for Cow<'a, str> {
    fn from(id: RfCreditorReference<'a>) -> Self {
        id.creditor_reference
    }
}

impl<'a> std::fmt::Display for RfCreditorReference<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.creditor_reference)
    }
}

impl FromStr for RfCreditorReference<'_> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GEN_REFS: &[&str] = &[
        "RF00539007547034",
        "RF002348231",
        "RF00 5390 0754 7034",
        " RF00 5390 0754 7034 ",
        "RF00 ABCD 0754 efgh",
        "RF00539007547034928301234",
    ];

    const VALID_REFS: &[&str] = &[
        "RF18539007547034",
        "RF712348231",
        "RF18 5390 0754 7034",
        " RF18 5390 0754 7034 ",
        "RF63 ABCD 0754 efgh",
        "RF93539007547034928301234",
    ];

    const INVALID_REFS: &[&str] = &[
        "18539007547034",
        "18 5390 0754 7034",
        " 18 5390 0754 7034 ",
        "RF18539007547034928TOOLONG",
        "RF18539007547034_123",
        "RF18539007547034älsö",
        "RF18539007547034@",
    ];

    #[test]
    fn check_mod_97_test() {
        #[allow(clippy::unreadable_literal)]
        let remainder = 2348231271500u64 % 97;
        assert_eq!(
            remainder, 27,
            "2348231271500 mod 97 should be 27 but is {}",
            remainder
        );
    }

    #[test]
    fn gen_check_digits_test() {
        assert_eq!(
            RfCreditorReference::gen_check_digits(VALID_REFS[0]).unwrap(),
            vec![5, 3, 9, 0, 0, 7, 5, 4, 7, 0, 3, 4, 2, 7, 1, 5, 1, 8]
        );
        assert_eq!(
            RfCreditorReference::gen_check_digits(VALID_REFS[1]).unwrap(),
            vec![2, 3, 4, 8, 2, 3, 1, 2, 7, 1, 5, 7, 1]
        );

        let r = "RF18AB";
        assert_eq!(
            RfCreditorReference::gen_check_digits(r).unwrap(),
            vec![1, 0, 1, 1, 2, 7, 1, 5, 1, 8]
        );
    }

    #[test]
    #[allow(
        clippy::unreadable_literal,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    fn gen_checksum_test() {
        let nr = [2i8, 3, 4, 8, 2, 3, 1, 2, 7, 1, 5, 0, 0]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &n)| (n as usize) * 10usize.pow(i as u32))
            .sum::<usize>();

        assert_eq!(nr, 2348231271500);

        let nr = nr % 97;
        assert_eq!(nr, 27);

        let nr = 98usize.checked_sub(nr).unwrap();
        assert_eq!(nr, 71);

        let nr = u8::try_from(nr).unwrap();
        assert_eq!(nr, 71);

        assert_eq!(
            RfCreditorReference::gen_check_digits(GEN_REFS[0]).unwrap(),
            vec![5, 3, 9, 0, 0, 7, 5, 4, 7, 0, 3, 4, 2, 7, 1, 5, 0, 0]
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(&[2i8, 3, 4, 8, 2, 3, 1, 2, 7, 1, 5, 0, 0]),
            (71, ['7', '1'])
        );

        assert_eq!(
            RfCreditorReference::gen_check_digits(GEN_REFS[1]).unwrap(),
            vec![2, 3, 4, 8, 2, 3, 1, 2, 7, 1, 5, 0, 0]
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(&[
                5, 3, 9, 0, 0, 7, 5, 4, 7, 0, 3, 4, 2, 7, 1, 5, 0, 0
            ]),
            (18, ['1', '8'])
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(
                &RfCreditorReference::gen_check_digits(GEN_REFS[0]).unwrap()
            )
            .0,
            18
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(
                &RfCreditorReference::gen_check_digits(GEN_REFS[1]).unwrap()
            )
            .0,
            71
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(
                &RfCreditorReference::gen_check_digits(&RfCreditorReference::convert_electronic(
                    GEN_REFS[4]
                ))
                .unwrap()
            )
            .0,
            63
        );

        assert_eq!(
            RfCreditorReference::gen_checksum(
                &RfCreditorReference::gen_check_digits(GEN_REFS[5]).unwrap()
            )
            .0,
            93
        );
    }

    #[test]
    fn parse_str_test() {
        for vr in VALID_REFS {
            let res = RfCreditorReference::parse_str(vr);
            assert!(res.is_ok(), "not valid: {} [{}]", vr, res.unwrap());
        }
        for ir in INVALID_REFS {
            assert!(
                RfCreditorReference::parse_str(ir).is_err(),
                "should not be valid: {}",
                ir
            );
        }
    }

    #[test]
    fn from_str_test() {
        for vr in VALID_REFS {
            let res = RfCreditorReference::from_str(vr);
            assert!(res.is_ok(), "not valid: {} [{}]", vr, res.unwrap());
        }
        for ir in INVALID_REFS {
            #[allow(clippy::match_same_arms)]
            match RfCreditorReference::from_str(ir) {
                Ok(_) => panic!("should not be valid: {}", ir),
                Err(err) => match err {
                    ParseError::InvalidCharacter(_) => {}
                    ParseError::InvalidChecksum(_) => {}
                    ParseError::InvalidFormat(_) => {}
                    ParseError::InvalidIdentifier(_) => {}
                },
            }
        }
    }
}
