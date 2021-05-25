use proc_macro2::{Ident, Span, TokenStream};
use syn::parenthesized;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Paren;
use syn::{Attribute, Data, DeriveInput, Field, Fields, ItemEnum};

/// parses `(...)`
struct ParenthesizedTokens {
	pub paren: Paren,
	pub tokens: TokenStream,
}

impl Parse for ParenthesizedTokens {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let content;
		let paren = parenthesized!(content in input);
		Ok(Self {
			paren,
			tokens: content.parse()?,
		})
	}
}

fn derive_discriminant_impl(input_item: TokenStream) -> syn::Result<TokenStream> {
	let input = syn::parse2::<DeriveInput>(input_item).unwrap();

	let mut data = match input.data {
		Data::Enum(data) => data,
		_ => {
			return Err(syn::Error::new(
				Span::call_site(),
				"Tried to derive a discriminant for non-enum",
			))
		}
	};

	let mut is_sub_discriminant = vec![];
	let mut attr_errs = vec![];

	for var in &mut data.variants {
		if var
			.attrs
			.iter()
			.any(|a| a.path.is_ident("sub_discriminant"))
		{
			match var.fields.len() {
				1 => {
					let Field { ty, .. } = var.fields.iter_mut().next().unwrap();
					*ty = syn::parse_quote! {
						<#ty as ToDiscriminant>::Discriminant
					};
					is_sub_discriminant.push(true);
				}
				n => unimplemented!(
					"#[sub_discriminant] on variants with {} fields is not supported (for now)",
					n
				),
			}
		} else {
			var.fields = Fields::Unit;
			is_sub_discriminant.push(false);
		}
		let mut retain = vec![];
		for (i, a) in var.attrs.iter_mut().enumerate() {
			if a.path.is_ident("discriminant_attr") {
				match syn::parse2::<ParenthesizedTokens>(a.tokens.clone()) {
					Ok(ParenthesizedTokens { tokens, .. }) => {
						let attr: Attribute = syn::parse_quote! {
							#[#tokens]
						};
						*a = attr;
						retain.push(i);
					}
					Err(e) => {
						attr_errs.push(syn::Error::new(a.span(), e));
					}
				}
			}
		}
		var.attrs = var
			.attrs
			.iter()
			.enumerate()
			.filter_map(|(i, x)| retain.contains(&i).then(|| x.clone()))
			.collect();
	}

	let attrs = input
		.attrs
		.iter()
		.cloned()
		.filter_map(|a| {
			let a_span = a.span();
			a.path
				.is_ident("discriminant_attr")
				.then(|| match syn::parse2::<ParenthesizedTokens>(a.tokens) {
					Ok(ParenthesizedTokens { tokens, .. }) => {
						let attr: Attribute = syn::parse_quote! {
							#[#tokens]
						};
						Some(attr)
					}
					Err(e) => {
						attr_errs.push(syn::Error::new(a_span, e));
						None
					}
				})
				.and_then(|opt| opt)
		})
		.collect::<Vec<Attribute>>();

	if !attr_errs.is_empty() {
		return Err(attr_errs
			.into_iter()
			.reduce(|mut l, r| {
				l.combine(r);
				l
			})
			.unwrap());
	}

	let discriminant = ItemEnum {
		attrs,
		vis: input.vis,
		enum_token: data.enum_token,
		ident: Ident::new(&format!("{}Discriminant", input.ident), Span::call_site()),
		generics: input.generics,
		brace_token: data.brace_token,
		variants: data.variants,
	};

	let input_type = &input.ident;
	let discriminant_type = &discriminant.ident;
	let variant = &discriminant
		.variants
		.iter()
		.map(|var| &var.ident)
		.collect::<Vec<&Ident>>();

	let (pattern, value) = is_sub_discriminant
		.into_iter()
		.map(|b| {
			(
				if b {
					quote::quote! { (x) }
				} else {
					quote::quote! { { .. } }
				},
				b.then(|| quote::quote! { (x.to_discriminant()) })
					.unwrap_or_default(),
			)
		})
		.unzip::<_, _, Vec<_>, Vec<_>>();

	let res = quote::quote! {
		#discriminant

		impl ToDiscriminant for #input_type {
			type Discriminant = #discriminant_type;

			fn to_discriminant(&self) -> #discriminant_type {
				match self {
					#(
						#input_type::#variant #pattern => #discriminant_type::#variant #value
					),*
				}
			}
		}

		impl From<&#input_type> for #discriminant_type {
			fn from(x: &#input_type) -> #discriminant_type {
				x.to_discriminant()
			}
		}
	};

	Ok(res)
}

/// Derive the `ToDiscriminant` trait and create a `<Type Name>Discriminant` enum
///
/// This derive macro is enum-only.
///
/// The discriminant enum is a copy of the input enum with all fields of every variant removed.\
/// *) The exception to that rule is the `#[child]` attribute
///
/// # Helper attributes
/// - `#[sub_discriminant]`: only usable on variants with a single field; instead of no fields, the discriminant of the single field will be included in the discriminant,
///     acting as a sub-discriminant.
/// - `#[discriminant_attr(…)]`: usable on the enum itself or on any variant; applies `#[…]` in its place on the discriminant.
///
/// # Attributes on the Discriminant
/// All attributes on variants and the type itself are cleared when constructing the discriminant.
/// If the discriminant is supposed to also have an attribute, you must double it with `#[discriminant_attr(…)]`
///
/// # Example
/// ```
/// # use graphite_proc_macros::ToDiscriminant;
/// # use editor_core::misc::derivable_custom_traits::ToDiscriminant;
/// # use std::ffi::OsString;
///
/// #[derive(ToDiscriminant)]
/// #[discriminant_attr(derive(Debug, Eq, PartialEq))]
/// pub enum EnumA {
///     A(u8),
///     #[sub_discriminant]
///     B(EnumB)
/// }
///
/// #[derive(ToDiscriminant)]
/// #[discriminant_attr(derive(Debug, Eq, PartialEq))]
/// #[discriminant_attr(repr(u8))]
/// pub enum EnumB {
///     Foo(u8),
///     Bar(String),
///     #[cfg(feature = "some-feature")]
///     #[discriminant_attr(cfg(feature = "some-feature"))]
///     WindowsBar(OsString)
/// }
///
/// let a = EnumA::A(1);
/// assert_eq!(a.to_discriminant(), EnumADiscriminant::A);
/// let b = EnumA::B(EnumB::Bar("bar".to_string()));
/// assert_eq!(b.to_discriminant(), EnumADiscriminant::B(EnumBDiscriminant::Bar));
/// ```
#[proc_macro_derive(ToDiscriminant, attributes(sub_discriminant, discriminant_attr))]
pub fn derive_discriminant(input_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	proc_macro::TokenStream::from(
		derive_discriminant_impl(input_item.into()).unwrap_or_else(|err| err.to_compile_error()),
	)
}
