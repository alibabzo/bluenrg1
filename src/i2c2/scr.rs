#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
pub struct SA7R {
    bits: u8,
}
impl SA7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ESA10R {
    bits: u8,
}
impl ESA10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLSUR {
    bits: u16,
}
impl SLSUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SA7W<'a> {
    w: &'a mut W,
}
impl<'a> _SA7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ESA10W<'a> {
    w: &'a mut W,
}
impl<'a> _ESA10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLSUW<'a> {
    w: &'a mut W,
}
impl<'a> _SLSUW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:6 - Slave address 7-bit. SA7 includes the slave address 7-bit or the LSB bits of the slave address 10-bit"]
    #[inline]
    pub fn sa7(&self) -> SA7R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SA7R { bits }
    }
    #[doc = "Bits 7:9 - Extended slave address 10-bit. ESA10 includes the extension (MSB bits) to the SA7 register field in case of slave addressing mode set to 10-bit"]
    #[inline]
    pub fn esa10(&self) -> ESA10R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ESA10R { bits }
    }
    #[doc = "Bits 16:31 - Slave data setup time. SLSU defines the data setup time after SCL clock stretching in terms of i2c_clk cycles. Data setup time is actually equal to SLSU-1 clock cycles. The typical values for i2c_clk of 16 MHz are SLSU = 5 in standard mode and SLSU = 3 in fast modes."]
    #[inline]
    pub fn slsu(&self) -> SLSUR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SLSUR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 983125 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Slave address 7-bit. SA7 includes the slave address 7-bit or the LSB bits of the slave address 10-bit"]
    #[inline]
    pub fn sa7(&mut self) -> _SA7W {
        _SA7W { w: self }
    }
    #[doc = "Bits 7:9 - Extended slave address 10-bit. ESA10 includes the extension (MSB bits) to the SA7 register field in case of slave addressing mode set to 10-bit"]
    #[inline]
    pub fn esa10(&mut self) -> _ESA10W {
        _ESA10W { w: self }
    }
    #[doc = "Bits 16:31 - Slave data setup time. SLSU defines the data setup time after SCL clock stretching in terms of i2c_clk cycles. Data setup time is actually equal to SLSU-1 clock cycles. The typical values for i2c_clk of 16 MHz are SLSU = 5 in standard mode and SLSU = 3 in fast modes."]
    #[inline]
    pub fn slsu(&mut self) -> _SLSUW {
        _SLSUW { w: self }
    }
}
