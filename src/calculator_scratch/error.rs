use error_set::error_set;

error_set!(
    ParseError = {
        ParseInt(std::num::ParseIntError)
    } || ParseTokenKindError || ParseRawTokenError;
    ParseTokenKindError = {
        UnrecognizedToken
    };
    ParseRawTokenError = {
        U
    };
);
