#![allow(dead_code)]

use derive_more::TryIntoVariant;

#[derive(TryIntoVariant)]
enum Either<TLeft, TRight> {
    Left(TLeft),
    Right(TRight),
}

#[derive(TryIntoVariant)]
#[derive(Debug, PartialEq)]
#[try_into_variant(ref, ref_mut)]
enum Maybe<T: std::fmt::Debug + PartialEq> {
    Nothing,
    Just(T),
}

#[derive(TryIntoVariant)]
enum Color {
    RGB(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}

#[derive(TryIntoVariant)]
enum Nonsense<'a, T> {
    Ref(&'a T),
    NoRef,
    #[try_into_variant(ignore)]
    NoRefIgnored,
}

#[derive(TryIntoVariant)]
enum WithConstraints<T>
where
    T: Copy,
{
    One(T),
    Two,
}
#[derive(TryIntoVariant)]
enum KitchenSink<'a, 'b, T1: Copy, T2: Clone>
where
    T2: Into<T1> + 'b,
{
    Left(&'a T1),
    Right(&'b T2),
    OwnBoth(T1, T2),
    Empty,
    NeverMind(),
    NothingToSeeHere(),
}

#[test]
pub fn test_try_into_variant() {
    assert_eq!(Maybe::<()>::Nothing.try_into_nothing(), Ok(()));
    assert_eq!((&Maybe::Just(1)).try_into_just_ref(), Ok(&1));
    assert_eq!((&mut Maybe::Just(42)).try_into_just_mut(), Ok(&mut 42));

    assert_eq!(
        Maybe::<()>::Nothing.try_into_just(),
        Err(Maybe::<()>::Nothing)
    );
    assert_eq!(
        (&Maybe::Just(1)).try_into_nothing_ref(),
        Err(&Maybe::Just(1))
    );
    assert_eq!(
        (&mut Maybe::Just(42)).try_into_nothing_mut(),
        Err(&mut Maybe::Just(42))
    );
}
