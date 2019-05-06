#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTCR {
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
pub struct CKDIVR {
    bits: u16,
}
impl CKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CKDELR {
    bits: u16,
}
impl CKDELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWENR {
    bits: bool,
}
impl CWENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _CKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CKDELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CWENW<'a> {
    w: &'a mut W,
}
impl<'a> _CWENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:14 - Clock divider factor. This value plus one represents the integer part of the CLK32K clock divider used to produce the reference 1 Hz clock.<ul><li>0x000: CLK1HZ clock is similar to CLK32K for RTC timer and stopped for RTC clockwatch.</li><li>0x0001: 2 CLK32K clock cycles per CLK1HZ clock cycle.</li><li>...</li><li>0x7FFF: 32768 CLK32K clock cycles per CLK1HZ clock cycle (default value after PORn reset).</li><li>...</li><li>0xFFFF: CLK32K clock cycles per CLK1HZ clock cycle.</li></ul>Writing to this bit-field will be disregarded if CWEN = 1. A read returns the value of the CKDIV bit-field."]
    #[inline]
    pub fn ckdiv(&self) -> CKDIVR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CKDIVR { bits }
    }
    #[doc = "Bits 16:25 - Trim delete count. This value represents the number of CLK32K clock pulses to delete every 1023 CLK32K clock cycles to get a better reference 1 Hz clock for incrementing the RTC counter.<ul><li>0x000: No CLK32K clock cycle is deleted every 1023 CLK1HZ clock cycles (default value after PORn reset).</li><li>0x001: 1 CLK32K clock cycle is deleted every 1023 CLK1HZ clock cycles.</li><li>...</li><li>0x3FF: 1023 CLK32K clock cycles are deleted every 1023 CLK1HZ clock cycles.</li></ul>Writing to this bit-field will be disregarded if CWEN = 1. A read returns the value of the CKDEL bit-field."]
    #[inline]
    pub fn ckdel(&self) -> CKDELR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CKDELR { bits }
    }
    #[doc = "Bit 26 - Clockwatch enable bit. When set to 1, the clockwatch is enabled. Once it is enabled, any write to this register has no effect until a power-on reset. A read returns the value of the CWEN bit value."]
    #[inline]
    pub fn cwen(&self) -> CWENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CWENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32767 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:14 - Clock divider factor. This value plus one represents the integer part of the CLK32K clock divider used to produce the reference 1 Hz clock.<ul><li>0x000: CLK1HZ clock is similar to CLK32K for RTC timer and stopped for RTC clockwatch.</li><li>0x0001: 2 CLK32K clock cycles per CLK1HZ clock cycle.</li><li>...</li><li>0x7FFF: 32768 CLK32K clock cycles per CLK1HZ clock cycle (default value after PORn reset).</li><li>...</li><li>0xFFFF: CLK32K clock cycles per CLK1HZ clock cycle.</li></ul>Writing to this bit-field will be disregarded if CWEN = 1. A read returns the value of the CKDIV bit-field."]
    #[inline]
    pub fn ckdiv(&mut self) -> _CKDIVW {
        _CKDIVW { w: self }
    }
    #[doc = "Bits 16:25 - Trim delete count. This value represents the number of CLK32K clock pulses to delete every 1023 CLK32K clock cycles to get a better reference 1 Hz clock for incrementing the RTC counter.<ul><li>0x000: No CLK32K clock cycle is deleted every 1023 CLK1HZ clock cycles (default value after PORn reset).</li><li>0x001: 1 CLK32K clock cycle is deleted every 1023 CLK1HZ clock cycles.</li><li>...</li><li>0x3FF: 1023 CLK32K clock cycles are deleted every 1023 CLK1HZ clock cycles.</li></ul>Writing to this bit-field will be disregarded if CWEN = 1. A read returns the value of the CKDEL bit-field."]
    #[inline]
    pub fn ckdel(&mut self) -> _CKDELW {
        _CKDELW { w: self }
    }
    #[doc = "Bit 26 - Clockwatch enable bit. When set to 1, the clockwatch is enabled. Once it is enabled, any write to this register has no effect until a power-on reset. A read returns the value of the CWEN bit value."]
    #[inline]
    pub fn cwen(&mut self) -> _CWENW {
        _CWENW { w: self }
    }
}
