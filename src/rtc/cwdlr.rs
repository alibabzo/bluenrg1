#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CWDLR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CWSECLR {
    bits: u8,
}
impl CWSECLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMINLR {
    bits: u8,
}
impl CWMINLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWHOURLR {
    bits: u8,
}
impl CWHOURLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYWLR {
    bits: u8,
}
impl CWDAYWLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYMLR {
    bits: u8,
}
impl CWDAYMLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMONTHLR {
    bits: u8,
}
impl CWMONTHLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CWSECLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWSECLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWMINLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWMINLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWHOURLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWHOURLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWDAYWLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWDAYWLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWDAYMLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWDAYMLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWMONTHLW<'a> {
    w: &'a mut W,
}
impl<'a> _CWMONTHLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - RTC clockwatch second load value. Clockwatch seconds from 0 to 59 (0x3B). Other values must not be used."]
    #[inline]
    pub fn cwsecl(&self) -> CWSECLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWSECLR { bits }
    }
    #[doc = "Bits 6:11 - RTC clockwatch minute load value. Clockwatch minutes from 0 to 59 (0x3B). Other values must not be used."]
    #[inline]
    pub fn cwminl(&self) -> CWMINLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMINLR { bits }
    }
    #[doc = "Bits 12:16 - RTC clockwatch hour load value. Clockwatch hours from 0 to 23 (0x17). Other values must not be used."]
    #[inline]
    pub fn cwhourl(&self) -> CWHOURLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWHOURLR { bits }
    }
    #[doc = "Bits 17:19 - RTC clockwatch day of week load value. Clockwatch day of week:<ul><li>000b: Must not be used.</li><li>001b: Sunday.</li><li>010b: Monday.</li><li>011b: Tuesday.</li><li>100b: Wednesday.</li><li>101b: Thursday.</li><li>110b: Friday.</li><li>111b: Saturday.</li></ul>"]
    #[inline]
    pub fn cwdaywl(&self) -> CWDAYWLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYWLR { bits }
    }
    #[doc = "Bits 20:24 - RTC clockwatch day of month load value. 1 to 28/29/30 or 31 depending on month:<ul><li>1 to 28: February month, non-leap year.</li><li>1 to 29: February month, leap year.</li><li>1 to 30: April, June, September, November month.</li><li>1 to 31: January, March, May, August, October, December month.</li><li>Other values must not be used.</li></ul>"]
    #[inline]
    pub fn cwdayml(&self) -> CWDAYMLR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYMLR { bits }
    }
    #[doc = "Bits 25:28 - RTC clockwatch month load value:<ul><li>0001b: January.</li><li>...</li><li>1100: December.</li></ul>Other values must not be used."]
    #[inline]
    pub fn cwmonthl(&self) -> CWMONTHLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMONTHLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - RTC clockwatch second load value. Clockwatch seconds from 0 to 59 (0x3B). Other values must not be used."]
    #[inline]
    pub fn cwsecl(&mut self) -> _CWSECLW {
        _CWSECLW { w: self }
    }
    #[doc = "Bits 6:11 - RTC clockwatch minute load value. Clockwatch minutes from 0 to 59 (0x3B). Other values must not be used."]
    #[inline]
    pub fn cwminl(&mut self) -> _CWMINLW {
        _CWMINLW { w: self }
    }
    #[doc = "Bits 12:16 - RTC clockwatch hour load value. Clockwatch hours from 0 to 23 (0x17). Other values must not be used."]
    #[inline]
    pub fn cwhourl(&mut self) -> _CWHOURLW {
        _CWHOURLW { w: self }
    }
    #[doc = "Bits 17:19 - RTC clockwatch day of week load value. Clockwatch day of week:<ul><li>000b: Must not be used.</li><li>001b: Sunday.</li><li>010b: Monday.</li><li>011b: Tuesday.</li><li>100b: Wednesday.</li><li>101b: Thursday.</li><li>110b: Friday.</li><li>111b: Saturday.</li></ul>"]
    #[inline]
    pub fn cwdaywl(&mut self) -> _CWDAYWLW {
        _CWDAYWLW { w: self }
    }
    #[doc = "Bits 20:24 - RTC clockwatch day of month load value. 1 to 28/29/30 or 31 depending on month:<ul><li>1 to 28: February month, non-leap year.</li><li>1 to 29: February month, leap year.</li><li>1 to 30: April, June, September, November month.</li><li>1 to 31: January, March, May, August, October, December month.</li><li>Other values must not be used.</li></ul>"]
    #[inline]
    pub fn cwdayml(&mut self) -> _CWDAYMLW {
        _CWDAYMLW { w: self }
    }
    #[doc = "Bits 25:28 - RTC clockwatch month load value:<ul><li>0001b: January.</li><li>...</li><li>1100: December.</li></ul>Other values must not be used."]
    #[inline]
    pub fn cwmonthl(&mut self) -> _CWMONTHLW {
        _CWMONTHLW { w: self }
    }
}
