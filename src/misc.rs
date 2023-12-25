use crate::sealed::Sealed;
use crate::Aint;

impl<R: Sealed, const WIDTH: u32> core::fmt::Display for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::Binary for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::Octal for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::UpperHex for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::LowerHex for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::UpperExp for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperExp::fmt(&self.0, f)
    }
}

impl<R: Sealed, const WIDTH: u32> core::fmt::LowerExp for Aint<R, WIDTH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerExp::fmt(&self.0, f)
    }
}
