#![allow(dead_code)]

use std::{
	fmt::{self, Display, Formatter},
	marker::PhantomData,
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use paste::paste;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// Macros

#[macro_export]
macro_rules! deg {
	($value:expr) => {{
		$crate::Degrees::new($value).into()
	}};
}

#[macro_export]
macro_rules! rad {
	($value:expr) => {{
		$crate::Radians::new($value).into()
	}};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Angle<T = f32, U: AngleType = RadiansType> {
	value: T,
	_unit_type: PhantomData<U>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitAngleType;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DegreesType;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RadiansType;

pub type UnitAngle<T = f32> = Angle<T, UnitAngleType>;
pub type Degrees<T = f32> = Angle<T, DegreesType>;
pub type Radians<T = f32> = Angle<T, RadiansType>;

pub trait AngleType {}
impl AngleType for UnitAngleType {}
impl AngleType for DegreesType {}
impl AngleType for RadiansType {}

#[rustfmt::skip]
pub trait AngleTurnType {
	fn full_turn()          -> Self;
	fn three_quarter_turn() -> Self;
	fn half_turn()          -> Self;
	fn quarter_turn()       -> Self;
	fn sixth_turn()         -> Self;
	fn eighth_turn()        -> Self;
	fn zero()               -> Self;
}

pub trait AngleDegreesType<T> {
	fn degrees(&self) -> T;
}

pub trait AngleRadiansType<T> {
	fn radians(&self) -> T;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<T, U: AngleType> Angle<T, U> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			_unit_type: PhantomData,
		}
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[macro_export]
macro_rules! impl_angle {
($type:ident) => {

#[rustfmt::skip]
impl AngleTurnType for UnitAngle<$type> {
	fn full_turn()          -> Self {Self::new(1.0)}
	fn three_quarter_turn() -> Self {Self::new(0.75)}
	fn half_turn()          -> Self {Self::new(0.5)}
	fn quarter_turn()       -> Self {Self::new(0.25)}
	fn sixth_turn()         -> Self {Self::new(1./6.)}
	fn eighth_turn()        -> Self {Self::new(0.125)}
	fn zero()               -> Self {Self::new(0.)}
}

#[rustfmt::skip]
impl AngleTurnType for Degrees<$type> {
	fn full_turn()          -> Self {Self::new(360.)}
	fn three_quarter_turn() -> Self {Self::new(270.)}
	fn half_turn()          -> Self {Self::new(180.)}
	fn quarter_turn()       -> Self {Self::new(90.)}
	fn sixth_turn()         -> Self {Self::new(60.)}
	fn eighth_turn()        -> Self {Self::new(45.)}
	fn zero()               -> Self {Self::new(0.)}
}

#[rustfmt::skip]
impl AngleTurnType for Radians<$type> {
	fn full_turn()          -> Self {Self::new(std::$type::consts::TAU)}
	fn three_quarter_turn() -> Self {Self::new(std::$type::consts::PI + std::$type::consts::FRAC_PI_2)}
	fn half_turn()          -> Self {Self::new(std::$type::consts::PI)}
	fn quarter_turn()       -> Self {Self::new(std::$type::consts::FRAC_PI_2)}
	fn sixth_turn()         -> Self {Self::new(std::$type::consts::FRAC_PI_3)}
	fn eighth_turn()        -> Self {Self::new(std::$type::consts::FRAC_PI_4)}
	fn zero()               -> Self {Self::new(0.)}
}


/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl Display for UnitAngle<$type> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.pad(&format!("{}u", self.value))
	}
}

impl Display for Degrees<$type> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.pad(&format!("{}°", self.value))
	}
}

impl Display for Radians<$type> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.pad(&format!("{}rad", self.value))
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[rustfmt::skip] impl AngleDegreesType<$type> for UnitAngle<$type> {fn degrees(&self) -> $type {self.value * 360.}}
#[rustfmt::skip] impl AngleDegreesType<$type> for Degrees<$type>   {fn degrees(&self) -> $type {self.value}}
#[rustfmt::skip] impl AngleDegreesType<$type> for Radians<$type>   {fn degrees(&self) -> $type {self.value.to_degrees()}}

#[rustfmt::skip] impl AngleRadiansType<$type> for UnitAngle<$type> {fn radians(&self) -> $type {self.value * std::$type::consts::TAU}}
#[rustfmt::skip] impl AngleRadiansType<$type> for Degrees<$type>   {fn radians(&self) -> $type {self.value.to_radians()}}
#[rustfmt::skip] impl AngleRadiansType<$type> for Radians<$type>   {fn radians(&self) -> $type {self.value}}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[rustfmt::skip] impl From<UnitAngle<$type>> for Degrees<$type> where UnitAngle<$type>: AngleDegreesType<$type> {fn from(val: UnitAngle<$type>) -> Self {Degrees::new(val.degrees())}}
#[rustfmt::skip] impl From<Degrees<$type>>   for UnitAngle<$type>                                               {fn from(val: Degrees<$type>)   -> Self {UnitAngle::new(val.value / 360.)}}

#[rustfmt::skip] impl From<UnitAngle<$type>> for Radians<$type> where UnitAngle<$type>: AngleRadiansType<$type> {fn from(val: UnitAngle<$type>) -> Self {Radians::new(val.radians())}}
#[rustfmt::skip] impl From<Radians<$type>>   for UnitAngle<$type>                                               {fn from(val: Radians<$type>)   -> Self {UnitAngle::new(val.value / std::$type::consts::TAU)}}

#[rustfmt::skip] impl From<Degrees<$type>>   for Radians<$type> where Degrees<$type>: AngleRadiansType<$type>   {fn from(val: Degrees<$type>)   -> Self {Radians::new(val.radians())}}
#[rustfmt::skip] impl From<Radians<$type>>   for Degrees<$type> where Radians<$type>: AngleDegreesType<$type>   {fn from(val: Radians<$type>)   -> Self {Degrees::new(val.degrees())}}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[rustfmt::skip]
impl<U: AngleType> Angle<$type, U> {
	fn asin(ratio: $type)        -> Self {Self::new(ratio.asin())}
	fn acos(ratio: $type)        -> Self {Self::new(ratio.acos())}
	fn atan(ratio: $type)        -> Self {Self::new(ratio.atan())}
	fn atan2(a: $type, b: $type) -> Self {Self::new(a.atan2(b))}
}

#[rustfmt::skip]
impl<U: AngleType> Angle<$type, U> where Self: AngleTurnType {
	#[inline] pub fn opposite(self)                      -> Self {self - Self::half_turn()}
	#[inline] pub fn clamped(self, min: Self, max: Self) -> Self {Self::new(self.value.clamp(min.value, max.value))}
	#[inline] pub fn clamp(&mut self, min: Self, max: Self)      {self.value = self.value.clamp(min.value, max.value)}
}

#[rustfmt::skip]
impl<U: AngleType> Angle<$type, U> where Self: AngleRadiansType<$type>{
	#[inline] pub fn sin(self)     -> $type          {self.radians().sin()}
	#[inline] pub fn cos(self)     -> $type          {self.radians().cos()}
	#[inline] pub fn tan(self)     -> $type          {self.radians().tan()}
	#[inline] pub fn sin_cos(self) -> ($type, $type) {self.radians().sin_cos()}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[rustfmt::skip] impl<U: AngleType> Neg           for Angle<$type, U> {type Output = Self; fn neg(self)             -> Self::Output {Self::new(-self.value)}}
#[rustfmt::skip] impl<U: AngleType> Add           for Angle<$type, U> {type Output = Self; fn add(self, rhs: Self)  -> Self::Output {Self::new( self.value + rhs.value)}}
#[rustfmt::skip] impl<U: AngleType> Sub           for Angle<$type, U> {type Output = Self; fn sub(self, rhs: Self)  -> Self::Output {Self::new( self.value - rhs.value)}}
#[rustfmt::skip] impl<U: AngleType> Rem           for Angle<$type, U> {type Output = Self; fn rem(self, rhs: Self)  -> Self::Output {Self::new( self.value % rhs.value)}}

#[rustfmt::skip] impl<U: AngleType> Mul<$type>    for Angle<$type, U> {type Output = Self; fn mul(self, rhs: $type) -> Self::Output {Self::new( self.value * rhs)}}
#[rustfmt::skip] impl<U: AngleType> Div<$type>    for Angle<$type, U> {type Output = Self; fn div(self, rhs: $type) -> Self::Output {Self::new( self.value / rhs)}}

#[rustfmt::skip] impl<U: AngleType> Div           for Angle<$type, U> {type Output = $type; fn div(self, rhs: Self) -> Self::Output {self.value / rhs.value}}

#[rustfmt::skip] impl<U: AngleType> AddAssign        for Angle<$type, U> {fn add_assign(&mut self, other: Self)  { self.value += other.value;}}
#[rustfmt::skip] impl<U: AngleType> SubAssign        for Angle<$type, U> {fn sub_assign(&mut self, other: Self)  { self.value -= other.value;}}
#[rustfmt::skip] impl<U: AngleType> RemAssign        for Angle<$type, U> {fn rem_assign(&mut self, other: Self)  { self.value %= other.value;}}

#[rustfmt::skip] impl<U: AngleType> MulAssign<$type> for Angle<$type, U> {fn mul_assign(&mut self, other: $type) { self.value *= other;}}
#[rustfmt::skip] impl<U: AngleType> DivAssign<$type> for Angle<$type, U> {fn div_assign(&mut self, other: $type) { self.value /= other;}}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

paste! {
#[cfg(test)]
mod [<tests_ $type>] {
	use super::*;
	use approx::{assert_relative_eq};
	use rstest::rstest;

	#[rstest]
	#[case(720., std::$type::consts::PI * 4.,                            2.)]
	#[case(360., std::$type::consts::TAU,                                1.)]
	#[case(270., std::$type::consts::PI + std::$type::consts::FRAC_PI_2, 3./4.)]
	#[case(180., std::$type::consts::PI,                                 1./2.)]
	#[case(90.,  std::$type::consts::FRAC_PI_2,                          1./4.)]
	#[case(60.,  std::$type::consts::FRAC_PI_3,                          1./6.)]
	#[case(45.,  std::$type::consts::FRAC_PI_4,                          1./8.)]
	fn test_angle_conversion(#[case] deg: $type, #[case] rad: $type, #[case] uni: $type)
	{
		let a = Degrees::<$type>::new(deg);
		assert_relative_eq!(a.radians(), deg.to_radians());
		assert_relative_eq!(a.degrees(), deg);
		assert_relative_eq!(a.radians(), rad);
		assert_relative_eq!(a.degrees(), rad.to_degrees());

		let b = Radians::<$type>::new(rad);
		assert_relative_eq!(b.radians(), deg.to_radians());
		assert_relative_eq!(b.degrees(), deg);
		assert_relative_eq!(b.radians(), rad);
		assert_relative_eq!(b.degrees(), rad.to_degrees());

		let c = UnitAngle::<$type>::new(uni);
		assert_relative_eq!(c.radians(), deg.to_radians());
		assert_relative_eq!(c.degrees(), deg);
		assert_relative_eq!(c.radians(), rad);
		assert_relative_eq!(c.degrees(), rad.to_degrees());

		assert_relative_eq!(Into::<Degrees<$type>>::into(a).degrees(), a.degrees());
		assert_relative_eq!(Into::<Degrees<$type>>::into(b).degrees(), a.degrees());
		assert_relative_eq!(Into::<Degrees<$type>>::into(c).degrees(), a.degrees());

		assert_relative_eq!(Into::<Radians<$type>>::into(a).radians(), b.radians());
		assert_relative_eq!(Into::<Radians<$type>>::into(b).radians(), b.radians());
		assert_relative_eq!(Into::<Radians<$type>>::into(c).radians(), b.radians());
	}
}
}
};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl_angle!(f32);
impl_angle!(f64);
