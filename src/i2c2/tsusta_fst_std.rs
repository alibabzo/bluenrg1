#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSUSTA_FST_STD {
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
pub struct TSUSTA_STDR {
    bits: u16,
}
impl TSUSTA_STDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSUSTA_FSTR {
    bits: u16,
}
impl TSUSTA_FSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TSUSTA_STDW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUSTA_STDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSUSTA_FSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUSTA_FSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
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
    #[doc = "Bits 0:8 - Setup time start condition value for standard mode. After a non-stop on the SCL line the decimeter loads the value of TSUSTA_STD according to standard mode. Once the counter is expired, the start condition is generated."]
    #[inline]
    pub fn tsusta_std(&self) -> TSUSTA_STDR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSUSTA_STDR { bits }
    }
    #[doc = "Bits 16:24 - Setup time start condition value for fast mode. After a non-stop on the SCL line the decimeter loads the value of TSUSTA_FST according to fast mode. Once the counter is expired the start condition is generated."]
    #[inline]
    pub fn tsusta_fst(&self) -> TSUSTA_FSTR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSUSTA_FSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1900770 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - Setup time start condition value for standard mode. After a non-stop on the SCL line the decimeter loads the value of TSUSTA_STD according to standard mode. Once the counter is expired, the start condition is generated."]
    #[inline]
    pub fn tsusta_std(&mut self) -> _TSUSTA_STDW {
        _TSUSTA_STDW { w: self }
    }
    #[doc = "Bits 16:24 - Setup time start condition value for fast mode. After a non-stop on the SCL line the decimeter loads the value of TSUSTA_FST according to fast mode. Once the counter is expired the start condition is generated."]
    #[inline]
    pub fn tsusta_fst(&mut self) -> _TSUSTA_FSTW {
        _TSUSTA_FSTW { w: self }
    }
}
