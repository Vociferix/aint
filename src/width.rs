use crate::tybit::{self, Assert, Bit, _0, _1};

pub trait Width {
    const WIDTH: u32;
    type B7: Bit;
    type B6: Bit;
    type B5: Bit;
    type B4: Bit;
    type B3: Bit;
    type B2: Bit;
    type B1: Bit;
    type B0: Bit;

    type Eq<Rhs: Width>: Bit;
    type Ne<Rhs: Width>: Bit;
    type Lt<Rhs: Width>: Bit;
    type Gt<Rhs: Width>: Bit;
    type Le<Rhs: Width>: Bit;
    type Ge<Rhs: Width>: Bit;
    type Add<Rhs: Width>: Width;
}

pub struct WidthVal<B7: Bit, B6: Bit, B5: Bit, B4: Bit, B3: Bit, B2: Bit, B1: Bit, B0: Bit>(
    core::marker::PhantomData<(B7, B6, B5, B4, B3, B2, B1, B0)>,
);

impl<B7: Bit, B6: Bit, B5: Bit, B4: Bit, B3: Bit, B2: Bit, B1: Bit, B0: Bit> Width
    for WidthVal<B7, B6, B5, B4, B3, B2, B1, B0>
{
    const WIDTH: u32 = (B7::VALUE << 7)
        | (B6::VALUE << 6)
        | (B5::VALUE << 5)
        | (B4::VALUE << 4)
        | (B3::VALUE << 3)
        | (B2::VALUE << 2)
        | (B1::VALUE << 1)
        | B0::VALUE;
    type B7 = B7;
    type B6 = B6;
    type B5 = B5;
    type B4 = B4;
    type B3 = B3;
    type B2 = B2;
    type B1 = B1;
    type B0 = B0;

    type Eq<Rhs: Width> = tybit::And<
        tybit::And<
            tybit::And<tybit::Eq<B7, Rhs::B7>, tybit::Eq<B6, Rhs::B6>>,
            tybit::And<tybit::Eq<B5, Rhs::B5>, tybit::Eq<B4, Rhs::B4>>,
        >,
        tybit::And<
            tybit::And<tybit::Eq<B3, Rhs::B3>, tybit::Eq<B2, Rhs::B2>>,
            tybit::And<tybit::Eq<B1, Rhs::B1>, tybit::Eq<B0, Rhs::B0>>,
        >,
    >;

    type Ne<Rhs: Width> = tybit::Not<Self::Eq<Rhs>>;

    type Lt<Rhs: Width> = tybit::Or<
        tybit::Lt<B7, Rhs::B7>,
        tybit::And<
            tybit::Eq<B7, Rhs::B7>,
            tybit::Or<
                tybit::Lt<B6, Rhs::B6>,
                tybit::And<
                    tybit::Eq<B6, Rhs::B6>,
                    tybit::Or<
                        tybit::Lt<B5, Rhs::B5>,
                        tybit::And<
                            tybit::Eq<B5, Rhs::B5>,
                            tybit::Or<
                                tybit::Lt<B4, Rhs::B4>,
                                tybit::And<
                                    tybit::Eq<B4, Rhs::B4>,
                                    tybit::Or<
                                        tybit::Lt<B3, Rhs::B3>,
                                        tybit::And<
                                            tybit::Eq<B3, Rhs::B3>,
                                            tybit::Or<
                                                tybit::Lt<B2, Rhs::B2>,
                                                tybit::And<
                                                    tybit::Eq<B2, Rhs::B2>,
                                                    tybit::Or<
                                                        tybit::Lt<B1, Rhs::B1>,
                                                        tybit::And<
                                                            tybit::Eq<B1, Rhs::B1>,
                                                            tybit::Lt<B0, Rhs::B0>,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >;

    type Gt<Rhs: Width> = Rhs::Lt<Self>;

    type Le<Rhs: Width> = tybit::Not<Self::Gt<Rhs>>;

    type Ge<Rhs: Width> = tybit::Not<Self::Lt<Rhs>>;

    type Add<Rhs: Width> = WidthVal<
        tybit::Add<
            B7,
            Rhs::B7,
            tybit::AddCarry<
                B6,
                Rhs::B6,
                tybit::AddCarry<
                    B5,
                    Rhs::B5,
                    tybit::AddCarry<
                        B4,
                        Rhs::B4,
                        tybit::AddCarry<
                            B3,
                            Rhs::B3,
                            tybit::AddCarry<
                                B2,
                                Rhs::B1,
                                tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
                            >,
                        >,
                    >,
                >,
            >,
        >,
        tybit::Add<
            B6,
            Rhs::B6,
            tybit::AddCarry<
                B5,
                Rhs::B5,
                tybit::AddCarry<
                    B4,
                    Rhs::B4,
                    tybit::AddCarry<
                        B3,
                        Rhs::B3,
                        tybit::AddCarry<
                            B2,
                            Rhs::B1,
                            tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
                        >,
                    >,
                >,
            >,
        >,
        tybit::Add<
            B5,
            Rhs::B5,
            tybit::AddCarry<
                B4,
                Rhs::B4,
                tybit::AddCarry<
                    B3,
                    Rhs::B3,
                    tybit::AddCarry<
                        B2,
                        Rhs::B1,
                        tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
                    >,
                >,
            >,
        >,
        tybit::Add<
            B4,
            Rhs::B4,
            tybit::AddCarry<
                B3,
                Rhs::B3,
                tybit::AddCarry<
                    B2,
                    Rhs::B1,
                    tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
                >,
            >,
        >,
        tybit::Add<
            B3,
            Rhs::B3,
            tybit::AddCarry<
                B2,
                Rhs::B1,
                tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
            >,
        >,
        tybit::Add<B2, Rhs::B2, tybit::AddCarry<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>>,
        tybit::Add<B1, Rhs::B1, tybit::AddCarry<B0, Rhs::B0>>,
        tybit::Add<B0, Rhs::B0>,
    >;
}

