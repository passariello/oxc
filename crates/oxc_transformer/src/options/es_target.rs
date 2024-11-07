use std::str::FromStr;

use cow_utils::CowUtils;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ESTarget {
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
    ES2022,
    ES2023,
    ES2024,
    #[default]
    ESNext,
}

impl FromStr for ESTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.cow_to_lowercase().as_ref() {
            "es5" => Ok(Self::ES5),
            "es2015" => Ok(Self::ES2015),
            "es2016" => Ok(Self::ES2016),
            "es2017" => Ok(Self::ES2017),
            "es2018" => Ok(Self::ES2018),
            "es2019" => Ok(Self::ES2019),
            "es2020" => Ok(Self::ES2020),
            "es2021" => Ok(Self::ES2021),
            "es2022" => Ok(Self::ES2022),
            "es2023" => Ok(Self::ES2023),
            "es2024" => Ok(Self::ES2024),
            "esnext" => Ok(Self::ESNext),
            _ => Err(format!("Invalid target \"{s}\".")),
        }
    }
}