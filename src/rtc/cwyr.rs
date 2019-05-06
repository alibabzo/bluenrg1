#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::CWYR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CWYEARR {
    bits: u16,
}
impl CWYEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:13 - RTC clockwatch year value. Clockwatch year, in BCD format is from 0 to 3999."]
    #[inline]
    pub fn cwyear(&self) -> CWYEARR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        CWYEARR { bits }
    }
}
