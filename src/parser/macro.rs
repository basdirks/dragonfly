/// Parse a literal from the given input.
#[macro_export]
macro_rules! literal {
    ($literal:expr) => {
        |input| literal(input, $literal)
    };
}

/// Replace the result of a successful parser with a constant.
#[macro_export]
macro_rules! tag {
    ($parser:expr, $f:expr) => {
        |input| tag(input, $parser, $f)
    };
}

/// Apply a function to the result of a successful parser.
#[macro_export]
macro_rules! map {
    ($parser:expr, $f:expr) => {
        |input| map(input, $parser, $f)
    };
}
