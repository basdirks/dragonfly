#[macro_export]
macro_rules! literal {
    ($literal:expr) => {
        |input| literal(input, $literal)
    };
}

#[macro_export]
macro_rules! tag {
    ($parser:expr, $f:expr) => {
        |input| tag(input, $parser, $f)
    };
}

#[macro_export]
macro_rules! map {
    ($parser:expr, $f:expr) => {
        |input| map(input, $parser, $f)
    };
}
