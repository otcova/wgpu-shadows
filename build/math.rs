use std::fmt::Display;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2::new({}", self.x)?;
        if self.x.fract() == 0. {
            write!(f, ".")?;
        }
        write!(f, ", {}", self.y)?;
        if self.y.fract() == 0. {
            write!(f, ".")?;
        }
        write!(f, ")")
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

macro_rules! impl_op {
    ($Trait:ident, $fn:ident, $symbol:tt) => {
        impl std::ops::$Trait<Vec2> for Vec2 {
            type Output = Vec2;
            #[inline]
            fn $fn(self, rhs: Vec2) -> Self::Output {
                Self {
                    x: self.x $symbol rhs.x,
                    y: self.y $symbol rhs.y,
                }
            }
        }

        impl std::ops::$Trait<f32> for Vec2 {
            type Output = Vec2;
            #[inline]
            fn $fn(self, rhs: f32) -> Self::Output {
                Self {
                    x: self.x $symbol rhs,
                    y: self.y $symbol rhs,
                }
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

macro_rules! impl_op_assign {
    ($Trait:ident, $fn:ident, $symbol:tt) => {
        impl std::ops::$Trait<Vec2> for Vec2 {
            #[inline]
            fn $fn(&mut self, rhs: Vec2) {
                self.x $symbol rhs.x;
                self.y $symbol rhs.y;
            }
        }
        impl std::ops::$Trait<f32> for Vec2 {
            #[inline]
            fn $fn(&mut self, rhs: f32) {
                self.x $symbol rhs;
                self.y $symbol rhs;
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);
impl_op_assign!(DivAssign, div_assign, /=);
