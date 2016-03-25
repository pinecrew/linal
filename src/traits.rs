pub trait Cross<RHS = Self> {
    type Output;
    fn cross(self, rhs: RHS) -> Self::Output;
}
