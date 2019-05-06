#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "DMA channel disable"]
    DISABLE,
    #[doc = "DMA channel enable"]
    ENABLE,
}
impl ENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENR::DISABLE => false,
            ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::DISABLE,
            true => ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl TCIER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TCIER::DISABLE => false,
            TCIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLE,
            true => TCIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TCIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TCIER::ENABLE
    }
}
#[doc = "Possible values of the field `HTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIER {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl HTIER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HTIER::DISABLE => false,
            HTIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIER {
        match value {
            false => HTIER::DISABLE,
            true => HTIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HTIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HTIER::ENABLE
    }
}
#[doc = "Possible values of the field `TEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIER {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl TEIER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TEIER::DISABLE => false,
            TEIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIER {
        match value {
            false => TEIER::DISABLE,
            true => TEIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TEIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TEIER::ENABLE
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Read from peripheral"]
    FROM_PERIPHERAL,
    #[doc = "Read from memory"]
    FROM_MEMORY,
}
impl DIRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIRR::FROM_PERIPHERAL => false,
            DIRR::FROM_MEMORY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::FROM_PERIPHERAL,
            true => DIRR::FROM_MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `FROM_PERIPHERAL`"]
    #[inline]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIRR::FROM_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `FROM_MEMORY`"]
    #[inline]
    pub fn is_from_memory(&self) -> bool {
        *self == DIRR::FROM_MEMORY
    }
}
#[doc = "Possible values of the field `CIRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRCR {
    #[doc = "Circular mode disable"]
    DISABLE,
    #[doc = "Circular mode enable"]
    ENABLE,
}
impl CIRCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CIRCR::DISABLE => false,
            CIRCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIRCR {
        match value {
            false => CIRCR::DISABLE,
            true => CIRCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CIRCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CIRCR::ENABLE
    }
}
#[doc = "Possible values of the field `PINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCR {
    #[doc = "Peripheral increment disable"]
    DISABLE,
    #[doc = "Peripheral increment enable"]
    ENABLE,
}
impl PINCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PINCR::DISABLE => false,
            PINCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINCR {
        match value {
            false => PINCR::DISABLE,
            true => PINCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PINCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PINCR::ENABLE
    }
}
#[doc = "Possible values of the field `MINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINCR {
    #[doc = "Memory increment disable"]
    DISABLE,
    #[doc = "Memory increment enable"]
    ENABLE,
}
impl MINCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MINCR::DISABLE => false,
            MINCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MINCR {
        match value {
            false => MINCR::DISABLE,
            true => MINCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MINCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MINCR::ENABLE
    }
}
#[doc = "Possible values of the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZER {
    #[doc = "Size 8 bits"]
    SIZE8BIT,
    #[doc = "Size 16 bits"]
    SIZE16BIT,
    #[doc = "Size 32 bits"]
    SIZE32BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSIZER::SIZE8BIT => 0,
            PSIZER::SIZE16BIT => 1,
            PSIZER::SIZE32BIT => 2,
            PSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSIZER {
        match value {
            0 => PSIZER::SIZE8BIT,
            1 => PSIZER::SIZE16BIT,
            2 => PSIZER::SIZE32BIT,
            i => PSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE8BIT`"]
    #[inline]
    pub fn is_size8bit(&self) -> bool {
        *self == PSIZER::SIZE8BIT
    }
    #[doc = "Checks if the value of the field is `SIZE16BIT`"]
    #[inline]
    pub fn is_size16bit(&self) -> bool {
        *self == PSIZER::SIZE16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE32BIT`"]
    #[inline]
    pub fn is_size32bit(&self) -> bool {
        *self == PSIZER::SIZE32BIT
    }
}
#[doc = "Possible values of the field `MSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIZER {
    #[doc = "Size 8 bits"]
    SIZE8BIT,
    #[doc = "Size 16 bits"]
    SIZE16BIT,
    #[doc = "Size 32 bits"]
    SIZE32BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSIZER::SIZE8BIT => 0,
            MSIZER::SIZE16BIT => 1,
            MSIZER::SIZE32BIT => 2,
            MSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSIZER {
        match value {
            0 => MSIZER::SIZE8BIT,
            1 => MSIZER::SIZE16BIT,
            2 => MSIZER::SIZE32BIT,
            i => MSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE8BIT`"]
    #[inline]
    pub fn is_size8bit(&self) -> bool {
        *self == MSIZER::SIZE8BIT
    }
    #[doc = "Checks if the value of the field is `SIZE16BIT`"]
    #[inline]
    pub fn is_size16bit(&self) -> bool {
        *self == MSIZER::SIZE16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE32BIT`"]
    #[inline]
    pub fn is_size32bit(&self) -> bool {
        *self == MSIZER::SIZE32BIT
    }
}
#[doc = "Possible values of the field `PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLR {
    #[doc = "Low priority"]
    LOW,
    #[doc = "Medium priority"]
    MEDIUM,
    #[doc = "High priority"]
    HIGH,
    #[doc = "Very high priority"]
    VERY_HIGH,
}
impl PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLR::LOW => 0,
            PLR::MEDIUM => 1,
            PLR::HIGH => 2,
            PLR::VERY_HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLR {
        match value {
            0 => PLR::LOW,
            1 => PLR::MEDIUM,
            2 => PLR::HIGH,
            3 => PLR::VERY_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PLR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == PLR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PLR::HIGH
    }
    #[doc = "Checks if the value of the field is `VERY_HIGH`"]
    #[inline]
    pub fn is_very_high(&self) -> bool {
        *self == PLR::VERY_HIGH
    }
}
#[doc = r" Value of the field"]
pub struct MEM2MEMR {
    bits: bool,
}
impl MEM2MEMR {
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
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u32,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "DMA channel disable"]
    DISABLE,
    #[doc = "DMA channel enable"]
    ENABLE,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::DISABLE => false,
            ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA channel disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENW::DISABLE)
    }
    #[doc = "DMA channel enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENW::ENABLE)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLE => false,
            TCIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt source disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TCIEW::DISABLE)
    }
    #[doc = "Interrupt source enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TCIEW::ENABLE)
    }
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HTIE`"]
pub enum HTIEW {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl HTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HTIEW::DISABLE => false,
            HTIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HTIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt source disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HTIEW::DISABLE)
    }
    #[doc = "Interrupt source enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HTIEW::ENABLE)
    }
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEIE`"]
pub enum TEIEW {
    #[doc = "Interrupt source disable"]
    DISABLE,
    #[doc = "Interrupt source enable"]
    ENABLE,
}
impl TEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEIEW::DISABLE => false,
            TEIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt source disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TEIEW::DISABLE)
    }
    #[doc = "Interrupt source enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TEIEW::ENABLE)
    }
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Read from peripheral"]
    FROM_PERIPHERAL,
    #[doc = "Read from memory"]
    FROM_MEMORY,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::FROM_PERIPHERAL => false,
            DIRW::FROM_MEMORY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read from peripheral"]
    #[inline]
    pub fn from_peripheral(self) -> &'a mut W {
        self.variant(DIRW::FROM_PERIPHERAL)
    }
    #[doc = "Read from memory"]
    #[inline]
    pub fn from_memory(self) -> &'a mut W {
        self.variant(DIRW::FROM_MEMORY)
    }
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CIRC`"]
pub enum CIRCW {
    #[doc = "Circular mode disable"]
    DISABLE,
    #[doc = "Circular mode enable"]
    ENABLE,
}
impl CIRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIRCW::DISABLE => false,
            CIRCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Circular mode disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CIRCW::DISABLE)
    }
    #[doc = "Circular mode enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CIRCW::ENABLE)
    }
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINC`"]
pub enum PINCW {
    #[doc = "Peripheral increment disable"]
    DISABLE,
    #[doc = "Peripheral increment enable"]
    ENABLE,
}
impl PINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINCW::DISABLE => false,
            PINCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral increment disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINCW::DISABLE)
    }
    #[doc = "Peripheral increment enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINCW::ENABLE)
    }
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MINC`"]
pub enum MINCW {
    #[doc = "Memory increment disable"]
    DISABLE,
    #[doc = "Memory increment enable"]
    ENABLE,
}
impl MINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MINCW::DISABLE => false,
            MINCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MINCW<'a> {
    w: &'a mut W,
}
impl<'a> _MINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory increment disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MINCW::DISABLE)
    }
    #[doc = "Memory increment enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MINCW::ENABLE)
    }
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSIZE`"]
pub enum PSIZEW {
    #[doc = "Size 8 bits"]
    SIZE8BIT,
    #[doc = "Size 16 bits"]
    SIZE16BIT,
    #[doc = "Size 32 bits"]
    SIZE32BIT,
}
impl PSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSIZEW::SIZE8BIT => 0,
            PSIZEW::SIZE16BIT => 1,
            PSIZEW::SIZE32BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Size 8 bits"]
    #[inline]
    pub fn size8bit(self) -> &'a mut W {
        self.variant(PSIZEW::SIZE8BIT)
    }
    #[doc = "Size 16 bits"]
    #[inline]
    pub fn size16bit(self) -> &'a mut W {
        self.variant(PSIZEW::SIZE16BIT)
    }
    #[doc = "Size 32 bits"]
    #[inline]
    pub fn size32bit(self) -> &'a mut W {
        self.variant(PSIZEW::SIZE32BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSIZE`"]
pub enum MSIZEW {
    #[doc = "Size 8 bits"]
    SIZE8BIT,
    #[doc = "Size 16 bits"]
    SIZE16BIT,
    #[doc = "Size 32 bits"]
    SIZE32BIT,
}
impl MSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSIZEW::SIZE8BIT => 0,
            MSIZEW::SIZE16BIT => 1,
            MSIZEW::SIZE32BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Size 8 bits"]
    #[inline]
    pub fn size8bit(self) -> &'a mut W {
        self.variant(MSIZEW::SIZE8BIT)
    }
    #[doc = "Size 16 bits"]
    #[inline]
    pub fn size16bit(self) -> &'a mut W {
        self.variant(MSIZEW::SIZE16BIT)
    }
    #[doc = "Size 32 bits"]
    #[inline]
    pub fn size32bit(self) -> &'a mut W {
        self.variant(MSIZEW::SIZE32BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PL`"]
pub enum PLW {
    #[doc = "Low priority"]
    LOW,
    #[doc = "Medium priority"]
    MEDIUM,
    #[doc = "High priority"]
    HIGH,
    #[doc = "Very high priority"]
    VERY_HIGH,
}
impl PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLW::LOW => 0,
            PLW::MEDIUM => 1,
            PLW::HIGH => 2,
            PLW::VERY_HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low priority"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PLW::LOW)
    }
    #[doc = "Medium priority"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(PLW::MEDIUM)
    }
    #[doc = "High priority"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PLW::HIGH)
    }
    #[doc = "Very high priority"]
    #[inline]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PLW::VERY_HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEM2MEMW<'a> {
    w: &'a mut W,
}
impl<'a> _MEM2MEMW<'a> {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - DMA channel enable.<ul><li>0: DMA channel disabled.</li><li>1: DMA channel enabled.</li></ul>"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable.<ul><li>0: TC interrupt disabled.</li><li>1: TC interrupt enabled.</li></ul>"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Half transfer interrupt enable.<ul><li>0: HT interrupt disabled.</li><li>1: HT interrupt enabled.</li></ul>"]
    #[inline]
    pub fn htie(&self) -> HTIER {
        HTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transfer error interrupt enable.<ul><li>0: TE interrupt disabled.</li><li>1: TE interrupt enabled.</li></ul>"]
    #[inline]
    pub fn teie(&self) -> TEIER {
        TEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data transfer direction.<ul><li>0: Read from peripheral.</li><li>1: Read from memory.</li></ul>"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Circular mode.<ul><li>0: Circular mode disabled.</li><li>1: Circular mode enabled.</li></ul>"]
    #[inline]
    pub fn circ(&self) -> CIRCR {
        CIRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Peripheral increment mode.<ul><li>0: Peripheral increment disabled.</li><li>1: Peripheral increment enabled.</li></ul>"]
    #[inline]
    pub fn pinc(&self) -> PINCR {
        PINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Memory increment mode.<ul><li>0: Memory increment disabled.</li><li>1: Memory increment enabled.</li></ul>"]
    #[inline]
    pub fn minc(&self) -> MINCR {
        MINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Peripheral size.<ul><li>00b: Size 8 bits.</li><li>01b: Size 16 bits.</li><li>10b: Size 32 bits.</li></ul>"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Memory size.<ul><li>00b: Size 8 bits.</li><li>01b: Size 16 bits.</li><li>10b: Size 32 bits.</li></ul>"]
    #[inline]
    pub fn msize(&self) -> MSIZER {
        MSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Channel priority level.<ul><li>00b: Low priority.</li><li>01b: Medium priority.</li><li>10b: High priority.</li><li>11b: Very high priority.</li></ul>"]
    #[inline]
    pub fn pl(&self) -> PLR {
        PLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Memory to memory mode.<ul><li>0: Memory to memory mode disabled.</li><li>0: Memory to memory mode enabled.</li></ul>"]
    #[inline]
    pub fn mem2mem(&self) -> MEM2MEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEM2MEMR { bits }
    }
    #[doc = "Bits 15:31 - Reserved"]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED1R { bits }
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
    #[doc = "Bit 0 - DMA channel enable.<ul><li>0: DMA channel disabled.</li><li>1: DMA channel enabled.</li></ul>"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable.<ul><li>0: TC interrupt disabled.</li><li>1: TC interrupt enabled.</li></ul>"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 2 - Half transfer interrupt enable.<ul><li>0: HT interrupt disabled.</li><li>1: HT interrupt enabled.</li></ul>"]
    #[inline]
    pub fn htie(&mut self) -> _HTIEW {
        _HTIEW { w: self }
    }
    #[doc = "Bit 3 - Transfer error interrupt enable.<ul><li>0: TE interrupt disabled.</li><li>1: TE interrupt enabled.</li></ul>"]
    #[inline]
    pub fn teie(&mut self) -> _TEIEW {
        _TEIEW { w: self }
    }
    #[doc = "Bit 4 - Data transfer direction.<ul><li>0: Read from peripheral.</li><li>1: Read from memory.</li></ul>"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 5 - Circular mode.<ul><li>0: Circular mode disabled.</li><li>1: Circular mode enabled.</li></ul>"]
    #[inline]
    pub fn circ(&mut self) -> _CIRCW {
        _CIRCW { w: self }
    }
    #[doc = "Bit 6 - Peripheral increment mode.<ul><li>0: Peripheral increment disabled.</li><li>1: Peripheral increment enabled.</li></ul>"]
    #[inline]
    pub fn pinc(&mut self) -> _PINCW {
        _PINCW { w: self }
    }
    #[doc = "Bit 7 - Memory increment mode.<ul><li>0: Memory increment disabled.</li><li>1: Memory increment enabled.</li></ul>"]
    #[inline]
    pub fn minc(&mut self) -> _MINCW {
        _MINCW { w: self }
    }
    #[doc = "Bits 8:9 - Peripheral size.<ul><li>00b: Size 8 bits.</li><li>01b: Size 16 bits.</li><li>10b: Size 32 bits.</li></ul>"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bits 10:11 - Memory size.<ul><li>00b: Size 8 bits.</li><li>01b: Size 16 bits.</li><li>10b: Size 32 bits.</li></ul>"]
    #[inline]
    pub fn msize(&mut self) -> _MSIZEW {
        _MSIZEW { w: self }
    }
    #[doc = "Bits 12:13 - Channel priority level.<ul><li>00b: Low priority.</li><li>01b: Medium priority.</li><li>10b: High priority.</li><li>11b: Very high priority.</li></ul>"]
    #[inline]
    pub fn pl(&mut self) -> _PLW {
        _PLW { w: self }
    }
    #[doc = "Bit 14 - Memory to memory mode.<ul><li>0: Memory to memory mode disabled.</li><li>0: Memory to memory mode enabled.</li></ul>"]
    #[inline]
    pub fn mem2mem(&mut self) -> _MEM2MEMW {
        _MEM2MEMW { w: self }
    }
}
