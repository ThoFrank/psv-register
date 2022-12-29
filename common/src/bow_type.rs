#[derive(Debug, Clone, Copy)]
pub enum BowType {
    Recurve,
    Compound,
    Barebow,
}

impl BowType {
    pub fn is_recurve(&self) -> bool {
        matches!(self, Self::Recurve)
    }
    pub fn is_compound(&self) -> bool {
        matches!(self, Self::Compound)
    }
    pub fn is_barebow(&self) -> bool {
        matches!(self, Self::Barebow)
    }
}

impl Default for BowType {
    fn default() -> Self {
        Self::Recurve
    }
}
