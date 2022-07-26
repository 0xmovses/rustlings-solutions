
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

// This is the custom error type that we will be using for the parser for
// `Climate`.
#[derive(Debug, PartialEq)]
enum ParseClimateError {
    Empty,
    BadLen,
    NoCity,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

// This `From` implementation allows the `?` operator to work on
// `ParseIntError` values.
impl From<ParseIntError> for ParseClimateError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

// This `From` implementation allows the `?` operator to work on
// `ParseFloatError` values.
impl From<ParseFloatError> for ParseClimateError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloat(e)
    }
}

// impl Error for ParseClimateError {}

// The `Display` trait allows for other code to obtain the error formatted
// as a user-visible string.
impl Display for ParseClimateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Imports the variants to make the following code more compact.
        use ParseClimateError::*;
        match self {
            Empty => write!(f, "empty input"),
            BadLen => write!(f, "incorrect number of fields"),
            NoCity => write!(f, "no city name"),
            ParseInt(_e) => write!(f, "error parsing year: invalid digit found in string"),
            ParseFloat(e) => write!(f, "error parsing temperature: {}", e),
            _ => write!(f, "unhandled error!"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Climate {
    city: String,
    year: u32,
    temp: f32,
}

// Parser for `Climate`.
// 1. Split the input string into 3 fields: city, year, temp.
// 2. Return an error if the string is empty or has the wrong number of
//    fields.
// 3. Return an error if the city name is empty.
// 4. Parse the year as a `u32` and return an error if that fails.
// 5. Parse the temp as a `f32` and return an error if that fails.
// 6. Return an `Ok` value containing the completed `Climate` value.
impl FromStr for Climate {
    type Err = ParseClimateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseClimateError::Empty);
        }
        let splitted_item: Vec<_> = s.split(',').collect();
        let (city, year, temp) = match &splitted_item[..] {
            [city, year, temp] => (city.to_string(), year, temp),
            _ => return Err(ParseClimateError::BadLen),
        };
        if city == "" {
            return Err(ParseClimateError::NoCity);
        }
        let year: u32 = year.parse()?;
        let temp: f32 = temp.parse()?;
        Ok(Climate { city, year, temp })
    }
}

// Don't change anything below this line (other than to enable ignored
// tests).

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", "Hong Kong,1999,25.7".parse::<Climate>()?);
    println!("{:?}", "".parse::<Climate>()?);
    Ok(())
}
