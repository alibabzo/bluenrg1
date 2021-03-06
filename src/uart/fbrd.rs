#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FBRD {
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
pub struct DIVFRACR {
    bits: u8,
}
impl DIVFRACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DIVFRACW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVFRACW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:5 - Baud rate fraction. Baud rate integer. The baud rate divisor is calculated as follows:<p>When OVSFACT = 0b in the CR register: Baud rate divisor = (Frequency (UARTCLK)/(16*Baud rate))</p><p>When OVSFACT = 1b in CR register: Baud rate divisor = (Frequency (UARTCLK)/(8*Baud rate))</p>where Frequency (UARTCLK) is the UART reference clock frequency. The baud rate divisor comprises the integer value (DIVINT) and the fractional value (DIVFRAC). The contents of the IBRD and FBRD registers are not updated until transmission or reception of the current character has completed."]
    #[inline]
    pub fn divfrac(&self) -> DIVFRACR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        DIVFRACR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Baud rate fraction. Baud rate integer. The baud rate divisor is calculated as follows:<p>When OVSFACT = 0b in the CR register: Baud rate divisor = (Frequency (UARTCLK)/(16*Baud rate))</p><p>When OVSFACT = 1b in CR register: Baud rate divisor = (Frequency (UARTCLK)/(8*Baud rate))</p>where Frequency (UARTCLK) is the UART reference clock frequency. The baud rate divisor comprises the integer value (DIVINT) and the fractional value (DIVFRAC). The contents of the IBRD and FBRD registers are not updated until transmission or reception of the current character has completed."]
    #[inline]
    pub fn divfrac(&mut self) -> _DIVFRACW {
        _DIVFRACW { w: self }
    }
}
