#![allow(dead_code)]

use std::{
	fmt::Display,
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
	time::Duration,
};

use crate::angle::{Angle, AngleType};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Speed<T = f32> {
	units_per_second: T,
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[allow(dead_code)]
impl<T> Speed<T> {
	pub fn new_per_second(units_per_second: T) -> Self {
		Self { units_per_second }
	}

	pub fn per_second(self) -> T {
		self.units_per_second
	}
}

impl<T, F> Speed<T>
where
	T: Div<F, Output = T> + DurationConverter<Output = F>,
{
	pub fn new(units: T, duration: Duration) -> Self {
		Self {
			units_per_second: units / T::as_secs(duration),
		}
	}
}

#[macro_export]
macro_rules! spd {
	($value:expr) => {{
		$crate::math::speed::Speed::new_per_second($value).into()
	}};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<T> Default for Speed<T>
where
	T: Default,
{
	fn default() -> Self {
		Self {
			units_per_second: Default::default(),
		}
	}
}

impl<T> Display for Speed<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.units_per_second.fmt(f)?;
		write!(f, " u/s")
	}
}

impl<T, F> Mul<Duration> for Speed<T>
where
	T: Mul<F, Output = T> + DurationConverter<Output = F>,
{
	type Output = T;
	fn mul(self, rhs: Duration) -> Self::Output {
		self.units_per_second * T::as_secs(rhs)
	}
}

#[rustfmt::skip] impl<T: Neg<Output = T>> Neg    for Speed<T> {type Output = Self; fn neg(self)            -> Self::Output {Self::new_per_second(-self.units_per_second)}}
#[rustfmt::skip] impl<T: Add<Output = T>> Add    for Speed<T> {type Output = Self; fn add(self, rhs: Self) -> Self::Output {Self::new_per_second( self.units_per_second + rhs.units_per_second)}}
#[rustfmt::skip] impl<T: Sub<Output = T>> Sub    for Speed<T> {type Output = Self; fn sub(self, rhs: Self) -> Self::Output {Self::new_per_second( self.units_per_second - rhs.units_per_second)}}
#[rustfmt::skip] impl<T: Rem<Output = T>> Rem    for Speed<T> {type Output = Self; fn rem(self, rhs: Self) -> Self::Output {Self::new_per_second( self.units_per_second % rhs.units_per_second)}}

#[rustfmt::skip] impl<T: Mul<Output = T>> Mul<T> for Speed<T> {type Output = Self; fn mul(self, rhs: T)    -> Self::Output {Self::new_per_second( self.units_per_second * rhs)}}
#[rustfmt::skip] impl<T: Div<Output = T>> Div<T> for Speed<T> {type Output = Self; fn div(self, rhs: T)    -> Self::Output {Self::new_per_second( self.units_per_second / rhs)}}

#[rustfmt::skip] impl<T: AddAssign> AddAssign    for Speed<T> {fn add_assign(&mut self, other: Self) { self.units_per_second += other.units_per_second;}}
#[rustfmt::skip] impl<T: SubAssign> SubAssign    for Speed<T> {fn sub_assign(&mut self, other: Self) { self.units_per_second -= other.units_per_second;}}
#[rustfmt::skip] impl<T: RemAssign> RemAssign    for Speed<T> {fn rem_assign(&mut self, other: Self) { self.units_per_second %= other.units_per_second;}}

#[rustfmt::skip] impl<T: MulAssign> MulAssign<T> for Speed<T> {fn mul_assign(&mut self, other: T)    { self.units_per_second *= other;}}
#[rustfmt::skip] impl<T: DivAssign> DivAssign<T> for Speed<T> {fn div_assign(&mut self, other: T)    { self.units_per_second /= other;}}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait DurationConverter {
	type Output;
	fn as_secs(d: Duration) -> Self::Output;
}

#[rustfmt::skip] impl                                                 DurationConverter for f32          {type Output = f32; fn as_secs(d: Duration) -> Self::Output {d.as_secs_f32()}}
#[rustfmt::skip] impl                                                 DurationConverter for f64          {type Output = f64; fn as_secs(d: Duration) -> Self::Output {d.as_secs_f64()}}
#[rustfmt::skip] impl<T: DurationConverter<Output = T>, U: AngleType> DurationConverter for Angle<T, U>  {type Output = T;   fn as_secs(d: Duration) -> Self::Output {T::as_secs(d)}}
#[rustfmt::skip] impl<T: DurationConverter<Output = T>>               DurationConverter for Speed<T>     {type Output = T;   fn as_secs(d: Duration) -> Self::Output {T::as_secs(d)}}
#[rustfmt::skip] impl<T: DurationConverter<Output = T>>               DurationConverter for vek::Vec2<T> {type Output = T;   fn as_secs(d: Duration) -> Self::Output {T::as_secs(d)}}
#[rustfmt::skip] impl<T: DurationConverter<Output = T>>               DurationConverter for vek::Vec3<T> {type Output = T;   fn as_secs(d: Duration) -> Self::Output {T::as_secs(d)}}
#[rustfmt::skip] impl<T: DurationConverter<Output = T>>               DurationConverter for vek::Vec4<T> {type Output = T;   fn as_secs(d: Duration) -> Self::Output {T::as_secs(d)}}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(test)]
mod tests {
	use super::*;
	use crate::angle::{AngleDegreesType, Degrees};
	use approx::assert_relative_eq;

	#[test]
	#[rustfmt::skip]
	fn test_duration_mult() {
		assert_relative_eq!(Speed::new(10., Duration::new(1, 0)) * Duration::new(1, 0), 10.);
		assert_relative_eq!(Speed::new(0.1, Duration::new(1, 0)) * Duration::new(1, 0), 0.1);
		assert_relative_eq!(Speed::new_per_second(10.) * Duration::new(1, 0), 10.);
		assert_relative_eq!(Speed::new_per_second(0.1) * Duration::new(1, 0), 0.1);

		assert_relative_eq!((Speed::new(Degrees::new(10.), Duration::new(1, 0)) * Duration::new(1, 0)).degrees(), 10.);
		assert_relative_eq!((Speed::new(Degrees::new(0.1), Duration::new(1, 0)) * Duration::new(1, 0)).degrees(), 0.1);
		assert_relative_eq!((Speed::new_per_second(Degrees::new(10.)) * Duration::new(1, 0)).degrees(), 10.);
		assert_relative_eq!((Speed::new_per_second(Degrees::new(0.1)) * Duration::new(1, 0)).degrees(), 0.1);
	}
}
