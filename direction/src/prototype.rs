use super::*;
type ZarityDirection = TernaryDirection;
impl Axis {
    #[allow(non_upper_case_globals)]
    pub const Zname: Self = Axis::Side;
}
//sep
pub enum Axis {
    Side,
    Height,
    Depth,
}

pub enum BinaryDirection {
    Regular,
    Inverted,
}
//sep
use super::Axis;

pub enum TernaryDirection {
    Regular,
    Middle,
    Inverted,
}
//sep
pub struct Zname(ZarityDirection);

impl Zname {
    #[allow(non_upper_case_globals)]
    pub const Zregular: Self = Self(ZarityDirection::Regular);
    //if_ternary//toggle_included
    //toggle_included
    #[allow(non_upper_case_globals)]
    pub const Zmiddle: Self = Self(ZarityDirection::Middle);
    //if_ternary//toggle_included
    //toggle_included
    #[allow(non_upper_case_globals)]
    pub const Zinverted: Self = Self(ZarityDirection::Inverted);
    pub const AXIS: Axis = Axis::Zname;
}
