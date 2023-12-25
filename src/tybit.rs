pub enum _0 {}
pub enum _1 {}

pub trait Bit {
    const VALUE: u32;
    type Not: Bit;
    type And<Rhs: Bit>: Bit;
    type Or<Rhs: Bit>: Bit;
    type Xor<Rhs: Bit>: Bit;
    type Eq<Rhs: Bit>: Bit;
    type Ne<Rhs: Bit>: Bit;
    type Lt<Rhs: Bit>: Bit;
    type Gt<Rhs: Bit>: Bit;
    type Le<Rhs: Bit>: Bit;
    type Ge<Rhs: Bit>: Bit;
    type Add<Rhs: Bit, Carry: Bit>: Bit;
    type AddCarry<Rhs: Bit, Carry: Bit>: Bit;
}

pub trait Assert: Bit {}

impl Bit for _0 {
    const VALUE: u32 = 0;
    type Not = _1;
    type And<Rhs: Bit> = <_0 as ops::And<Rhs>>::Res;
    type Or<Rhs: Bit> = <_0 as ops::Or<Rhs>>::Res;
    type Xor<Rhs: Bit> = <_0 as ops::Xor<Rhs>>::Res;
    type Eq<Rhs: Bit> = <_0 as ops::Eq<Rhs>>::Res;
    type Ne<Rhs: Bit> = <_0 as ops::Ne<Rhs>>::Res;
    type Lt<Rhs: Bit> = <_0 as ops::Lt<Rhs>>::Res;
    type Gt<Rhs: Bit> = <_0 as ops::Gt<Rhs>>::Res;
    type Le<Rhs: Bit> = <_0 as ops::Le<Rhs>>::Res;
    type Ge<Rhs: Bit> = <_0 as ops::Ge<Rhs>>::Res;
    type Add<Rhs: Bit, Carry: Bit> = <_0 as ops::Add<Rhs, Carry>>::Res;
    type AddCarry<Rhs: Bit, Carry: Bit> = <_0 as ops::Add<Rhs, Carry>>::Carry;
}

impl Bit for _1 {
    const VALUE: u32 = 1;
    type Not = _0;
    type And<Rhs: Bit> = <_1 as ops::And<Rhs>>::Res;
    type Or<Rhs: Bit> = <_1 as ops::Or<Rhs>>::Res;
    type Xor<Rhs: Bit> = <_1 as ops::Xor<Rhs>>::Res;
    type Eq<Rhs: Bit> = <_1 as ops::Eq<Rhs>>::Res;
    type Ne<Rhs: Bit> = <_1 as ops::Ne<Rhs>>::Res;
    type Lt<Rhs: Bit> = <_1 as ops::Lt<Rhs>>::Res;
    type Gt<Rhs: Bit> = <_1 as ops::Gt<Rhs>>::Res;
    type Le<Rhs: Bit> = <_1 as ops::Le<Rhs>>::Res;
    type Ge<Rhs: Bit> = <_1 as ops::Ge<Rhs>>::Res;
    type Add<Rhs: Bit, Carry: Bit> = <_1 as ops::Add<Rhs, Carry>>::Res;
    type AddCarry<Rhs: Bit, Carry: Bit> = <_1 as ops::Add<Rhs, Carry>>::Carry;
}

impl Assert for _1 {}

pub type Not<B> = <B as Bit>::Not;
pub type And<Lhs, Rhs> = <Lhs as Bit>::And<Rhs>;
pub type Or<Lhs, Rhs> = <Lhs as Bit>::Or<Rhs>;
//pub type Xor<Lhs, Rhs> = <Lhs as Bit>::Xor<Rhs>;
pub type Eq<Lhs, Rhs> = <Lhs as Bit>::Eq<Rhs>;
//pub type Ne<Lhs, Rhs> = <Lhs as Bit>::Ne<Rhs>;
pub type Lt<Lhs, Rhs> = <Lhs as Bit>::Lt<Rhs>;
//pub type Gt<Lhs, Rhs> = <Lhs as Bit>::Gt<Rhs>;
//pub type Le<Lhs, Rhs> = <Lhs as Bit>::Le<Rhs>;
//pub type Ge<Lhs, Rhs> = <Lhs as Bit>::Ge<Rhs>;
pub type Add<Lhs, Rhs, CarryIn = _0> = <Lhs as Bit>::Add<Rhs, CarryIn>;
pub type AddCarry<Lhs, Rhs, CarryIn = _0> = <Lhs as Bit>::AddCarry<Rhs, CarryIn>;

mod ops {
    use super::{Bit, _0, _1};

    pub trait And<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Or<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Xor<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Eq<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Ne<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Lt<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Gt<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Le<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Ge<Rhs: Bit>: Bit {
        type Res: Bit;
    }

    pub trait Add<Rhs: Bit, CarryIn: Bit>: Bit {
        type Res: Bit;
        type Carry: Bit;
    }

    impl<Rhs: Bit> And<Rhs> for _0 {
        type Res = _0;
    }

    impl<Rhs: Bit> And<Rhs> for _1 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Or<Rhs> for _0 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Or<Rhs> for _1 {
        type Res = _1;
    }

    impl<Rhs: Bit> Xor<Rhs> for _0 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Xor<Rhs> for _1 {
        type Res = Rhs::Not;
    }

    impl<Rhs: Bit> Eq<Rhs> for _0 {
        type Res = Rhs::Not;
    }

    impl<Rhs: Bit> Eq<Rhs> for _1 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Ne<Rhs> for _0 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Ne<Rhs> for _1 {
        type Res = Rhs::Not;
    }

    impl<Rhs: Bit> Lt<Rhs> for _0 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Lt<Rhs> for _1 {
        type Res = _0;
    }

    impl<Rhs: Bit> Gt<Rhs> for _0 {
        type Res = _0;
    }

    impl<Rhs: Bit> Gt<Rhs> for _1 {
        type Res = Rhs::Not;
    }

    impl<Rhs: Bit> Le<Rhs> for _0 {
        type Res = _1;
    }

    impl<Rhs: Bit> Le<Rhs> for _1 {
        type Res = Rhs;
    }

    impl<Rhs: Bit> Ge<Rhs> for _0 {
        type Res = Rhs::Not;
    }

    impl<Rhs: Bit> Ge<Rhs> for _1 {
        type Res = _1;
    }

    impl<Rhs: Bit, CarryIn: Bit> Add<Rhs, CarryIn> for _0 {
        type Res = Rhs::Xor<CarryIn>;
        type Carry = Rhs::And<CarryIn>;
    }

    impl<Rhs: Bit, CarryIn: Bit> Add<Rhs, CarryIn> for _1 {
        type Res = <Rhs::Not as Bit>::Xor<CarryIn>;
        type Carry = Rhs::Or<<Rhs::Not as Bit>::And<CarryIn>>;
    }
}
