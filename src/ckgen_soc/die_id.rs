#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIE_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct REVR {
    bits: u8,
}
impl REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRODUCTR {
    bits: u8,
}
impl PRODUCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Cut revision"]
    #[inline]
    pub fn rev(&self) -> REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVR { bits }
    }
    #[doc = "Bits 4:8 - Cut version"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
    #[doc = "Bits 9:11 - Product"]
    #[inline]
    pub fn product(&self) -> PRODUCTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRODUCTR { bits }
    }
}
