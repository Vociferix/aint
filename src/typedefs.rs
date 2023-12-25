#![allow(non_camel_case_types)]

use super::Aint;

/// A single bit, unsigned integer
pub type bit = Aint<u8, 1>;

/// A 1-bit unsigned integer
pub type u1 = Aint<u8, 1>;
/// A 2-bit unsigned integer
pub type u2 = Aint<u8, 2>;
/// A 3-bit unsigned integer
pub type u3 = Aint<u8, 3>;
/// A 4-bit unsigned integer
pub type u4 = Aint<u8, 4>;
/// A 5-bit unsigned integer
pub type u5 = Aint<u8, 5>;
/// A 6-bit unsigned integer
pub type u6 = Aint<u8, 6>;
/// A 7-bit unsigned integer
pub type u7 = Aint<u8, 7>;
/// A 8-bit unsigned integer
pub type u8 = core::primitive::u8;
/// A 9-bit unsigned integer
pub type u9 = Aint<u16, 9>;
/// A 10-bit unsigned integer
pub type u10 = Aint<u16, 10>;
/// A 11-bit unsigned integer
pub type u11 = Aint<u16, 11>;
/// A 12-bit unsigned integer
pub type u12 = Aint<u16, 12>;
/// A 13-bit unsigned integer
pub type u13 = Aint<u16, 13>;
/// A 14-bit unsigned integer
pub type u14 = Aint<u16, 14>;
/// A 15-bit unsigned integer
pub type u15 = Aint<u16, 15>;
/// A 16-bit unsigned integer
pub type u16 = core::primitive::u16;
/// A 17-bit unsigned integer
pub type u17 = Aint<u32, 17>;
/// A 18-bit unsigned integer
pub type u18 = Aint<u32, 18>;
/// A 19-bit unsigned integer
pub type u19 = Aint<u32, 19>;
/// A 20-bit unsigned integer
pub type u20 = Aint<u32, 20>;
/// A 21-bit unsigned integer
pub type u21 = Aint<u32, 21>;
/// A 22-bit unsigned integer
pub type u22 = Aint<u32, 22>;
/// A 23-bit unsigned integer
pub type u23 = Aint<u32, 23>;
/// A 24-bit unsigned integer
pub type u24 = Aint<u32, 24>;
/// A 25-bit unsigned integer
pub type u25 = Aint<u32, 25>;
/// A 26-bit unsigned integer
pub type u26 = Aint<u32, 26>;
/// A 27-bit unsigned integer
pub type u27 = Aint<u32, 27>;
/// A 28-bit unsigned integer
pub type u28 = Aint<u32, 28>;
/// A 29-bit unsigned integer
pub type u29 = Aint<u32, 29>;
/// A 30-bit unsigned integer
pub type u30 = Aint<u32, 30>;
/// A 31-bit unsigned integer
pub type u31 = Aint<u32, 31>;
/// A 32-bit unsigned integer
pub type u32 = core::primitive::u32;
/// A 33-bit unsigned integer
pub type u33 = Aint<u64, 33>;
/// A 34-bit unsigned integer
pub type u34 = Aint<u64, 34>;
/// A 35-bit unsigned integer
pub type u35 = Aint<u64, 35>;
/// A 36-bit unsigned integer
pub type u36 = Aint<u64, 36>;
/// A 37-bit unsigned integer
pub type u37 = Aint<u64, 37>;
/// A 38-bit unsigned integer
pub type u38 = Aint<u64, 38>;
/// A 39-bit unsigned integer
pub type u39 = Aint<u64, 39>;
/// A 40-bit unsigned integer
pub type u40 = Aint<u64, 40>;
/// A 41-bit unsigned integer
pub type u41 = Aint<u64, 41>;
/// A 42-bit unsigned integer
pub type u42 = Aint<u64, 42>;
/// A 43-bit unsigned integer
pub type u43 = Aint<u64, 43>;
/// A 44-bit unsigned integer
pub type u44 = Aint<u64, 44>;
/// A 45-bit unsigned integer
pub type u45 = Aint<u64, 45>;
/// A 46-bit unsigned integer
pub type u46 = Aint<u64, 46>;
/// A 47-bit unsigned integer
pub type u47 = Aint<u64, 47>;
/// A 48-bit unsigned integer
pub type u48 = Aint<u64, 48>;
/// A 49-bit unsigned integer
pub type u49 = Aint<u64, 49>;
/// A 50-bit unsigned integer
pub type u50 = Aint<u64, 50>;
/// A 51-bit unsigned integer
pub type u51 = Aint<u64, 51>;
/// A 52-bit unsigned integer
pub type u52 = Aint<u64, 52>;
/// A 53-bit unsigned integer
pub type u53 = Aint<u64, 53>;
/// A 54-bit unsigned integer
pub type u54 = Aint<u64, 54>;
/// A 55-bit unsigned integer
pub type u55 = Aint<u64, 55>;
/// A 56-bit unsigned integer
pub type u56 = Aint<u64, 56>;
/// A 57-bit unsigned integer
pub type u57 = Aint<u64, 57>;
/// A 58-bit unsigned integer
pub type u58 = Aint<u64, 58>;
/// A 59-bit unsigned integer
pub type u59 = Aint<u64, 59>;
/// A 60-bit unsigned integer
pub type u60 = Aint<u64, 60>;
/// A 61-bit unsigned integer
pub type u61 = Aint<u64, 61>;
/// A 62-bit unsigned integer
pub type u62 = Aint<u64, 62>;
/// A 63-bit unsigned integer
pub type u63 = Aint<u64, 63>;
/// A 64-bit unsigned integer
pub type u64 = core::primitive::u64;
/// A 65-bit unsigned integer
pub type u65 = Aint<u128, 65>;
/// A 66-bit unsigned integer
pub type u66 = Aint<u128, 66>;
/// A 67-bit unsigned integer
pub type u67 = Aint<u128, 67>;
/// A 68-bit unsigned integer
pub type u68 = Aint<u128, 68>;
/// A 69-bit unsigned integer
pub type u69 = Aint<u128, 69>;
/// A 70-bit unsigned integer
pub type u70 = Aint<u128, 70>;
/// A 71-bit unsigned integer
pub type u71 = Aint<u128, 71>;
/// A 72-bit unsigned integer
pub type u72 = Aint<u128, 72>;
/// A 73-bit unsigned integer
pub type u73 = Aint<u128, 73>;
/// A 74-bit unsigned integer
pub type u74 = Aint<u128, 74>;
/// A 75-bit unsigned integer
pub type u75 = Aint<u128, 75>;
/// A 76-bit unsigned integer
pub type u76 = Aint<u128, 76>;
/// A 77-bit unsigned integer
pub type u77 = Aint<u128, 77>;
/// A 78-bit unsigned integer
pub type u78 = Aint<u128, 78>;
/// A 79-bit unsigned integer
pub type u79 = Aint<u128, 79>;
/// A 80-bit unsigned integer
pub type u80 = Aint<u128, 80>;
/// A 81-bit unsigned integer
pub type u81 = Aint<u128, 81>;
/// A 82-bit unsigned integer
pub type u82 = Aint<u128, 82>;
/// A 83-bit unsigned integer
pub type u83 = Aint<u128, 83>;
/// A 84-bit unsigned integer
pub type u84 = Aint<u128, 84>;
/// A 85-bit unsigned integer
pub type u85 = Aint<u128, 85>;
/// A 86-bit unsigned integer
pub type u86 = Aint<u128, 86>;
/// A 87-bit unsigned integer
pub type u87 = Aint<u128, 87>;
/// A 88-bit unsigned integer
pub type u88 = Aint<u128, 88>;
/// A 89-bit unsigned integer
pub type u89 = Aint<u128, 89>;
/// A 90-bit unsigned integer
pub type u90 = Aint<u128, 90>;
/// A 91-bit unsigned integer
pub type u91 = Aint<u128, 91>;
/// A 92-bit unsigned integer
pub type u92 = Aint<u128, 92>;
/// A 93-bit unsigned integer
pub type u93 = Aint<u128, 93>;
/// A 94-bit unsigned integer
pub type u94 = Aint<u128, 94>;
/// A 95-bit unsigned integer
pub type u95 = Aint<u128, 95>;
/// A 96-bit unsigned integer
pub type u96 = Aint<u128, 96>;
/// A 97-bit unsigned integer
pub type u97 = Aint<u128, 97>;
/// A 98-bit unsigned integer
pub type u98 = Aint<u128, 98>;
/// A 99-bit unsigned integer
pub type u99 = Aint<u128, 99>;
/// A 100-bit unsigned integer
pub type u100 = Aint<u128, 100>;
/// A 101-bit unsigned integer
pub type u101 = Aint<u128, 101>;
/// A 102-bit unsigned integer
pub type u102 = Aint<u128, 102>;
/// A 103-bit unsigned integer
pub type u103 = Aint<u128, 103>;
/// A 104-bit unsigned integer
pub type u104 = Aint<u128, 104>;
/// A 105-bit unsigned integer
pub type u105 = Aint<u128, 105>;
/// A 106-bit unsigned integer
pub type u106 = Aint<u128, 106>;
/// A 107-bit unsigned integer
pub type u107 = Aint<u128, 107>;
/// A 108-bit unsigned integer
pub type u108 = Aint<u128, 108>;
/// A 109-bit unsigned integer
pub type u109 = Aint<u128, 109>;
/// A 110-bit unsigned integer
pub type u110 = Aint<u128, 110>;
/// A 111-bit unsigned integer
pub type u111 = Aint<u128, 111>;
/// A 112-bit unsigned integer
pub type u112 = Aint<u128, 112>;
/// A 113-bit unsigned integer
pub type u113 = Aint<u128, 113>;
/// A 114-bit unsigned integer
pub type u114 = Aint<u128, 114>;
/// A 115-bit unsigned integer
pub type u115 = Aint<u128, 115>;
/// A 116-bit unsigned integer
pub type u116 = Aint<u128, 116>;
/// A 117-bit unsigned integer
pub type u117 = Aint<u128, 117>;
/// A 118-bit unsigned integer
pub type u118 = Aint<u128, 118>;
/// A 119-bit unsigned integer
pub type u119 = Aint<u128, 119>;
/// A 120-bit unsigned integer
pub type u120 = Aint<u128, 120>;
/// A 121-bit unsigned integer
pub type u121 = Aint<u128, 121>;
/// A 122-bit unsigned integer
pub type u122 = Aint<u128, 122>;
/// A 123-bit unsigned integer
pub type u123 = Aint<u128, 123>;
/// A 124-bit unsigned integer
pub type u124 = Aint<u128, 124>;
/// A 125-bit unsigned integer
pub type u125 = Aint<u128, 125>;
/// A 126-bit unsigned integer
pub type u126 = Aint<u128, 126>;
/// A 127-bit unsigned integer
pub type u127 = Aint<u128, 127>;
/// A 128-bit unsigned integer
pub type u128 = core::primitive::u128;

