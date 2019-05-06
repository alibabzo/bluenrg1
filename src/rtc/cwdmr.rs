#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CWDMR {
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
pub struct CWSECMR {
    bits: u8,
}
impl CWSECMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMINMR {
    bits: u8,
}
impl CWMINMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWHOURMR {
    bits: u8,
}
impl CWHOURMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYWMR {
    bits: u8,
}
impl CWDAYWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWDAYMMR {
    bits: u8,
}
impl CWDAYMMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWMONTHMR {
    bits: u8,
}
impl CWMONTHMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CWSECMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWSECMW<'a> {
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
pub struct _CWMINMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWMINMW<'a> {
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
pub struct _CWHOURMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWHOURMW<'a> {
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
pub struct _CWDAYWMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWDAYWMW<'a> {
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
pub struct _CWDAYMMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWDAYMMW<'a> {
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
pub struct _CWMONTHMW<'a> {
    w: &'a mut W,
}
impl<'a> _CWMONTHMW<'a> {
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
    #[doc = "Bits 0:5 - RTC clockwatch second match value:<ul><li>00 0000 to 11 1011: (0 to 59 or 0x00 to 0x3B) clockwatch seconds.</li><li>11 1100 to 11 1111 - (60 to 63 or 0x3C to 0x3F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwsecm(&self) -> CWSECMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWSECMR { bits }
    }
    #[doc = "Bits 6:11 - RTC clockwatch minute match value:<ul><li>00 0000 to 11 1011: (0 to 59 or 0x00 to 0x3B) clockwatch minutes.</li><li>11 1100 to 11 1111 - (60 to 63 or 0x3C to 0x3F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwminm(&self) -> CWMINMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMINMR { bits }
    }
    #[doc = "Bits 12:16 - RTC clockwatch hour match value:<ul><li>00000b to 10111b: (0 to 23 or 0x00 to 0x17) hour match value.</li><li>11000b to 11111b - (24 to 31 or 0x18 to 0x1F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwhourm(&self) -> CWHOURMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWHOURMR { bits }
    }
    #[doc = "Bits 17:19 - RTC clockwatch day of week match value:<ul><li>000b: day of week is don't care in the comparison. (Default value after PORn).</li><li>001b to 111b: (1 to 7) day of week match value.</li></ul>"]
    #[inline]
    pub fn cwdaywm(&self) -> CWDAYWMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYWMR { bits }
    }
    #[doc = "Bits 20:24 - RTC clockwatch day of month match value:<ul><li>0000b: (month is don't care in the comparison. Default value after PORn).</li><li>1 to 31: day of month match value.</li></ul>"]
    #[inline]
    pub fn cwdaymm(&self) -> CWDAYMMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWDAYMMR { bits }
    }
    #[doc = "Bits 25:28 - RTC clockwatch month match value:<ul><li>0000b: (day of month is don't care in the comparison. Default value after PORn).</li><li>0001b to 1100b: (1 to 12) month match value.</li><li>1101b (13, 0xD) to 1111b (0xF) non-valid data, match never occurs.</li></ul>"]
    #[inline]
    pub fn cwmonthm(&self) -> CWMONTHMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWMONTHMR { bits }
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
    #[doc = "Bits 0:5 - RTC clockwatch second match value:<ul><li>00 0000 to 11 1011: (0 to 59 or 0x00 to 0x3B) clockwatch seconds.</li><li>11 1100 to 11 1111 - (60 to 63 or 0x3C to 0x3F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwsecm(&mut self) -> _CWSECMW {
        _CWSECMW { w: self }
    }
    #[doc = "Bits 6:11 - RTC clockwatch minute match value:<ul><li>00 0000 to 11 1011: (0 to 59 or 0x00 to 0x3B) clockwatch minutes.</li><li>11 1100 to 11 1111 - (60 to 63 or 0x3C to 0x3F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwminm(&mut self) -> _CWMINMW {
        _CWMINMW { w: self }
    }
    #[doc = "Bits 12:16 - RTC clockwatch hour match value:<ul><li>00000b to 10111b: (0 to 23 or 0x00 to 0x17) hour match value.</li><li>11000b to 11111b - (24 to 31 or 0x18 to 0x1F).</li></ul>Non-valid data, match never occurs."]
    #[inline]
    pub fn cwhourm(&mut self) -> _CWHOURMW {
        _CWHOURMW { w: self }
    }
    #[doc = "Bits 17:19 - RTC clockwatch day of week match value:<ul><li>000b: day of week is don't care in the comparison. (Default value after PORn).</li><li>001b to 111b: (1 to 7) day of week match value.</li></ul>"]
    #[inline]
    pub fn cwdaywm(&mut self) -> _CWDAYWMW {
        _CWDAYWMW { w: self }
    }
    #[doc = "Bits 20:24 - RTC clockwatch day of month match value:<ul><li>0000b: (month is don't care in the comparison. Default value after PORn).</li><li>1 to 31: day of month match value.</li></ul>"]
    #[inline]
    pub fn cwdaymm(&mut self) -> _CWDAYMMW {
        _CWDAYMMW { w: self }
    }
    #[doc = "Bits 25:28 - RTC clockwatch month match value:<ul><li>0000b: (day of month is don't care in the comparison. Default value after PORn).</li><li>0001b to 1100b: (1 to 12) month match value.</li><li>1101b (13, 0xD) to 1111b (0xF) non-valid data, match never occurs.</li></ul>"]
    #[inline]
    pub fn cwmonthm(&mut self) -> _CWMONTHMW {
        _CWMONTHMW { w: self }
    }
}
