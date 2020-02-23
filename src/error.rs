use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown key name")]
    UnKnownKeyName,
    #[fail(display = "parse int error: {:?}", err)]
    IntParseError { err: std::num::ParseIntError },
    #[fail(display = "nom parse error: {}", string)]
    NomParseError { string: String },
    #[fail(display = "not complete parse: {}", rest)]
    NotCompleteParse { rest: String },
}
