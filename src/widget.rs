pub trait Widget {
    type Builder: Default;

    fn build(builder: Self::Builder) -> Self;
}

