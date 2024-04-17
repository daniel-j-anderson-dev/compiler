use error_set::error_set;

error_set!(
    ParseError = {
        ParseInteger(std::num::ParseIntError),
    } || ParseOperatorError || ParseAbstractSyntaxTreeError;
    ParseOperatorError = {
        InvalidCharacter,
        EmptyString,
        TooLong,
    };
    ParseAbstractSyntaxTreeError = {
        MissingCloseParenthesis,
        MissingOpenParenthesis,
        
    };
);
