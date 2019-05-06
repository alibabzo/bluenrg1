#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LCRH_TX {
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
#[doc = "Possible values of the field `BRK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKR {
    #[doc = "Normal transmission"]
    TX_NORMAL,
    #[doc = "Break condition transmission"]
    TX_BREAK_CONDITION,
}
impl BRKR {
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
            BRKR::TX_NORMAL => false,
            BRKR::TX_BREAK_CONDITION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRKR {
        match value {
            false => BRKR::TX_NORMAL,
            true => BRKR::TX_BREAK_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `TX_NORMAL`"]
    #[inline]
    pub fn is_tx_normal(&self) -> bool {
        *self == BRKR::TX_NORMAL
    }
    #[doc = "Checks if the value of the field is `TX_BREAK_CONDITION`"]
    #[inline]
    pub fn is_tx_break_condition(&self) -> bool {
        *self == BRKR::TX_BREAK_CONDITION
    }
}
#[doc = "Possible values of the field `PEN_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_TXR {
    #[doc = "Parity Enable"]
    PARITY_ENABLE,
    #[doc = "Parity Disable"]
    PARITY_DISABLE,
}
impl PEN_TXR {
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
            PEN_TXR::PARITY_ENABLE => true,
            PEN_TXR::PARITY_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEN_TXR {
        match value {
            true => PEN_TXR::PARITY_ENABLE,
            false => PEN_TXR::PARITY_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_ENABLE`"]
    #[inline]
    pub fn is_parity_enable(&self) -> bool {
        *self == PEN_TXR::PARITY_ENABLE
    }
    #[doc = "Checks if the value of the field is `PARITY_DISABLE`"]
    #[inline]
    pub fn is_parity_disable(&self) -> bool {
        *self == PEN_TXR::PARITY_DISABLE
    }
}
#[doc = "Possible values of the field `EPS_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_TXR {
    #[doc = "Odd parity generation and checking is performed during transmission, which check for an odd number of 1s in data and parity bits"]
    ODD,
    #[doc = "Even parity generation and checking is performed during transmission, which check for an even number of 1s in data and parity bits"]
    EVEN,
}
impl EPS_TXR {
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
            EPS_TXR::ODD => false,
            EPS_TXR::EVEN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPS_TXR {
        match value {
            false => EPS_TXR::ODD,
            true => EPS_TXR::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == EPS_TXR::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == EPS_TXR::EVEN
    }
}
#[doc = "Possible values of the field `STP2_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STP2_TXR {
    #[doc = "1 stop bit received"]
    STOP_BIT1,
    #[doc = "2 stop bits received"]
    STOP_BITS2,
}
impl STP2_TXR {
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
            STP2_TXR::STOP_BIT1 => false,
            STP2_TXR::STOP_BITS2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STP2_TXR {
        match value {
            false => STP2_TXR::STOP_BIT1,
            true => STP2_TXR::STOP_BITS2,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_BIT1`"]
    #[inline]
    pub fn is_stop_bit1(&self) -> bool {
        *self == STP2_TXR::STOP_BIT1
    }
    #[doc = "Checks if the value of the field is `STOP_BITS2`"]
    #[inline]
    pub fn is_stop_bits2(&self) -> bool {
        *self == STP2_TXR::STOP_BITS2
    }
}
#[doc = "Possible values of the field `FEN_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEN_TXR {
    #[doc = "TX FIFO is disabled"]
    TXFIFO_DISABLED,
    #[doc = "TX FIFO is enabled"]
    TXFIFO_ENABLED,
}
impl FEN_TXR {
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
            FEN_TXR::TXFIFO_DISABLED => false,
            FEN_TXR::TXFIFO_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEN_TXR {
        match value {
            false => FEN_TXR::TXFIFO_DISABLED,
            true => FEN_TXR::TXFIFO_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_DISABLED`"]
    #[inline]
    pub fn is_txfifo_disabled(&self) -> bool {
        *self == FEN_TXR::TXFIFO_DISABLED
    }
    #[doc = "Checks if the value of the field is `TXFIFO_ENABLED`"]
    #[inline]
    pub fn is_txfifo_enabled(&self) -> bool {
        *self == FEN_TXR::TXFIFO_ENABLED
    }
}
#[doc = "Possible values of the field `WLEN_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEN_TXR {
    #[doc = "5 bits"]
    BIT5,
    #[doc = "6 bits"]
    BIT6,
    #[doc = "7 bits"]
    BIT7,
    #[doc = "8 bits"]
    BIT8,
}
impl WLEN_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLEN_TXR::BIT5 => 0,
            WLEN_TXR::BIT6 => 1,
            WLEN_TXR::BIT7 => 2,
            WLEN_TXR::BIT8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLEN_TXR {
        match value {
            0 => WLEN_TXR::BIT5,
            1 => WLEN_TXR::BIT6,
            2 => WLEN_TXR::BIT7,
            3 => WLEN_TXR::BIT8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT5`"]
    #[inline]
    pub fn is_bit5(&self) -> bool {
        *self == WLEN_TXR::BIT5
    }
    #[doc = "Checks if the value of the field is `BIT6`"]
    #[inline]
    pub fn is_bit6(&self) -> bool {
        *self == WLEN_TXR::BIT6
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline]
    pub fn is_bit7(&self) -> bool {
        *self == WLEN_TXR::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline]
    pub fn is_bit8(&self) -> bool {
        *self == WLEN_TXR::BIT8
    }
}
#[doc = "Possible values of the field `SPS_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPS_TXR {
    #[doc = "stick parity disable"]
    STICK_PARITY_DISABLE,
    #[doc = "stick parity enable"]
    STICK_PARITY_ENABLE,
}
impl SPS_TXR {
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
            SPS_TXR::STICK_PARITY_DISABLE => false,
            SPS_TXR::STICK_PARITY_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPS_TXR {
        match value {
            false => SPS_TXR::STICK_PARITY_DISABLE,
            true => SPS_TXR::STICK_PARITY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `STICK_PARITY_DISABLE`"]
    #[inline]
    pub fn is_stick_parity_disable(&self) -> bool {
        *self == SPS_TXR::STICK_PARITY_DISABLE
    }
    #[doc = "Checks if the value of the field is `STICK_PARITY_ENABLE`"]
    #[inline]
    pub fn is_stick_parity_enable(&self) -> bool {
        *self == SPS_TXR::STICK_PARITY_ENABLE
    }
}
#[doc = "Values that can be written to the field `BRK`"]
pub enum BRKW {
    #[doc = "Normal transmission"]
    TX_NORMAL,
    #[doc = "Break condition transmission"]
    TX_BREAK_CONDITION,
}
impl BRKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRKW::TX_NORMAL => false,
            BRKW::TX_BREAK_CONDITION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRKW<'a> {
    w: &'a mut W,
}
impl<'a> _BRKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transmission"]
    #[inline]
    pub fn tx_normal(self) -> &'a mut W {
        self.variant(BRKW::TX_NORMAL)
    }
    #[doc = "Break condition transmission"]
    #[inline]
    pub fn tx_break_condition(self) -> &'a mut W {
        self.variant(BRKW::TX_BREAK_CONDITION)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEN_TX`"]
pub enum PEN_TXW {
    #[doc = "Parity Enable"]
    PARITY_ENABLE,
    #[doc = "Parity Disable"]
    PARITY_DISABLE,
}
impl PEN_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEN_TXW::PARITY_ENABLE => true,
            PEN_TXW::PARITY_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEN_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _PEN_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEN_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity Enable"]
    #[inline]
    pub fn parity_enable(self) -> &'a mut W {
        self.variant(PEN_TXW::PARITY_ENABLE)
    }
    #[doc = "Parity Disable"]
    #[inline]
    pub fn parity_disable(self) -> &'a mut W {
        self.variant(PEN_TXW::PARITY_DISABLE)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPS_TX`"]
pub enum EPS_TXW {
    #[doc = "Odd parity generation and checking is performed during transmission, which check for an odd number of 1s in data and parity bits"]
    ODD,
    #[doc = "Even parity generation and checking is performed during transmission, which check for an even number of 1s in data and parity bits"]
    EVEN,
}
impl EPS_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPS_TXW::ODD => false,
            EPS_TXW::EVEN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPS_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _EPS_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPS_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Odd parity generation and checking is performed during transmission, which check for an odd number of 1s in data and parity bits"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_TXW::ODD)
    }
    #[doc = "Even parity generation and checking is performed during transmission, which check for an even number of 1s in data and parity bits"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_TXW::EVEN)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STP2_TX`"]
pub enum STP2_TXW {
    #[doc = "1 stop bit received"]
    STOP_BIT1,
    #[doc = "2 stop bits received"]
    STOP_BITS2,
}
impl STP2_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STP2_TXW::STOP_BIT1 => false,
            STP2_TXW::STOP_BITS2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STP2_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _STP2_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STP2_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit received"]
    #[inline]
    pub fn stop_bit1(self) -> &'a mut W {
        self.variant(STP2_TXW::STOP_BIT1)
    }
    #[doc = "2 stop bits received"]
    #[inline]
    pub fn stop_bits2(self) -> &'a mut W {
        self.variant(STP2_TXW::STOP_BITS2)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEN_TX`"]
pub enum FEN_TXW {
    #[doc = "TX FIFO is disabled"]
    TXFIFO_DISABLED,
    #[doc = "TX FIFO is enabled"]
    TXFIFO_ENABLED,
}
impl FEN_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEN_TXW::TXFIFO_DISABLED => false,
            FEN_TXW::TXFIFO_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEN_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _FEN_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEN_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX FIFO is disabled"]
    #[inline]
    pub fn txfifo_disabled(self) -> &'a mut W {
        self.variant(FEN_TXW::TXFIFO_DISABLED)
    }
    #[doc = "TX FIFO is enabled"]
    #[inline]
    pub fn txfifo_enabled(self) -> &'a mut W {
        self.variant(FEN_TXW::TXFIFO_ENABLED)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WLEN_TX`"]
pub enum WLEN_TXW {
    #[doc = "5 bits"]
    BIT5,
    #[doc = "6 bits"]
    BIT6,
    #[doc = "7 bits"]
    BIT7,
    #[doc = "8 bits"]
    BIT8,
}
impl WLEN_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLEN_TXW::BIT5 => 0,
            WLEN_TXW::BIT6 => 1,
            WLEN_TXW::BIT7 => 2,
            WLEN_TXW::BIT8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLEN_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _WLEN_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLEN_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5 bits"]
    #[inline]
    pub fn bit5(self) -> &'a mut W {
        self.variant(WLEN_TXW::BIT5)
    }
    #[doc = "6 bits"]
    #[inline]
    pub fn bit6(self) -> &'a mut W {
        self.variant(WLEN_TXW::BIT6)
    }
    #[doc = "7 bits"]
    #[inline]
    pub fn bit7(self) -> &'a mut W {
        self.variant(WLEN_TXW::BIT7)
    }
    #[doc = "8 bits"]
    #[inline]
    pub fn bit8(self) -> &'a mut W {
        self.variant(WLEN_TXW::BIT8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPS_TX`"]
pub enum SPS_TXW {
    #[doc = "stick parity disable"]
    STICK_PARITY_DISABLE,
    #[doc = "stick parity enable"]
    STICK_PARITY_ENABLE,
}
impl SPS_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPS_TXW::STICK_PARITY_DISABLE => false,
            SPS_TXW::STICK_PARITY_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPS_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _SPS_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPS_TXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "stick parity disable"]
    #[inline]
    pub fn stick_parity_disable(self) -> &'a mut W {
        self.variant(SPS_TXW::STICK_PARITY_DISABLE)
    }
    #[doc = "stick parity enable"]
    #[inline]
    pub fn stick_parity_enable(self) -> &'a mut W {
        self.variant(SPS_TXW::STICK_PARITY_ENABLE)
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
    #[doc = "Bit 0 - Send break. This bit allows a continuous low-level to be forced on TX output, after completion of the current character. This bit must be asserted for at least one complete frame transmission time in order to generate a break condition. The transmit FIFO contents remain unaffected during a break condition.<ul><li>0: Normal transmission.</li><li>1: Break condition transmission.</li></ul>"]
    #[inline]
    pub fn brk(&self) -> BRKR {
        BRKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - TX parity enable:<ul><li>0: Parity disabled.</li><li>1: Parity Enable.</li></ul>"]
    #[inline]
    pub fn pen_tx(&self) -> PEN_TXR {
        PEN_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - TX even parity select. This bit selects the parity generation, when the parity is enabled (PEN_TX =1b). This bit has no effect when parity is disabled (PEN_TX = 0b).<ul><li>0: Odd parity generation and checking is performed during transmission, which check for an odd number of 1s in data and parity bits.</li><li>1: Even parity generation and checking is performed during transmission, which check for an even number of 1s in data and parity bits.</li></ul>"]
    #[inline]
    pub fn eps_tx(&self) -> EPS_TXR {
        EPS_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - TX two stop bits select. This bit enables the check for two stop bits being received:<ul><li>0: 1 stop bit received.</li><li>1: 2 stop bits received.</li></ul>"]
    #[inline]
    pub fn stp2_tx(&self) -> STP2_TXR {
        STP2_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - TX Enable FIFO. This bit enables/disables the transmit TX FIFO buffer:<ul><li>0: TX FIFO is disabled (character mode), i.e. the TX FIFO becomes a 1-byte deep holding register.</li><li>1: TX FIFO is enabled.</li></ul>"]
    #[inline]
    pub fn fen_tx(&self) -> FEN_TXR {
        FEN_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 5:6 - TX word length. This bit field indicates the number of data bits transmitted in a frame as follows:<ul><li>00b: 5 bits.</li><li>01b: 6 bits.</li><li>10b: 7 bits.</li><li>11b: 8 bits.</li></ul>"]
    #[inline]
    pub fn wlen_tx(&self) -> WLEN_TXR {
        WLEN_TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - TX Stick parity check:<ul><li>0: stick parity disable.</li><li>1: when PEN_TX = 1b (parity enabled) and EPS_TX = 1b (even parity), the parity is transmitted as a 0. When PEN_TX = 1b and EPS_TX = 0b (odd parity), the parity bit is transmitted as a 1.</li></ul>"]
    #[inline]
    pub fn sps_tx(&self) -> SPS_TXR {
        SPS_TXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
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
    #[doc = "Bit 0 - Send break. This bit allows a continuous low-level to be forced on TX output, after completion of the current character. This bit must be asserted for at least one complete frame transmission time in order to generate a break condition. The transmit FIFO contents remain unaffected during a break condition.<ul><li>0: Normal transmission.</li><li>1: Break condition transmission.</li></ul>"]
    #[inline]
    pub fn brk(&mut self) -> _BRKW {
        _BRKW { w: self }
    }
    #[doc = "Bit 1 - TX parity enable:<ul><li>0: Parity disabled.</li><li>1: Parity Enable.</li></ul>"]
    #[inline]
    pub fn pen_tx(&mut self) -> _PEN_TXW {
        _PEN_TXW { w: self }
    }
    #[doc = "Bit 2 - TX even parity select. This bit selects the parity generation, when the parity is enabled (PEN_TX =1b). This bit has no effect when parity is disabled (PEN_TX = 0b).<ul><li>0: Odd parity generation and checking is performed during transmission, which check for an odd number of 1s in data and parity bits.</li><li>1: Even parity generation and checking is performed during transmission, which check for an even number of 1s in data and parity bits.</li></ul>"]
    #[inline]
    pub fn eps_tx(&mut self) -> _EPS_TXW {
        _EPS_TXW { w: self }
    }
    #[doc = "Bit 3 - TX two stop bits select. This bit enables the check for two stop bits being received:<ul><li>0: 1 stop bit received.</li><li>1: 2 stop bits received.</li></ul>"]
    #[inline]
    pub fn stp2_tx(&mut self) -> _STP2_TXW {
        _STP2_TXW { w: self }
    }
    #[doc = "Bit 4 - TX Enable FIFO. This bit enables/disables the transmit TX FIFO buffer:<ul><li>0: TX FIFO is disabled (character mode), i.e. the TX FIFO becomes a 1-byte deep holding register.</li><li>1: TX FIFO is enabled.</li></ul>"]
    #[inline]
    pub fn fen_tx(&mut self) -> _FEN_TXW {
        _FEN_TXW { w: self }
    }
    #[doc = "Bits 5:6 - TX word length. This bit field indicates the number of data bits transmitted in a frame as follows:<ul><li>00b: 5 bits.</li><li>01b: 6 bits.</li><li>10b: 7 bits.</li><li>11b: 8 bits.</li></ul>"]
    #[inline]
    pub fn wlen_tx(&mut self) -> _WLEN_TXW {
        _WLEN_TXW { w: self }
    }
    #[doc = "Bit 7 - TX Stick parity check:<ul><li>0: stick parity disable.</li><li>1: when PEN_TX = 1b (parity enabled) and EPS_TX = 1b (even parity), the parity is transmitted as a 0. When PEN_TX = 1b and EPS_TX = 0b (odd parity), the parity bit is transmitted as a 1.</li></ul>"]
    #[inline]
    pub fn sps_tx(&mut self) -> _SPS_TXW {
        _SPS_TXW { w: self }
    }
}
