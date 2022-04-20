pub enum Distance {
    Inch(f64),
    Feet(f64),
}

impl Distance {
    pub fn to_feet(&self) -> f64 {
        match self {
            Distance::Inch(v) => *v / 12 as f64,
            Distance::Feet(v) => *v,
        }
    }
}
