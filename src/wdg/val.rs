#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VAL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct WDTVALR {
    bits: u32,
}
impl WDTVALR {
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
    #[doc = "Bits 0:31 - Watchdog load value. When read, returns the current value of the decrementing watchdog counter. A write has no effect."]
    #[inline]
    pub fn wdtval(&self) -> WDTVALR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        WDTVALR { bits }
    }
}
