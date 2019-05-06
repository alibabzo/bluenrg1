#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CWDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CWSECR {
    bits: u8,
}
impl CWSECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMINR {
    bits: u8,
}
impl CWMINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWHOURR {
    bits: u8,
}
impl CWHOURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYWR {
    bits: u8,
}
impl CWDAYWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYMR {
    bits: u8,
}
impl CWDAYMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMONTHR {
    bits: u8,
}
impl CWMONTHR {
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
    #[doc = "Bits 0:5 - RTC clockwatch second value. Clockwatch seconds: 0 to 59 (max 0x3B)."]
    #[inline]
    pub fn cwsec(&self) -> CWSECR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWSECR { bits }
    }
    #[doc = "Bits 6:11 - RTC clockwatch minute value. Clockwatch seconds: 0 to 59 (max 0x3B)."]
    #[inline]
    pub fn cwmin(&self) -> CWMINR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMINR { bits }
    }
    #[doc = "Bits 12:16 - RTC clockwatch hour value. Clockwatch seconds: 0 to 23 (max 0x17)."]
    #[inline]
    pub fn cwhour(&self) -> CWHOURR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWHOURR { bits }
    }
    #[doc = "Bits 17:19 - RTC clockwatch day of week value. Clockwatch day of week:<ul><li>001b: Sunday.</li><li>010b: Monday.</li><li>011b: Tuesday.</li><li>100b: Wednesday.</li><li>101b: Thursday.</li><li>110b: Friday.</li><li>111b: Saturday.</li></ul>"]
    #[inline]
    pub fn cwdayw(&self) -> CWDAYWR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYWR { bits }
    }
    #[doc = "Bits 20:24 - RTC clockwatch day of month value: 1 to 28/29/30 or 31. Range of value to program depends on the month:<ul><li>1 to 28: February month, non-leap year.</li><li>1 to 29: February month, leap year.</li><li>1 to 30: April, June, September, November month.</li><li>1 to 31: January, March, May, August, October, December month.</li></ul>"]
    #[inline]
    pub fn cwdaym(&self) -> CWDAYMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYMR { bits }
    }
    #[doc = "Bits 25:28 - RTC clockwatch month value:<ul><li>0001b: January.</li><li>...</li><li>1100: December.</li></ul>"]
    #[inline]
    pub fn cwmonth(&self) -> CWMONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMONTHR { bits }
    }
}
