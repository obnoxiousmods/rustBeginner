// Arrays - Fixed list where elemtns are the same data type

use std::mem;

pub fn run() {
	let mut numbers: [i32; 5] = [1,2,3,4,5];
	println!("{:?}", numbers);

	// Re-assign value
	numbers[2] = 20;

	// Get single val
	println!("Single Value: {}", numbers[2]);

	// Get array length
	println!("Array Length: {}", numbers.len());

	// Arrays are stack allocated
	println!("Array occupies {} bytes", mem::size_of_val(&numbers));

	// Get Slice
	let slice: &[i32] = &numbers[0..2];
	println!("Slice: {:?}", slice);

}