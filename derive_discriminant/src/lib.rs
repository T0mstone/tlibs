pub use d_impl::ToDiscriminant;

pub trait ToDiscriminant {
	type Discriminant;

	fn to_discriminant(&self) -> Self::Discriminant;
}
