pub fn add_two(a: i32) -> i32 {
    a + 2
}


pub fn find<T: PartialEq>(val: T, arr : &[T]) -> isize {
	let mut inc: isize = 0;
	for k in arr.iter() {
		if *k == val {return inc};
		inc += 1;
	}
	-1
}


#[cfg(test)]
mod tests {
	use super::add_two;
	use super::find;

	#[test]
	fn test_add_two() {
		assert_eq!(4, add_two(2));
	}

	#[test]
	fn test_find() {
		assert_eq!(-1, find(3, &[] ));
		assert_eq!(-1, find(3, &[1]));
		assert_eq!(0,  find(1, &[1]));
		assert_eq!(0,  find(1, &[1, 3, 5]));
		assert_eq!(1,  find(3, &[1, 3, 5]));
		assert_eq!(2,  find(5, &[1, 3, 5]));
		assert_eq!(-1, find(0, &[1, 3, 5]));
		assert_eq!(-1, find(2, &[1, 3, 5]));
		assert_eq!(-1, find(4, &[1, 3, 5]));
		assert_eq!(-1, find(6, &[1, 3, 5]));
		
		assert_eq!(0,  find(1, &[1, 3, 5, 7]));
		assert_eq!(1,  find(3, &[1, 3, 5, 7]));
		assert_eq!(2,  find(5, &[1, 3, 5, 7]));
		assert_eq!(3,  find(7, &[1, 3, 5, 7]));
		assert_eq!(-1, find(0, &[1, 3, 5, 7]));
		assert_eq!(-1, find(2, &[1, 3, 5, 7]));
		assert_eq!(-1, find(4, &[1, 3, 5, 7]));
		assert_eq!(-1, find(6, &[1, 3, 5, 7]));
		assert_eq!(-1, find(8, &[1, 3, 5, 7]));
	}
}