pub trait WidthEq<Rhs: Width>: Width {}
pub trait WidthNe<Rhs: Width>: Width {}
pub trait WidthLt<Rhs: Width>: Width {}
pub trait WidthGt<Rhs: Width>: Width {}
pub trait WidthLe<Rhs: Width>: Width {}
pub trait WidthGe<Rhs: Width>: Width {}

impl<L: Width, R: Width> WidthEq<R> for L where L::Eq<R>: Assert {}

impl<L: Width, R: Width> WidthNe<R> for L where L::Ne<R>: Assert {}

impl<L: Width, R: Width> WidthLt<R> for L where L::Lt<R>: Assert {}

impl<L: Width, R: Width> WidthGt<R> for L where L::Gt<R>: Assert {}

impl<L: Width, R: Width> WidthLe<R> for L where L::Le<R>: Assert {}

impl<L: Width, R: Width> WidthGe<R> for L where L::Ge<R>: Assert {}

pub struct W<const WIDTH: u32>;

macro_rules! impl_w {
    ($width:literal, $b7:ident, $b6:ident, $b5:ident, $b4:ident, $b3:ident, $b2:ident, $b1:ident, $b0:ident) => {
        impl Width for W<$width> {
            const WIDTH: u32 = $width;
            type B7 = $b7;
            type B6 = $b6;
            type B5 = $b5;
            type B4 = $b4;
            type B3 = $b3;
            type B2 = $b2;
            type B1 = $b1;
            type B0 = $b0;

            type Eq<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Eq<Rhs>;

            type Ne<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Ne<Rhs>;

            type Lt<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Lt<Rhs>;

            type Gt<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Gt<Rhs>;

            type Le<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Le<Rhs>;

            type Ge<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Ge<Rhs>;

            type Add<Rhs: Width> =
                <WidthVal<$b7, $b6, $b5, $b4, $b3, $b2, $b1, $b0> as Width>::Add<Rhs>;
        }
    };
}

