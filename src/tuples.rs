#[rustfmt::skip]
pub trait TupleMath<T> {
	fn at_least_1(&self, _pred: impl Fn(T) -> bool) -> bool {false}
	fn at_least_2(&self, _pred: impl Fn(T) -> bool) -> bool {false}
	fn at_least_3(&self, _pred: impl Fn(T) -> bool) -> bool {false}
	fn at_least_4(&self, _pred: impl Fn(T) -> bool) -> bool {false}
}

#[rustfmt::skip]
impl<T> TupleMath<T> for () {}

#[rustfmt::skip]
impl<T> TupleMath<T> for (T,) where T: Copy {
	#[inline] fn at_least_1(&self, pred:  impl Fn(T) -> bool) -> bool {pred(self.0)}
}

#[rustfmt::skip]
impl<T> TupleMath<T> for (T,T) where T: Copy {
	#[inline] fn at_least_1(&self, pred:  impl Fn(T) -> bool) -> bool {(self.0,).at_least_1(&pred) || (self.1,).at_least_1(&pred)}
	#[inline] fn at_least_2(&self, pred:  impl Fn(T) -> bool) -> bool {(self.0,).at_least_1(&pred) && (self.1,).at_least_1(&pred)}
}

#[rustfmt::skip]
impl<T> TupleMath<T> for (T,T,T) where T: Copy {
	#[inline] fn at_least_1(&self, pred:  impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_1(&pred) || (self.2,).at_least_1(&pred)}
	#[inline] fn at_least_2(&self, pred:  impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_2(&pred) || ((self.0,self.1).at_least_1(&pred) && (self.2,).at_least_1(&pred))}
	#[inline] fn at_least_3(&self, pred:  impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_2(&pred) && (self.2,).at_least_1(&pred)}
}

#[rustfmt::skip]
impl<T> TupleMath<T> for (T,T,T,T) where T: Copy {
	#[inline] fn at_least_1(&self, pred: impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_1(&pred) || (self.2,self.3).at_least_1(&pred)}
	#[inline] fn at_least_2(&self, pred: impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_2(&pred) || (self.2,self.3).at_least_2(&pred) || ((self.0,self.1).at_least_1(&pred) && (self.2,self.3).at_least_1(&pred))}
	#[inline] fn at_least_3(&self, pred: impl Fn(T) -> bool) -> bool {(self.0,self.1,self.2).at_least_3(&pred) || ((self.0,self.1,self.2).at_least_2(&pred) && (self.3,).at_least_1(&pred))}
	#[inline] fn at_least_4(&self, pred: impl Fn(T) -> bool) -> bool {(self.0,self.1).at_least_2(&pred) && (self.2,self.3).at_least_2(&pred)}
}

#[rustfmt::skip]
pub trait TupleMathEq<T> : TupleMath<T> where T: Eq {
	fn at_least_1_eq(&self, value: T) -> bool {self.at_least_1(|v| v == value)}
	fn at_least_2_eq(&self, value: T) -> bool {self.at_least_2(|v| v == value)}
	fn at_least_3_eq(&self, value: T) -> bool {self.at_least_3(|v| v == value)}
	fn at_least_4_eq(&self, value: T) -> bool {self.at_least_4(|v| v == value)}

	fn at_least_1_is_either_or(&self, value1: T, value2: T) -> bool {self.at_least_1(|v| v == value1 || v == value2)}
	fn at_least_2_is_either_or(&self, value1: T, value2: T) -> bool {self.at_least_2(|v| v == value1 || v == value2)}
	fn at_least_3_is_either_or(&self, value1: T, value2: T) -> bool {self.at_least_3(|v| v == value1 || v == value2)}
	fn at_least_4_is_either_or(&self, value1: T, value2: T) -> bool {self.at_least_4(|v| v == value1 || v == value2)}
}

