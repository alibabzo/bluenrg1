#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE1 {
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
#[doc = "Possible values of the field `IO8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO8R {
    #[doc = "GPIO mode"]
    GPIO_MODE,
    #[doc = "serial1 mode"]
    SERIAL1_MODE,
    #[doc = "standalone mode"]
    STANDALONE_MODE,
    #[doc = "BLE mode"]
    BLUE_MODE,
    #[doc = "serial0 mode"]
    SERIAL0_MODE,
    #[doc = "ADC mode for microphone"]
    MICROPHONE_ADC_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IO8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO8R::GPIO_MODE => 0,
            IO8R::SERIAL1_MODE => 1,
            IO8R::STANDALONE_MODE => 2,
            IO8R::BLUE_MODE => 3,
            IO8R::SERIAL0_MODE => 4,
            IO8R::MICROPHONE_ADC_MODE => 5,
            IO8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO8R {
        match value {
            0 => IO8R::GPIO_MODE,
            1 => IO8R::SERIAL1_MODE,
            2 => IO8R::STANDALONE_MODE,
            3 => IO8R::BLUE_MODE,
            4 => IO8R::SERIAL0_MODE,
            5 => IO8R::MICROPHONE_ADC_MODE,
            i => IO8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline]
    pub fn is_gpio_mode(&self) -> bool {
        *self == IO8R::GPIO_MODE
    }
    #[doc = "Checks if the value of the field is `SERIAL1_MODE`"]
    #[inline]
    pub fn is_serial1_mode(&self) -> bool {
        *self == IO8R::SERIAL1_MODE
    }
    #[doc = "Checks if the value of the field is `STANDALONE_MODE`"]
    #[inline]
    pub fn is_standalone_mode(&self) -> bool {
        *self == IO8R::STANDALONE_MODE
    }
    #[doc = "Checks if the value of the field is `BLUE_MODE`"]
    #[inline]
    pub fn is_blue_mode(&self) -> bool {
        *self == IO8R::BLUE_MODE
    }
    #[doc = "Checks if the value of the field is `SERIAL0_MODE`"]
    #[inline]
    pub fn is_serial0_mode(&self) -> bool {
        *self == IO8R::SERIAL0_MODE
    }
    #[doc = "Checks if the value of the field is `MICROPHONE_ADC_MODE`"]
    #[inline]
    pub fn is_microphone_adc_mode(&self) -> bool {
        *self == IO8R::MICROPHONE_ADC_MODE
    }
}
#[doc = r" Value of the field"]
pub struct IO9R {
    bits: u8,
}
impl IO9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO10R {
    bits: u8,
}
impl IO10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO11R {
    bits: u8,
}
impl IO11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO12R {
    bits: u8,
}
impl IO12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO13R {
    bits: u8,
}
impl IO13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO14R {
    bits: u8,
}
impl IO14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `IO8`"]
pub enum IO8W {
    #[doc = "GPIO mode"]
    GPIO_MODE,
    #[doc = "serial1 mode"]
    SERIAL1_MODE,
    #[doc = "standalone mode"]
    STANDALONE_MODE,
    #[doc = "BLE mode"]
    BLUE_MODE,
    #[doc = "serial0 mode"]
    SERIAL0_MODE,
    #[doc = "ADC mode for microphone"]
    MICROPHONE_ADC_MODE,
}
impl IO8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO8W::GPIO_MODE => 0,
            IO8W::SERIAL1_MODE => 1,
            IO8W::STANDALONE_MODE => 2,
            IO8W::BLUE_MODE => 3,
            IO8W::SERIAL0_MODE => 4,
            IO8W::MICROPHONE_ADC_MODE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO8W<'a> {
    w: &'a mut W,
}
impl<'a> _IO8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO mode"]
    #[inline]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(IO8W::GPIO_MODE)
    }
    #[doc = "serial1 mode"]
    #[inline]
    pub fn serial1_mode(self) -> &'a mut W {
        self.variant(IO8W::SERIAL1_MODE)
    }
    #[doc = "standalone mode"]
    #[inline]
    pub fn standalone_mode(self) -> &'a mut W {
        self.variant(IO8W::STANDALONE_MODE)
    }
    #[doc = "BLE mode"]
    #[inline]
    pub fn blue_mode(self) -> &'a mut W {
        self.variant(IO8W::BLUE_MODE)
    }
    #[doc = "serial0 mode"]
    #[inline]
    pub fn serial0_mode(self) -> &'a mut W {
        self.variant(IO8W::SERIAL0_MODE)
    }
    #[doc = "ADC mode for microphone"]
    #[inline]
    pub fn microphone_adc_mode(self) -> &'a mut W {
        self.variant(IO8W::MICROPHONE_ADC_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO9W<'a> {
    w: &'a mut W,
}
impl<'a> _IO9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO10W<'a> {
    w: &'a mut W,
}
impl<'a> _IO10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO11W<'a> {
    w: &'a mut W,
}
impl<'a> _IO11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO12W<'a> {
    w: &'a mut W,
}
impl<'a> _IO12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO13W<'a> {
    w: &'a mut W,
}
impl<'a> _IO13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO14W<'a> {
    w: &'a mut W,
}
impl<'a> _IO14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - IO8 mode"]
    #[inline]
    pub fn io8(&self) -> IO8R {
        IO8R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - IO9 mode"]
    #[inline]
    pub fn io9(&self) -> IO9R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO9R { bits }
    }
    #[doc = "Bits 8:10 - IO10 mode"]
    #[inline]
    pub fn io10(&self) -> IO10R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO10R { bits }
    }
    #[doc = "Bits 12:14 - IO11 mode"]
    #[inline]
    pub fn io11(&self) -> IO11R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO11R { bits }
    }
    #[doc = "Bits 16:18 - IO12 mode"]
    #[inline]
    pub fn io12(&self) -> IO12R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO12R { bits }
    }
    #[doc = "Bits 20:22 - IO13 mode"]
    #[inline]
    pub fn io13(&self) -> IO13R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO13R { bits }
    }
    #[doc = "Bits 24:26 - IO14 mode"]
    #[inline]
    pub fn io14(&self) -> IO14R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO14R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 272 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - IO8 mode"]
    #[inline]
    pub fn io8(&mut self) -> _IO8W {
        _IO8W { w: self }
    }
    #[doc = "Bits 4:6 - IO9 mode"]
    #[inline]
    pub fn io9(&mut self) -> _IO9W {
        _IO9W { w: self }
    }
    #[doc = "Bits 8:10 - IO10 mode"]
    #[inline]
    pub fn io10(&mut self) -> _IO10W {
        _IO10W { w: self }
    }
    #[doc = "Bits 12:14 - IO11 mode"]
    #[inline]
    pub fn io11(&mut self) -> _IO11W {
        _IO11W { w: self }
    }
    #[doc = "Bits 16:18 - IO12 mode"]
    #[inline]
    pub fn io12(&mut self) -> _IO12W {
        _IO12W { w: self }
    }
    #[doc = "Bits 20:22 - IO13 mode"]
    #[inline]
    pub fn io13(&mut self) -> _IO13W {
        _IO13W { w: self }
    }
    #[doc = "Bits 24:26 - IO14 mode"]
    #[inline]
    pub fn io14(&mut self) -> _IO14W {
        _IO14W { w: self }
    }
}
