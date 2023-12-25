use crate::sealed::Sealed;
use crate::Aint;

#[derive(Clone, Copy)]
struct AintVisitor<R: Sealed, const WIDTH: u32>(core::marker::PhantomData<R>);

impl<'de, R: Sealed, const WIDTH: u32> serde::de::Visitor<'de> for AintVisitor<R, WIDTH> {
    type Value = Aint<R, WIDTH>;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if R::SIGNED {
            write!(f, "a {}-bit signed integer", WIDTH)
        } else {
            write!(f, "a {}-bit unsigned integer", WIDTH)
        }
    }

    fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(v as u64),
                &self,
            )),
        }
    }

    fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(v as u64),
                &self,
            )),
        }
    }

    fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(v as u64),
                &self,
            )),
        }
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(serde::de::Unexpected::Unsigned(v), &self)),
        }
    }

    fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(v as u64),
                &self,
            )),
        }
    }

    fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Signed(v as i64),
                &self,
            )),
        }
    }

    fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Signed(v as i64),
                &self,
            )),
        }
    }

    fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Signed(v as i64),
                &self,
            )),
        }
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(serde::de::Unexpected::Signed(v), &self)),
        }
    }

    fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<Self::Value, E> {
        match v.try_into() {
            Ok(val) => Ok(Aint(val)),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Signed(v as i64),
                &self,
            )),
        }
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<u8, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<u8, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_u8(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<u16, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<u16, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_u16(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<u32, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<u32, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_u32(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<u64, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<u64, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_u64(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<u128, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<u128, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_u128(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<i8, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<i8, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_i8(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<i16, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<i16, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_i16(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<i32, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<i32, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_i32(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<i64, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<i64, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_i64(visitor)
    }
}

impl<'de, const WIDTH: u32> serde::Deserialize<'de> for Aint<i128, WIDTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor: AintVisitor<i128, WIDTH> = AintVisitor(core::marker::PhantomData);
        deserializer.deserialize_i128(visitor)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<u8, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<u16, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<u32, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<u64, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<u128, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u128(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<i8, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<i16, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i16(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<i32, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<i64, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<const WIDTH: u32> serde::Serialize for Aint<i128, WIDTH> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i128(self.0)
    }
}
