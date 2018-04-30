pub trait Ops {
    type T;

    fn nop(self) -> Self::T;
}
