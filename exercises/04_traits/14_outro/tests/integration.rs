use outro_03::SaturatingU16;

#[test]
fn test_saturating_u16() {
    // &u8 -> SaturatingU16
    // impl From<&u8> for SaturatingU16
    let a: SaturatingU16 = (&10u8).into();

    // u8 -> SaturatingU16
    // impl From<u8> for SaturatingU16
    let b: SaturatingU16 = 5u8.into();

    // u16 -> SaturatingU16
    // impl From<u16> for SaturatingU16
    let c: SaturatingU16 = u16::MAX.into();

    // &u16 -> SaturatingU16
    // impl From<&u16> for SaturatingU16
    let d: SaturatingU16 = (&1u16).into();

    // &SaturatingU16
    let e = &c;

    // SaturatingU16 + SaturatingU16, SaturatingU16
    // impl Add<SaturatingU16> for SaturatingU16
    // impl PartialEq<SaturatingU16> for SaturatingU16
    assert_eq!(a + b, SaturatingU16::from(15u16));

    // SaturatingU16 + SaturatingU16, SaturatingU16
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));

    // SaturatingU16 + SaturatingU16, SaturatingU16
    assert_eq!(a + d, SaturatingU16::from(11u16));

    // SaturatingU16 + SaturatingU16, u16
    // impl PartialEq<u16> for SaturatingU16
    assert_eq!(a + a, 20u16);

    // SaturatingU16 + u16, u16
    // impl Add<u16> for SaturatingU16
    assert_eq!(a + 5u16, 15u16);

    // SaturatingU16 + &SaturatingU16, SaturatingU16
    // impl Add<&SaturatingU16> for SaturatingU16
    assert_eq!(a + e, SaturatingU16::from(u16::MAX));
}
