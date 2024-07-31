use clap::ValueEnum;

#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum License {
    /// License type
    MIT,
    Apache2_0,
    GnuGplV3,
    Bsd3Clause,
    CC0_1_0,
}

impl ToString for License {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}