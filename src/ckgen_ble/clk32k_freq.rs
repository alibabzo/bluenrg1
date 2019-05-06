#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLK32K_FREQ {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SLOW_FREQR {
    bits: u32,
}
impl SLOW_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:26 - Value equal to 2^33 / SLOW_PERIOD"]
    #[inline]
    pub fn slow_freq(&self) -> SLOW_FREQR {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SLOW_FREQR { bits }
    }
}
