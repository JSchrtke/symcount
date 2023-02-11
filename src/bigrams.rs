use std::cmp::Ordering;

#[derive(Default, Debug, PartialEq)]
struct Bigram {
    first: String,
    second: String,
}

#[derive(Debug, PartialEq)]
enum BigramError {
    InvalidLength(String),
}

impl TryFrom<&str> for Bigram {
    type Error = BigramError;

    fn try_from(value: &str) -> Result<Self, BigramError> {
        match value.len().cmp(&2) {
            Ordering::Less => Err(BigramError::InvalidLength(format!(
                "Can not parse given string '{}' with length {} into a bigram as it is too short.",
                value,
                value.len()
            ))),
            Ordering::Greater => Err(BigramError::InvalidLength(format!(
                "Can not parse given string '{}' with length {} into a bigram as it is too long.",
                value,
                value.len()
            ))),
            Ordering::Equal => Ok(Self::default()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bigrams::Bigram;
    use crate::bigrams::BigramError;

    #[test]
    fn from_too_short_string_fails() {
        let expected_result = Err(BigramError::InvalidLength(
            "Can not parse given string '' with length 0 into a bigram as it is too short.".into(),
        ));

        let maybe_bigram = Bigram::try_from("");

        assert_eq!(expected_result, maybe_bigram);

        let too_short = ";";
        let expected_result = Err(BigramError::InvalidLength(format!(
            "Can not parse given string '{too_short}' with length 1 into a bigram as it is too short."
        )));

        let maybe_bigram = Bigram::try_from(too_short);

        assert_eq!(expected_result, maybe_bigram);
    }

    #[test]
    fn from_too_long_string_fails() {
        let too_long = ";/*";
        let expected_result = Err(BigramError::InvalidLength(format!(
            "Can not parse given string '{too_long}' with length 3 into a bigram as it is too long."
        )));

        let maybe_bigram = Bigram::try_from(too_long);

        assert_eq!(expected_result, maybe_bigram);
    }
}
