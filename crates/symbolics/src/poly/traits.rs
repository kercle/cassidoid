pub trait RingElement {}

pub trait Ring {
    fn zero() -> dyn RingElement
    where
        Self: Sized;

    fn one() -> dyn RingElement
    where
        Self: Sized;
}
