#![deny(
	future_incompatible,
	nonstandard_style,
	rust_2018_compatibility,
	rust_2018_idioms,
	unused,
	warnings
)]
// rustc's additional allowed by default lints
#![deny(
	absolute_paths_not_starting_with_crate,
	deprecated_in_future,
	elided_lifetimes_in_paths,
	explicit_outlives_requirements,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	noop_method_call,
	or_patterns_back_compat,
	pointer_structural_match,
	semicolon_in_expressions_from_macros,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_code,
	unsafe_op_in_unsafe_fn,
	unstable_features,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
	unused_lifetimes,
	unused_qualifications,
	unused_results,
	variant_size_differences
)]
// enable all of Clippy's lints
#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
	clippy::blanket_clippy_restriction_lints,
	clippy::implicit_return,
	clippy::missing_docs_in_private_items,
	clippy::redundant_pub_crate,
	clippy::tabs_in_doc_comments
)]
#![deny(
	rustdoc::bare_urls,
	rustdoc::broken_intra_doc_links,
	rustdoc::invalid_codeblock_attributes,
	rustdoc::invalid_html_tags,
	rustdoc::missing_crate_level_docs,
	rustdoc::private_doc_tests,
	rustdoc::private_intra_doc_links
)]

//! # Captur
//!
//! Starting in Rust 2021, Rust will no longer capture whole structs and instead will only capture a
//! disjoint set of the fields used in a closure. In some cases, it is necessary to capture the
//! structs to retain a particular drop order. This macro will capture the struct within the
//! closure, ensuring the correct drop order.
//!
//! # Example
//! ```
//! use captur::capture;
//! struct SomeStruct {
//! 	a: String,
//! 	b: String,
//! }
//!
//! impl SomeStruct {
//! 	fn new() -> Self {
//! 		Self {
//! 			a: String::from("a"),
//! 			b: String::from("b"),
//! 		}
//! 	}
//! }
//!
//! let some_struct = SomeStruct::new();
//! let result = || {
//! 	captur::capture!(some_struct);
//! 	format!("{}", some_struct.b)
//! };
//!
//! println!("{}", result());
//! ```

/// Create a reference to a struct, that will ensure it is captured by a closure.
#[macro_export]
macro_rules! capture {
	($( $v:expr ),*) => {
		$(let _ = &$v;)*
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestStruct {
		a: String,
		b: String,
	}

	impl TestStruct {
		fn new() -> Self {
			Self {
				a: String::from("a"),
				b: String::from("b"),
			}
		}
	}

	#[test]
	fn single() {
		let a = TestStruct::new();
		let result = || {
			capture!(a);
			format!("Value: {}", a.a)
		};

		assert_eq!(result(), "Value: a");
	}

	#[test]
	fn multiple() {
		let a = TestStruct::new();
		let b = TestStruct::new();
		let result = || {
			capture!(a, b);
			format!("Value: {} {}", a.a, b.b)
		};

		assert_eq!(result(), "Value: a b");
	}
}