impl_w!(0, _0, _0, _0, _0, _0, _0, _0, _0);
impl_w!(1, _0, _0, _0, _0, _0, _0, _0, _1);
impl_w!(2, _0, _0, _0, _0, _0, _0, _1, _0);
impl_w!(3, _0, _0, _0, _0, _0, _0, _1, _1);
impl_w!(4, _0, _0, _0, _0, _0, _1, _0, _0);
impl_w!(5, _0, _0, _0, _0, _0, _1, _0, _1);
impl_w!(6, _0, _0, _0, _0, _0, _1, _1, _0);
impl_w!(7, _0, _0, _0, _0, _0, _1, _1, _1);
impl_w!(8, _0, _0, _0, _0, _1, _0, _0, _0);
impl_w!(9, _0, _0, _0, _0, _1, _0, _0, _1);
impl_w!(10, _0, _0, _0, _0, _1, _0, _1, _0);
impl_w!(11, _0, _0, _0, _0, _1, _0, _1, _1);
impl_w!(12, _0, _0, _0, _0, _1, _1, _0, _0);
impl_w!(13, _0, _0, _0, _0, _1, _1, _0, _1);
impl_w!(14, _0, _0, _0, _0, _1, _1, _1, _0);
impl_w!(15, _0, _0, _0, _0, _1, _1, _1, _1);
impl_w!(16, _0, _0, _0, _1, _0, _0, _0, _0);
impl_w!(17, _0, _0, _0, _1, _0, _0, _0, _1);
impl_w!(18, _0, _0, _0, _1, _0, _0, _1, _0);
impl_w!(19, _0, _0, _0, _1, _0, _0, _1, _1);
impl_w!(20, _0, _0, _0, _1, _0, _1, _0, _0);
impl_w!(21, _0, _0, _0, _1, _0, _1, _0, _1);
impl_w!(22, _0, _0, _0, _1, _0, _1, _1, _0);
impl_w!(23, _0, _0, _0, _1, _0, _1, _1, _1);
impl_w!(24, _0, _0, _0, _1, _1, _0, _0, _0);
impl_w!(25, _0, _0, _0, _1, _1, _0, _0, _1);
impl_w!(26, _0, _0, _0, _1, _1, _0, _1, _0);
impl_w!(27, _0, _0, _0, _1, _1, _0, _1, _1);
impl_w!(28, _0, _0, _0, _1, _1, _1, _0, _0);
impl_w!(29, _0, _0, _0, _1, _1, _1, _0, _1);
impl_w!(30, _0, _0, _0, _1, _1, _1, _1, _0);
impl_w!(31, _0, _0, _0, _1, _1, _1, _1, _1);
impl_w!(32, _0, _0, _1, _0, _0, _0, _0, _0);
impl_w!(33, _0, _0, _1, _0, _0, _0, _0, _1);
impl_w!(34, _0, _0, _1, _0, _0, _0, _1, _0);
impl_w!(35, _0, _0, _1, _0, _0, _0, _1, _1);
impl_w!(36, _0, _0, _1, _0, _0, _1, _0, _0);
impl_w!(37, _0, _0, _1, _0, _0, _1, _0, _1);
impl_w!(38, _0, _0, _1, _0, _0, _1, _1, _0);
impl_w!(39, _0, _0, _1, _0, _0, _1, _1, _1);
impl_w!(40, _0, _0, _1, _0, _1, _0, _0, _0);
impl_w!(41, _0, _0, _1, _0, _1, _0, _0, _1);
impl_w!(42, _0, _0, _1, _0, _1, _0, _1, _0);
impl_w!(43, _0, _0, _1, _0, _1, _0, _1, _1);
impl_w!(44, _0, _0, _1, _0, _1, _1, _0, _0);
impl_w!(45, _0, _0, _1, _0, _1, _1, _0, _1);
impl_w!(46, _0, _0, _1, _0, _1, _1, _1, _0);
impl_w!(47, _0, _0, _1, _0, _1, _1, _1, _1);
impl_w!(48, _0, _0, _1, _1, _0, _0, _0, _0);
impl_w!(49, _0, _0, _1, _1, _0, _0, _0, _1);
impl_w!(50, _0, _0, _1, _1, _0, _0, _1, _0);
impl_w!(51, _0, _0, _1, _1, _0, _0, _1, _1);
impl_w!(52, _0, _0, _1, _1, _0, _1, _0, _0);
impl_w!(53, _0, _0, _1, _1, _0, _1, _0, _1);
impl_w!(54, _0, _0, _1, _1, _0, _1, _1, _0);
impl_w!(55, _0, _0, _1, _1, _0, _1, _1, _1);
impl_w!(56, _0, _0, _1, _1, _1, _0, _0, _0);
impl_w!(57, _0, _0, _1, _1, _1, _0, _0, _1);
impl_w!(58, _0, _0, _1, _1, _1, _0, _1, _0);
impl_w!(59, _0, _0, _1, _1, _1, _0, _1, _1);
impl_w!(60, _0, _0, _1, _1, _1, _1, _0, _0);
impl_w!(61, _0, _0, _1, _1, _1, _1, _0, _1);
impl_w!(62, _0, _0, _1, _1, _1, _1, _1, _0);
impl_w!(63, _0, _0, _1, _1, _1, _1, _1, _1);
impl_w!(64, _0, _1, _0, _0, _0, _0, _0, _0);
impl_w!(65, _0, _1, _0, _0, _0, _0, _0, _1);
impl_w!(66, _0, _1, _0, _0, _0, _0, _1, _0);
impl_w!(67, _0, _1, _0, _0, _0, _0, _1, _1);
impl_w!(68, _0, _1, _0, _0, _0, _1, _0, _0);
impl_w!(69, _0, _1, _0, _0, _0, _1, _0, _1);
impl_w!(70, _0, _1, _0, _0, _0, _1, _1, _0);
impl_w!(71, _0, _1, _0, _0, _0, _1, _1, _1);
impl_w!(72, _0, _1, _0, _0, _1, _0, _0, _0);
impl_w!(73, _0, _1, _0, _0, _1, _0, _0, _1);
impl_w!(74, _0, _1, _0, _0, _1, _0, _1, _0);
impl_w!(75, _0, _1, _0, _0, _1, _0, _1, _1);
impl_w!(76, _0, _1, _0, _0, _1, _1, _0, _0);
impl_w!(77, _0, _1, _0, _0, _1, _1, _0, _1);
impl_w!(78, _0, _1, _0, _0, _1, _1, _1, _0);
impl_w!(79, _0, _1, _0, _0, _1, _1, _1, _1);
impl_w!(80, _0, _1, _0, _1, _0, _0, _0, _0);
impl_w!(81, _0, _1, _0, _1, _0, _0, _0, _1);
impl_w!(82, _0, _1, _0, _1, _0, _0, _1, _0);
impl_w!(83, _0, _1, _0, _1, _0, _0, _1, _1);
impl_w!(84, _0, _1, _0, _1, _0, _1, _0, _0);
impl_w!(85, _0, _1, _0, _1, _0, _1, _0, _1);
impl_w!(86, _0, _1, _0, _1, _0, _1, _1, _0);
impl_w!(87, _0, _1, _0, _1, _0, _1, _1, _1);
impl_w!(88, _0, _1, _0, _1, _1, _0, _0, _0);
impl_w!(89, _0, _1, _0, _1, _1, _0, _0, _1);
impl_w!(90, _0, _1, _0, _1, _1, _0, _1, _0);
impl_w!(91, _0, _1, _0, _1, _1, _0, _1, _1);
impl_w!(92, _0, _1, _0, _1, _1, _1, _0, _0);
impl_w!(93, _0, _1, _0, _1, _1, _1, _0, _1);
impl_w!(94, _0, _1, _0, _1, _1, _1, _1, _0);
impl_w!(95, _0, _1, _0, _1, _1, _1, _1, _1);
impl_w!(96, _0, _1, _1, _0, _0, _0, _0, _0);
impl_w!(97, _0, _1, _1, _0, _0, _0, _0, _1);
impl_w!(98, _0, _1, _1, _0, _0, _0, _1, _0);
impl_w!(99, _0, _1, _1, _0, _0, _0, _1, _1);
impl_w!(100, _0, _1, _1, _0, _0, _1, _0, _0);
impl_w!(101, _0, _1, _1, _0, _0, _1, _0, _1);
impl_w!(102, _0, _1, _1, _0, _0, _1, _1, _0);
impl_w!(103, _0, _1, _1, _0, _0, _1, _1, _1);
impl_w!(104, _0, _1, _1, _0, _1, _0, _0, _0);
impl_w!(105, _0, _1, _1, _0, _1, _0, _0, _1);
impl_w!(106, _0, _1, _1, _0, _1, _0, _1, _0);
impl_w!(107, _0, _1, _1, _0, _1, _0, _1, _1);
impl_w!(108, _0, _1, _1, _0, _1, _1, _0, _0);
impl_w!(109, _0, _1, _1, _0, _1, _1, _0, _1);
impl_w!(110, _0, _1, _1, _0, _1, _1, _1, _0);
impl_w!(111, _0, _1, _1, _0, _1, _1, _1, _1);
impl_w!(112, _0, _1, _1, _1, _0, _0, _0, _0);
impl_w!(113, _0, _1, _1, _1, _0, _0, _0, _1);
impl_w!(114, _0, _1, _1, _1, _0, _0, _1, _0);
impl_w!(115, _0, _1, _1, _1, _0, _0, _1, _1);
impl_w!(116, _0, _1, _1, _1, _0, _1, _0, _0);
impl_w!(117, _0, _1, _1, _1, _0, _1, _0, _1);
impl_w!(118, _0, _1, _1, _1, _0, _1, _1, _0);
impl_w!(119, _0, _1, _1, _1, _0, _1, _1, _1);
impl_w!(120, _0, _1, _1, _1, _1, _0, _0, _0);
impl_w!(121, _0, _1, _1, _1, _1, _0, _0, _1);
impl_w!(122, _0, _1, _1, _1, _1, _0, _1, _0);
impl_w!(123, _0, _1, _1, _1, _1, _0, _1, _1);
impl_w!(124, _0, _1, _1, _1, _1, _1, _0, _0);
impl_w!(125, _0, _1, _1, _1, _1, _1, _0, _1);
impl_w!(126, _0, _1, _1, _1, _1, _1, _1, _0);
impl_w!(127, _0, _1, _1, _1, _1, _1, _1, _1);
impl_w!(128, _1, _0, _0, _0, _0, _0, _0, _0);