#[rustfmt::skip] impl<T> TupleMathEq<T> for ()           where T: Copy + Eq {}
#[rustfmt::skip] impl<T> TupleMathEq<T> for (T,)         where T: Copy + Eq {}
#[rustfmt::skip] impl<T> TupleMathEq<T> for (T, T)       where T: Copy + Eq {}
#[rustfmt::skip] impl<T> TupleMathEq<T> for (T, T, T)    where T: Copy + Eq {}
#[rustfmt::skip] impl<T> TupleMathEq<T> for (T, T, T, T) where T: Copy + Eq {}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn test_tuple0_math() {
		// Empty tuple
		assert!(!().at_least_1_eq(1));
		assert!(!().at_least_2_eq(1));
		assert!(!().at_least_3_eq(1));
		assert!(!().at_least_4_eq(1));
	}

	#[test]
	#[rustfmt::skip]
	fn test_tuple1_math() {
		// 1-tuple
		assert!(!(0,).at_least_1_eq(1));
		assert!(!(0,).at_least_2_eq(1));
		assert!(!(0,).at_least_3_eq(1));
		assert!(!(0,).at_least_4_eq(1));
		
		assert!( (1,).at_least_1_eq(1));
		assert!(!(1,).at_least_2_eq(1));
		assert!(!(1,).at_least_3_eq(1));
		assert!(!(1,).at_least_4_eq(1));
	}

	#[test]
	#[rustfmt::skip]
	fn test_tuple2_math() {
		// 2-tuple permutations
		assert!(!(0,0).at_least_1_eq(1));
		assert!(!(0,0).at_least_2_eq(1));
		assert!(!(0,0).at_least_3_eq(1));
		assert!(!(0,0).at_least_4_eq(1));
		
		assert!( (0,1).at_least_1_eq(1));
		assert!(!(0,1).at_least_2_eq(1));
		assert!(!(0,1).at_least_3_eq(1));
		assert!(!(0,1).at_least_4_eq(1));
		
		assert!( (1,0).at_least_1_eq(1));
		assert!(!(1,0).at_least_2_eq(1));
		assert!(!(1,0).at_least_3_eq(1));
		assert!(!(1,0).at_least_4_eq(1));
		
		assert!( (1,1).at_least_1_eq(1));
		assert!( (1,1).at_least_2_eq(1));
		assert!(!(1,1).at_least_3_eq(1));
		assert!(!(1,1).at_least_4_eq(1));
	}

	#[test]
	#[rustfmt::skip]
	fn test_tuple3_math() {
		// 3-tuple
		assert!(!(0,0,0).at_least_1_eq(1));
		assert!(!(0,0,0).at_least_2_eq(1));
		assert!(!(0,0,0).at_least_3_eq(1));
		assert!(!(0,0,0).at_least_4_eq(1));
		
		// 3-tuple permutations with 1
		assert!( (1,0,0).at_least_1_eq(1));
		assert!(!(1,0,0).at_least_2_eq(1));
		assert!(!(1,0,0).at_least_3_eq(1));
		assert!(!(1,0,0).at_least_4_eq(1));
		
		assert!( (0,1,0).at_least_1_eq(1));
		assert!(!(0,1,0).at_least_2_eq(1));
		assert!(!(0,1,0).at_least_3_eq(1));
		assert!(!(0,1,0).at_least_4_eq(1));
		
		assert!( (0,0,1).at_least_1_eq(1));
		assert!(!(0,0,1).at_least_2_eq(1));
		assert!(!(0,0,1).at_least_3_eq(1));
		assert!(!(0,0,1).at_least_4_eq(1));
		
		// 3-tuple permutations with 2
		assert!( (1,1,0).at_least_1_eq(1));
		assert!( (1,1,0).at_least_2_eq(1));
		assert!(!(1,1,0).at_least_3_eq(1));
		assert!(!(1,1,0).at_least_4_eq(1));
		
		assert!( (0,1,1).at_least_1_eq(1));
		assert!( (0,1,1).at_least_2_eq(1));
		assert!(!(0,1,1).at_least_3_eq(1));
		assert!(!(0,1,1).at_least_4_eq(1));

		assert!( (1,0,1).at_least_1_eq(1));
		assert!( (1,0,1).at_least_2_eq(1));
		assert!(!(1,0,1).at_least_3_eq(1));
		assert!(!(1,0,1).at_least_4_eq(1));
		
		// 3-tuple with 3
		assert!( (1,1,1).at_least_1_eq(1));
		assert!( (1,1,1).at_least_2_eq(1));
		assert!( (1,1,1).at_least_3_eq(1));
		assert!(!(1,1,1).at_least_4_eq(1));
	}

	#[test]
	#[rustfmt::skip]
	fn test_tuple4_math() {
		// 4-tuple
		assert!(!(0,0,0,0).at_least_1_eq(1));
		assert!(!(0,0,0,0).at_least_2_eq(1));
		assert!(!(0,0,0,0).at_least_3_eq(1));
		assert!(!(0,0,0,0).at_least_4_eq(1));
		
		// 4-tuple permutations with 1
		assert!( (1,0,0,0).at_least_1_eq(1));
		assert!(!(1,0,0,0).at_least_2_eq(1));
		assert!(!(1,0,0,0).at_least_3_eq(1));
		assert!(!(1,0,0,0).at_least_4_eq(1));
		
		assert!( (0,1,0,0).at_least_1_eq(1));
		assert!(!(0,1,0,0).at_least_2_eq(1));
		assert!(!(0,1,0,0).at_least_3_eq(1));
		assert!(!(0,1,0,0).at_least_4_eq(1));
		
		assert!( (0,0,1,0).at_least_1_eq(1));
		assert!(!(0,0,1,0).at_least_2_eq(1));
		assert!(!(0,0,1,0).at_least_3_eq(1));
		assert!(!(0,0,1,0).at_least_4_eq(1));
		
		assert!( (0,0,0,1).at_least_1_eq(1));
		assert!(!(0,0,0,1).at_least_2_eq(1));
		assert!(!(0,0,0,1).at_least_3_eq(1));
		assert!(!(0,0,0,1).at_least_4_eq(1));
		
		// 4-tuple permutations with 2
		assert!( (1,1,0,0).at_least_1_eq(1));
		assert!( (1,1,0,0).at_least_2_eq(1));
		assert!(!(1,1,0,0).at_least_3_eq(1));
		assert!(!(1,1,0,0).at_least_4_eq(1));
		
		assert!( (0,1,1,0).at_least_1_eq(1));
		assert!( (0,1,1,0).at_least_2_eq(1));
		assert!(!(0,1,1,0).at_least_3_eq(1));
		assert!(!(0,1,1,0).at_least_4_eq(1));
		
		assert!( (0,0,1,1).at_least_1_eq(1));
		assert!( (0,0,1,1).at_least_2_eq(1));
		assert!(!(0,0,1,1).at_least_3_eq(1));
		assert!(!(0,0,1,1).at_least_4_eq(1));
		
		assert!( (1,0,1,0).at_least_1_eq(1));
		assert!( (1,0,1,0).at_least_2_eq(1));
		assert!(!(1,0,1,0).at_least_3_eq(1));
		assert!(!(1,0,1,0).at_least_4_eq(1));
		
		assert!( (0,1,0,1).at_least_1_eq(1));
		assert!( (0,1,0,1).at_least_2_eq(1));
		assert!(!(0,1,0,1).at_least_3_eq(1));
		assert!(!(0,1,0,1).at_least_4_eq(1));
		
		assert!( (1,0,0,1).at_least_1_eq(1));
		assert!( (1,0,0,1).at_least_2_eq(1));
		assert!(!(1,0,0,1).at_least_3_eq(1));
		assert!(!(1,0,0,1).at_least_4_eq(1));
		
		// 4-tuple permutations with 3
		assert!( (1,1,1,0).at_least_1_eq(1));
		assert!( (1,1,1,0).at_least_2_eq(1));
		assert!( (1,1,1,0).at_least_3_eq(1));
		assert!(!(1,1,1,0).at_least_4_eq(1));
		
		assert!( (0,1,1,1).at_least_1_eq(1));
		assert!( (0,1,1,1).at_least_2_eq(1));
		assert!( (0,1,1,1).at_least_3_eq(1));
		assert!(!(0,1,1,1).at_least_4_eq(1));
		
		assert!( (1,1,0,1).at_least_1_eq(1));
		assert!( (1,1,0,1).at_least_2_eq(1));
		assert!( (1,1,0,1).at_least_3_eq(1));
		assert!(!(1,1,0,1).at_least_4_eq(1));
		
		assert!( (1,0,1,1).at_least_1_eq(1));
		assert!( (1,0,1,1).at_least_2_eq(1));
		assert!( (1,0,1,1).at_least_3_eq(1));
		assert!(!(1,0,1,1).at_least_4_eq(1));
		
		// 4-tuple with 4
		assert!( (1,1,1,1).at_least_1_eq(1));
		assert!( (1,1,1,1).at_least_2_eq(1));
		assert!( (1,1,1,1).at_least_3_eq(1));
		assert!( (1,1,1,1).at_least_4_eq(1));
	}
}
