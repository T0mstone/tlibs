pub enum Axis {
    Side,
    Height,
    Depth,
}

pub enum BinaryDirection {
    Regular,
    Inverted,
}

pub struct Side(BinaryDirection);

impl Side {
    #[allow(non_upper_case_globals)]
    pub const Right: Self = Self(BinaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Left: Self = Self(BinaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Side;
}
pub struct Height(BinaryDirection);

impl Height {
    #[allow(non_upper_case_globals)]
    pub const Top: Self = Self(BinaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Bottom: Self = Self(BinaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Height;
}
pub struct Depth(BinaryDirection);

impl Depth {
    #[allow(non_upper_case_globals)]
    pub const Front: Self = Self(BinaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Back: Self = Self(BinaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Depth;
}

pub mod ternary {
use super::Axis;

pub enum TernaryDirection {
    Regular,
    Middle,
    Inverted,
}

pub struct Side(TernaryDirection);

impl Side {
    #[allow(non_upper_case_globals)]
    pub const Right: Self = Self(TernaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Center: Self = Self(TernaryDirection::Middle);
    
    #[allow(non_upper_case_globals)]
    pub const Left: Self = Self(TernaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Side;
}

pub struct Height(TernaryDirection);

impl Height {
    #[allow(non_upper_case_globals)]
    pub const Top: Self = Self(TernaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Center: Self = Self(TernaryDirection::Middle);
    
    #[allow(non_upper_case_globals)]
    pub const Bottom: Self = Self(TernaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Height;
}

pub struct Depth(TernaryDirection);

impl Depth {
    #[allow(non_upper_case_globals)]
    pub const Front: Self = Self(TernaryDirection::Regular);
    
    #[allow(non_upper_case_globals)]
    pub const Center: Self = Self(TernaryDirection::Middle);
    
    #[allow(non_upper_case_globals)]
    pub const Back: Self = Self(TernaryDirection::Inverted);
    pub const AXIS: Axis = Axis::Depth;
}

}