/// A 1-bit signed integer
pub type i1 = Aint<i8, 1>;
/// A 2-bit signed integer
pub type i2 = Aint<i8, 2>;
/// A 3-bit signed integer
pub type i3 = Aint<i8, 3>;
/// A 4-bit signed integer
pub type i4 = Aint<i8, 4>;
/// A 5-bit signed integer
pub type i5 = Aint<i8, 5>;
/// A 6-bit signed integer
pub type i6 = Aint<i8, 6>;
/// A 7-bit signed integer
pub type i7 = Aint<i8, 7>;
/// A 8-bit signed integer
pub type i8 = core::primitive::i8;
/// A 9-bit signed integer
pub type i9 = Aint<i16, 9>;
/// A 10-bit signed integer
pub type i10 = Aint<i16, 10>;
/// A 11-bit signed integer
pub type i11 = Aint<i16, 11>;
/// A 12-bit signed integer
pub type i12 = Aint<i16, 12>;
/// A 13-bit signed integer
pub type i13 = Aint<i16, 13>;
/// A 14-bit signed integer
pub type i14 = Aint<i16, 14>;
/// A 15-bit signed integer
pub type i15 = Aint<i16, 15>;
/// A 16-bit signed integer
pub type i16 = core::primitive::i16;
/// A 17-bit signed integer
pub type i17 = Aint<i32, 17>;
/// A 18-bit signed integer
pub type i18 = Aint<i32, 18>;
/// A 19-bit signed integer
pub type i19 = Aint<i32, 19>;
/// A 20-bit signed integer
pub type i20 = Aint<i32, 20>;
/// A 21-bit signed integer
pub type i21 = Aint<i32, 21>;
/// A 22-bit signed integer
pub type i22 = Aint<i32, 22>;
/// A 23-bit signed integer
pub type i23 = Aint<i32, 23>;
/// A 24-bit signed integer
pub type i24 = Aint<i32, 24>;
/// A 25-bit signed integer
pub type i25 = Aint<i32, 25>;
/// A 26-bit signed integer
pub type i26 = Aint<i32, 26>;
/// A 27-bit signed integer
pub type i27 = Aint<i32, 27>;
/// A 28-bit signed integer
pub type i28 = Aint<i32, 28>;
/// A 29-bit signed integer
pub type i29 = Aint<i32, 29>;
/// A 30-bit signed integer
pub type i30 = Aint<i32, 30>;
/// A 31-bit signed integer
pub type i31 = Aint<i32, 31>;
/// A 32-bit signed integer
pub type i32 = core::primitive::i32;
/// A 33-bit signed integer
pub type i33 = Aint<i64, 33>;
/// A 34-bit signed integer
pub type i34 = Aint<i64, 34>;
/// A 35-bit signed integer
pub type i35 = Aint<i64, 35>;
/// A 36-bit signed integer
pub type i36 = Aint<i64, 36>;
/// A 37-bit signed integer
pub type i37 = Aint<i64, 37>;
/// A 38-bit signed integer
pub type i38 = Aint<i64, 38>;
/// A 39-bit signed integer
pub type i39 = Aint<i64, 39>;
/// A 40-bit signed integer
pub type i40 = Aint<i64, 40>;
/// A 41-bit signed integer
pub type i41 = Aint<i64, 41>;
/// A 42-bit signed integer
pub type i42 = Aint<i64, 42>;
/// A 43-bit signed integer
pub type i43 = Aint<i64, 43>;
/// A 44-bit signed integer
pub type i44 = Aint<i64, 44>;
/// A 45-bit signed integer
pub type i45 = Aint<i64, 45>;
/// A 46-bit signed integer
pub type i46 = Aint<i64, 46>;
/// A 47-bit signed integer
pub type i47 = Aint<i64, 47>;
/// A 48-bit signed integer
pub type i48 = Aint<i64, 48>;
/// A 49-bit signed integer
pub type i49 = Aint<i64, 49>;
/// A 50-bit signed integer
pub type i50 = Aint<i64, 50>;
/// A 51-bit signed integer
pub type i51 = Aint<i64, 51>;
/// A 52-bit signed integer
pub type i52 = Aint<i64, 52>;
/// A 53-bit signed integer
pub type i53 = Aint<i64, 53>;
/// A 54-bit signed integer
pub type i54 = Aint<i64, 54>;
/// A 55-bit signed integer
pub type i55 = Aint<i64, 55>;
/// A 56-bit signed integer
pub type i56 = Aint<i64, 56>;
/// A 57-bit signed integer
pub type i57 = Aint<i64, 57>;
/// A 58-bit signed integer
pub type i58 = Aint<i64, 58>;
/// A 59-bit signed integer
pub type i59 = Aint<i64, 59>;
/// A 60-bit signed integer
pub type i60 = Aint<i64, 60>;
/// A 61-bit signed integer
pub type i61 = Aint<i64, 61>;
/// A 62-bit signed integer
pub type i62 = Aint<i64, 62>;
/// A 63-bit signed integer
pub type i63 = Aint<i64, 63>;
/// A 64-bit signed integer
pub type i64 = core::primitive::i64;
/// A 65-bit signed integer
pub type i65 = Aint<i128, 65>;
/// A 66-bit signed integer
pub type i66 = Aint<i128, 66>;
/// A 67-bit signed integer
pub type i67 = Aint<i128, 67>;
/// A 68-bit signed integer
pub type i68 = Aint<i128, 68>;
/// A 69-bit signed integer
pub type i69 = Aint<i128, 69>;
/// A 70-bit signed integer
pub type i70 = Aint<i128, 70>;
/// A 71-bit signed integer
pub type i71 = Aint<i128, 71>;
/// A 72-bit signed integer
pub type i72 = Aint<i128, 72>;
/// A 73-bit signed integer
pub type i73 = Aint<i128, 73>;
/// A 74-bit signed integer
pub type i74 = Aint<i128, 74>;
/// A 75-bit signed integer
pub type i75 = Aint<i128, 75>;
/// A 76-bit signed integer
pub type i76 = Aint<i128, 76>;
/// A 77-bit signed integer
pub type i77 = Aint<i128, 77>;
/// A 78-bit signed integer
pub type i78 = Aint<i128, 78>;
/// A 79-bit signed integer
pub type i79 = Aint<i128, 79>;
/// A 80-bit signed integer
pub type i80 = Aint<i128, 80>;
/// A 81-bit signed integer
pub type i81 = Aint<i128, 81>;
/// A 82-bit signed integer
pub type i82 = Aint<i128, 82>;
/// A 83-bit signed integer
pub type i83 = Aint<i128, 83>;
/// A 84-bit signed integer
pub type i84 = Aint<i128, 84>;
/// A 85-bit signed integer
pub type i85 = Aint<i128, 85>;
/// A 86-bit signed integer
pub type i86 = Aint<i128, 86>;
/// A 87-bit signed integer
pub type i87 = Aint<i128, 87>;
/// A 88-bit signed integer
pub type i88 = Aint<i128, 88>;
/// A 89-bit signed integer
pub type i89 = Aint<i128, 89>;
/// A 90-bit signed integer
pub type i90 = Aint<i128, 90>;
/// A 91-bit signed integer
pub type i91 = Aint<i128, 91>;
/// A 92-bit signed integer
pub type i92 = Aint<i128, 92>;
/// A 93-bit signed integer
pub type i93 = Aint<i128, 93>;
/// A 94-bit signed integer
pub type i94 = Aint<i128, 94>;
/// A 95-bit signed integer
pub type i95 = Aint<i128, 95>;
/// A 96-bit signed integer
pub type i96 = Aint<i128, 96>;
/// A 97-bit signed integer
pub type i97 = Aint<i128, 97>;
/// A 98-bit signed integer
pub type i98 = Aint<i128, 98>;
/// A 99-bit signed integer
pub type i99 = Aint<i128, 99>;
/// A 100-bit signed integer
pub type i100 = Aint<i128, 100>;
/// A 101-bit signed integer
pub type i101 = Aint<i128, 101>;
/// A 102-bit signed integer
pub type i102 = Aint<i128, 102>;
/// A 103-bit signed integer
pub type i103 = Aint<i128, 103>;
/// A 104-bit signed integer
pub type i104 = Aint<i128, 104>;
/// A 105-bit signed integer
pub type i105 = Aint<i128, 105>;
/// A 106-bit signed integer
pub type i106 = Aint<i128, 106>;
/// A 107-bit signed integer
pub type i107 = Aint<i128, 107>;
/// A 108-bit signed integer
pub type i108 = Aint<i128, 108>;
/// A 109-bit signed integer
pub type i109 = Aint<i128, 109>;
/// A 110-bit signed integer
pub type i110 = Aint<i128, 110>;
/// A 111-bit signed integer
pub type i111 = Aint<i128, 111>;
/// A 112-bit signed integer
pub type i112 = Aint<i128, 112>;
/// A 113-bit signed integer
pub type i113 = Aint<i128, 113>;
/// A 114-bit signed integer
pub type i114 = Aint<i128, 114>;
/// A 115-bit signed integer
pub type i115 = Aint<i128, 115>;
/// A 116-bit signed integer
pub type i116 = Aint<i128, 116>;
/// A 117-bit signed integer
pub type i117 = Aint<i128, 117>;
/// A 118-bit signed integer
pub type i118 = Aint<i128, 118>;
/// A 119-bit signed integer
pub type i119 = Aint<i128, 119>;
/// A 120-bit signed integer
pub type i120 = Aint<i128, 120>;
/// A 121-bit signed integer
pub type i121 = Aint<i128, 121>;
/// A 122-bit signed integer
pub type i122 = Aint<i128, 122>;
/// A 123-bit signed integer
pub type i123 = Aint<i128, 123>;
/// A 124-bit signed integer
pub type i124 = Aint<i128, 124>;
/// A 125-bit signed integer
pub type i125 = Aint<i128, 125>;
/// A 126-bit signed integer
pub type i126 = Aint<i128, 126>;
/// A 127-bit signed integer
pub type i127 = Aint<i128, 127>;
/// A 128-bit signed integer
pub type i128 = core::primitive::i128;
