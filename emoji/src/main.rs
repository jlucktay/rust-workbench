fn main() -> std::result::Result<(), std::str::Utf8Error> {
	// Functions that can fail typically return a result, like here where we're creating a UTF-8 string from bytes.
	// Not all bytes represent a valid string.

	// In this first example, we can see that `s1` is the okay variant of the `Result` enum.
	let s1 = std::str::from_utf8(&[240, 159, 141, 137]);
	println!("{:?}", s1);

	// `s2` is the error variant of the `Result` enum.
	// You can use `expect()` for a custom error message.
	// It's called expect because it telegraphs to people reading both the code and the error what you were expecting
	// when you unwrapped the result.
	#[allow(clippy::expect_used)] // reason = "seeing functionality of `expect()`"
	let s2 = std::str::from_utf8(&[195, 40]).expect("expected valid utf-8");
	println!("{:?}", s2);

	// This pattern of errors as values keeps us in the functional world where other languages would have exceptions
	// which break us out.

	let melon = &[240, 159, 141, 137];

	// You can also match and handle the error, or in this case, panic anyway.
	match std::str::from_utf8(melon) {
		Ok(s) => println!("{}", s),
		Err(e) => panic!("{}", e),
	}

	// Or you can use `if let` to safely destructure the inner value if it is okay.
	if let Ok(s) = std::str::from_utf8(melon) {
		println!("{}", s);
	}

	// Or you can bubble up the error, returning it to the calling function, which then handles it.
	match std::str::from_utf8(melon) {
		Ok(s) => println!("{}", s),
		Err(e) => return Err(e),
	}
	// Ok(()) // needs to be commented out as the function continues below

	// This pattern of unwrapping the value inside a result if it's okay or returning if it's an error is so common
	// that Rust has dedicated syntax to do it.

	// The question mark operator at the end of this line does the exact same thing as the larger match statement
	// immediately above.
	// This is the normal Rust error pattern in application code where you're trying to just write the happy path,
	// though the previous options are available to you when you need them.
	let s = std::str::from_utf8(melon)?;
	println!("{}", s);
	Ok(())
}
