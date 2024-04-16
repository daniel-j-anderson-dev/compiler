use error_set::error_set;

error_set!(
    ParseError = {
        ParseInteger(std::num::ParseIntError),
    } || ParseOperatorError;
    ParseOperatorError = {
        InvalidCharacter
    };
);
