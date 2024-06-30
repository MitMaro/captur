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
