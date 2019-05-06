#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MFTX {
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
#[doc = "Possible values of the field `MFT1_TIMER_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFT1_TIMER_AR {
    #[doc = "IO0"]
    IO0,
    #[doc = "IO1"]
    IO1,
    #[doc = "IO2"]
    IO2,
    #[doc = "IO3"]
    IO3,
    #[doc = "IO4"]
    IO4,
    #[doc = "IO5"]
    IO5,
    #[doc = "IO6"]
    IO6,
    #[doc = "IO7"]
    IO7,
    #[doc = "IO8"]
    IO8,
    #[doc = "IO9"]
    IO9,
    #[doc = "IO10"]
    IO10,
    #[doc = "IO11"]
    IO11,
    #[doc = "IO12"]
    IO12,
    #[doc = "IO13"]
    IO13,
    #[doc = "IO14"]
    IO14,
    #[doc = "IO15"]
    IO15,
    #[doc = "IO16"]
    IO16,
    #[doc = "IO17"]
    IO17,
    #[doc = "IO18"]
    IO18,
    #[doc = "IO19"]
    IO19,
    #[doc = "IO20"]
    IO20,
    #[doc = "IO21"]
    IO21,
    #[doc = "IO22"]
    IO22,
    #[doc = "IO23"]
    IO23,
    #[doc = "IO24"]
    IO24,
    #[doc = "IO25"]
    IO25,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MFT1_TIMER_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MFT1_TIMER_AR::IO0 => 0,
            MFT1_TIMER_AR::IO1 => 1,
            MFT1_TIMER_AR::IO2 => 2,
            MFT1_TIMER_AR::IO3 => 3,
            MFT1_TIMER_AR::IO4 => 4,
            MFT1_TIMER_AR::IO5 => 5,
            MFT1_TIMER_AR::IO6 => 6,
            MFT1_TIMER_AR::IO7 => 7,
            MFT1_TIMER_AR::IO8 => 8,
            MFT1_TIMER_AR::IO9 => 9,
            MFT1_TIMER_AR::IO10 => 10,
            MFT1_TIMER_AR::IO11 => 11,
            MFT1_TIMER_AR::IO12 => 12,
            MFT1_TIMER_AR::IO13 => 13,
            MFT1_TIMER_AR::IO14 => 14,
            MFT1_TIMER_AR::IO15 => 15,
            MFT1_TIMER_AR::IO16 => 16,
            MFT1_TIMER_AR::IO17 => 17,
            MFT1_TIMER_AR::IO18 => 18,
            MFT1_TIMER_AR::IO19 => 19,
            MFT1_TIMER_AR::IO20 => 20,
            MFT1_TIMER_AR::IO21 => 21,
            MFT1_TIMER_AR::IO22 => 22,
            MFT1_TIMER_AR::IO23 => 23,
            MFT1_TIMER_AR::IO24 => 24,
            MFT1_TIMER_AR::IO25 => 25,
            MFT1_TIMER_AR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MFT1_TIMER_AR {
        match value {
            0 => MFT1_TIMER_AR::IO0,
            1 => MFT1_TIMER_AR::IO1,
            2 => MFT1_TIMER_AR::IO2,
            3 => MFT1_TIMER_AR::IO3,
            4 => MFT1_TIMER_AR::IO4,
            5 => MFT1_TIMER_AR::IO5,
            6 => MFT1_TIMER_AR::IO6,
            7 => MFT1_TIMER_AR::IO7,
            8 => MFT1_TIMER_AR::IO8,
            9 => MFT1_TIMER_AR::IO9,
            10 => MFT1_TIMER_AR::IO10,
            11 => MFT1_TIMER_AR::IO11,
            12 => MFT1_TIMER_AR::IO12,
            13 => MFT1_TIMER_AR::IO13,
            14 => MFT1_TIMER_AR::IO14,
            15 => MFT1_TIMER_AR::IO15,
            16 => MFT1_TIMER_AR::IO16,
            17 => MFT1_TIMER_AR::IO17,
            18 => MFT1_TIMER_AR::IO18,
            19 => MFT1_TIMER_AR::IO19,
            20 => MFT1_TIMER_AR::IO20,
            21 => MFT1_TIMER_AR::IO21,
            22 => MFT1_TIMER_AR::IO22,
            23 => MFT1_TIMER_AR::IO23,
            24 => MFT1_TIMER_AR::IO24,
            25 => MFT1_TIMER_AR::IO25,
            i => MFT1_TIMER_AR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IO0`"]
    #[inline]
    pub fn is_io0(&self) -> bool {
        *self == MFT1_TIMER_AR::IO0
    }
    #[doc = "Checks if the value of the field is `IO1`"]
    #[inline]
    pub fn is_io1(&self) -> bool {
        *self == MFT1_TIMER_AR::IO1
    }
    #[doc = "Checks if the value of the field is `IO2`"]
    #[inline]
    pub fn is_io2(&self) -> bool {
        *self == MFT1_TIMER_AR::IO2
    }
    #[doc = "Checks if the value of the field is `IO3`"]
    #[inline]
    pub fn is_io3(&self) -> bool {
        *self == MFT1_TIMER_AR::IO3
    }
    #[doc = "Checks if the value of the field is `IO4`"]
    #[inline]
    pub fn is_io4(&self) -> bool {
        *self == MFT1_TIMER_AR::IO4
    }
    #[doc = "Checks if the value of the field is `IO5`"]
    #[inline]
    pub fn is_io5(&self) -> bool {
        *self == MFT1_TIMER_AR::IO5
    }
    #[doc = "Checks if the value of the field is `IO6`"]
    #[inline]
    pub fn is_io6(&self) -> bool {
        *self == MFT1_TIMER_AR::IO6
    }
    #[doc = "Checks if the value of the field is `IO7`"]
    #[inline]
    pub fn is_io7(&self) -> bool {
        *self == MFT1_TIMER_AR::IO7
    }
    #[doc = "Checks if the value of the field is `IO8`"]
    #[inline]
    pub fn is_io8(&self) -> bool {
        *self == MFT1_TIMER_AR::IO8
    }
    #[doc = "Checks if the value of the field is `IO9`"]
    #[inline]
    pub fn is_io9(&self) -> bool {
        *self == MFT1_TIMER_AR::IO9
    }
    #[doc = "Checks if the value of the field is `IO10`"]
    #[inline]
    pub fn is_io10(&self) -> bool {
        *self == MFT1_TIMER_AR::IO10
    }
    #[doc = "Checks if the value of the field is `IO11`"]
    #[inline]
    pub fn is_io11(&self) -> bool {
        *self == MFT1_TIMER_AR::IO11
    }
    #[doc = "Checks if the value of the field is `IO12`"]
    #[inline]
    pub fn is_io12(&self) -> bool {
        *self == MFT1_TIMER_AR::IO12
    }
    #[doc = "Checks if the value of the field is `IO13`"]
    #[inline]
    pub fn is_io13(&self) -> bool {
        *self == MFT1_TIMER_AR::IO13
    }
    #[doc = "Checks if the value of the field is `IO14`"]
    #[inline]
    pub fn is_io14(&self) -> bool {
        *self == MFT1_TIMER_AR::IO14
    }
    #[doc = "Checks if the value of the field is `IO15`"]
    #[inline]
    pub fn is_io15(&self) -> bool {
        *self == MFT1_TIMER_AR::IO15
    }
    #[doc = "Checks if the value of the field is `IO16`"]
    #[inline]
    pub fn is_io16(&self) -> bool {
        *self == MFT1_TIMER_AR::IO16
    }
    #[doc = "Checks if the value of the field is `IO17`"]
    #[inline]
    pub fn is_io17(&self) -> bool {
        *self == MFT1_TIMER_AR::IO17
    }
    #[doc = "Checks if the value of the field is `IO18`"]
    #[inline]
    pub fn is_io18(&self) -> bool {
        *self == MFT1_TIMER_AR::IO18
    }
    #[doc = "Checks if the value of the field is `IO19`"]
    #[inline]
    pub fn is_io19(&self) -> bool {
        *self == MFT1_TIMER_AR::IO19
    }
    #[doc = "Checks if the value of the field is `IO20`"]
    #[inline]
    pub fn is_io20(&self) -> bool {
        *self == MFT1_TIMER_AR::IO20
    }
    #[doc = "Checks if the value of the field is `IO21`"]
    #[inline]
    pub fn is_io21(&self) -> bool {
        *self == MFT1_TIMER_AR::IO21
    }
    #[doc = "Checks if the value of the field is `IO22`"]
    #[inline]
    pub fn is_io22(&self) -> bool {
        *self == MFT1_TIMER_AR::IO22
    }
    #[doc = "Checks if the value of the field is `IO23`"]
    #[inline]
    pub fn is_io23(&self) -> bool {
        *self == MFT1_TIMER_AR::IO23
    }
    #[doc = "Checks if the value of the field is `IO24`"]
    #[inline]
    pub fn is_io24(&self) -> bool {
        *self == MFT1_TIMER_AR::IO24
    }
    #[doc = "Checks if the value of the field is `IO25`"]
    #[inline]
    pub fn is_io25(&self) -> bool {
        *self == MFT1_TIMER_AR::IO25
    }
}
#[doc = r" Value of the field"]
pub struct MFT1_TIMER_BR {
    bits: u8,
}
impl MFT1_TIMER_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MFT2_TIMER_AR {
    bits: u8,
}
impl MFT2_TIMER_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MFT2_TIMER_BR {
    bits: u8,
}
impl MFT2_TIMER_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MFT1_TIMER_A`"]
pub enum MFT1_TIMER_AW {
    #[doc = "IO0"]
    IO0,
    #[doc = "IO1"]
    IO1,
    #[doc = "IO2"]
    IO2,
    #[doc = "IO3"]
    IO3,
    #[doc = "IO4"]
    IO4,
    #[doc = "IO5"]
    IO5,
    #[doc = "IO6"]
    IO6,
    #[doc = "IO7"]
    IO7,
    #[doc = "IO8"]
    IO8,
    #[doc = "IO9"]
    IO9,
    #[doc = "IO10"]
    IO10,
    #[doc = "IO11"]
    IO11,
    #[doc = "IO12"]
    IO12,
    #[doc = "IO13"]
    IO13,
    #[doc = "IO14"]
    IO14,
    #[doc = "IO15"]
    IO15,
    #[doc = "IO16"]
    IO16,
    #[doc = "IO17"]
    IO17,
    #[doc = "IO18"]
    IO18,
    #[doc = "IO19"]
    IO19,
    #[doc = "IO20"]
    IO20,
    #[doc = "IO21"]
    IO21,
    #[doc = "IO22"]
    IO22,
    #[doc = "IO23"]
    IO23,
    #[doc = "IO24"]
    IO24,
    #[doc = "IO25"]
    IO25,
}
impl MFT1_TIMER_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MFT1_TIMER_AW::IO0 => 0,
            MFT1_TIMER_AW::IO1 => 1,
            MFT1_TIMER_AW::IO2 => 2,
            MFT1_TIMER_AW::IO3 => 3,
            MFT1_TIMER_AW::IO4 => 4,
            MFT1_TIMER_AW::IO5 => 5,
            MFT1_TIMER_AW::IO6 => 6,
            MFT1_TIMER_AW::IO7 => 7,
            MFT1_TIMER_AW::IO8 => 8,
            MFT1_TIMER_AW::IO9 => 9,
            MFT1_TIMER_AW::IO10 => 10,
            MFT1_TIMER_AW::IO11 => 11,
            MFT1_TIMER_AW::IO12 => 12,
            MFT1_TIMER_AW::IO13 => 13,
            MFT1_TIMER_AW::IO14 => 14,
            MFT1_TIMER_AW::IO15 => 15,
            MFT1_TIMER_AW::IO16 => 16,
            MFT1_TIMER_AW::IO17 => 17,
            MFT1_TIMER_AW::IO18 => 18,
            MFT1_TIMER_AW::IO19 => 19,
            MFT1_TIMER_AW::IO20 => 20,
            MFT1_TIMER_AW::IO21 => 21,
            MFT1_TIMER_AW::IO22 => 22,
            MFT1_TIMER_AW::IO23 => 23,
            MFT1_TIMER_AW::IO24 => 24,
            MFT1_TIMER_AW::IO25 => 25,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MFT1_TIMER_AW<'a> {
    w: &'a mut W,
}
impl<'a> _MFT1_TIMER_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MFT1_TIMER_AW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IO0"]
    #[inline]
    pub fn io0(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO0)
    }
    #[doc = "IO1"]
    #[inline]
    pub fn io1(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO1)
    }
    #[doc = "IO2"]
    #[inline]
    pub fn io2(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO2)
    }
    #[doc = "IO3"]
    #[inline]
    pub fn io3(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO3)
    }
    #[doc = "IO4"]
    #[inline]
    pub fn io4(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO4)
    }
    #[doc = "IO5"]
    #[inline]
    pub fn io5(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO5)
    }
    #[doc = "IO6"]
    #[inline]
    pub fn io6(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO6)
    }
    #[doc = "IO7"]
    #[inline]
    pub fn io7(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO7)
    }
    #[doc = "IO8"]
    #[inline]
    pub fn io8(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO8)
    }
    #[doc = "IO9"]
    #[inline]
    pub fn io9(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO9)
    }
    #[doc = "IO10"]
    #[inline]
    pub fn io10(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO10)
    }
    #[doc = "IO11"]
    #[inline]
    pub fn io11(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO11)
    }
    #[doc = "IO12"]
    #[inline]
    pub fn io12(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO12)
    }
    #[doc = "IO13"]
    #[inline]
    pub fn io13(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO13)
    }
    #[doc = "IO14"]
    #[inline]
    pub fn io14(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO14)
    }
    #[doc = "IO15"]
    #[inline]
    pub fn io15(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO15)
    }
    #[doc = "IO16"]
    #[inline]
    pub fn io16(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO16)
    }
    #[doc = "IO17"]
    #[inline]
    pub fn io17(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO17)
    }
    #[doc = "IO18"]
    #[inline]
    pub fn io18(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO18)
    }
    #[doc = "IO19"]
    #[inline]
    pub fn io19(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO19)
    }
    #[doc = "IO20"]
    #[inline]
    pub fn io20(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO20)
    }
    #[doc = "IO21"]
    #[inline]
    pub fn io21(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO21)
    }
    #[doc = "IO22"]
    #[inline]
    pub fn io22(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO22)
    }
    #[doc = "IO23"]
    #[inline]
    pub fn io23(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO23)
    }
    #[doc = "IO24"]
    #[inline]
    pub fn io24(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO24)
    }
    #[doc = "IO25"]
    #[inline]
    pub fn io25(self) -> &'a mut W {
        self.variant(MFT1_TIMER_AW::IO25)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MFT1_TIMER_BW<'a> {
    w: &'a mut W,
}
impl<'a> _MFT1_TIMER_BW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MFT2_TIMER_AW<'a> {
    w: &'a mut W,
}
impl<'a> _MFT2_TIMER_AW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MFT2_TIMER_BW<'a> {
    w: &'a mut W,
}
impl<'a> _MFT2_TIMER_BW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - MFT1 timer A.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft1_timer_a(&self) -> MFT1_TIMER_AR {
        MFT1_TIMER_AR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - MFT1 timer B.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft1_timer_b(&self) -> MFT1_TIMER_BR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MFT1_TIMER_BR { bits }
    }
    #[doc = "Bits 16:23 - MFT2 timer A.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft2_timer_a(&self) -> MFT2_TIMER_AR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MFT2_TIMER_AR { bits }
    }
    #[doc = "Bits 24:31 - MFT2 timer B.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft2_timer_b(&self) -> MFT2_TIMER_BR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MFT2_TIMER_BR { bits }
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
    #[doc = "Bits 0:7 - MFT1 timer A.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft1_timer_a(&mut self) -> _MFT1_TIMER_AW {
        _MFT1_TIMER_AW { w: self }
    }
    #[doc = "Bits 8:15 - MFT1 timer B.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft1_timer_b(&mut self) -> _MFT1_TIMER_BW {
        _MFT1_TIMER_BW { w: self }
    }
    #[doc = "Bits 16:23 - MFT2 timer A.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft2_timer_a(&mut self) -> _MFT2_TIMER_AW {
        _MFT2_TIMER_AW { w: self }
    }
    #[doc = "Bits 24:31 - MFT2 timer B.<ul><li>0x00: IO0.</li><li>0x01: IO1</li><li>0x02: IO2</li><li>...</li><li>0x0E: IO14</li></ul>"]
    #[inline]
    pub fn mft2_timer_b(&mut self) -> _MFT2_TIMER_BW {
        _MFT2_TIMER_BW { w: self }
    }
}
