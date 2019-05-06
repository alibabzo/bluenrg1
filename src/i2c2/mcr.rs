#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPR {
    #[doc = "Indicates a master write operation"]
    MASTER_WRITE,
    #[doc = "Indicates a master read operation"]
    MASTER_READ,
}
impl OPR {
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
            OPR::MASTER_WRITE => false,
            OPR::MASTER_READ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPR {
        match value {
            false => OPR::MASTER_WRITE,
            true => OPR::MASTER_READ,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_WRITE`"]
    #[inline]
    pub fn is_master_write(&self) -> bool {
        *self == OPR::MASTER_WRITE
    }
    #[doc = "Checks if the value of the field is `MASTER_READ`"]
    #[inline]
    pub fn is_master_read(&self) -> bool {
        *self == OPR::MASTER_READ
    }
}
#[doc = r" Value of the field"]
pub struct A7R {
    bits: u8,
}
impl A7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EA10R {
    bits: u8,
}
impl EA10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SBR {
    bits: bool,
}
impl SBR {
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
#[doc = "Possible values of the field `AM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMR {
    #[doc = "The transaction is initiated by a general call command"]
    GENERAL_CALL,
    #[doc = "The transaction is initiated by the 7-bit address included in the A7 field"]
    BIT7_ADDRESS,
    #[doc = "The transaction is initiated by the 10-bit address included in the EA10 and A7 fields"]
    BIT10_ADDRESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMR::GENERAL_CALL => 0,
            AMR::BIT7_ADDRESS => 1,
            AMR::BIT10_ADDRESS => 2,
            AMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMR {
        match value {
            0 => AMR::GENERAL_CALL,
            1 => AMR::BIT7_ADDRESS,
            2 => AMR::BIT10_ADDRESS,
            i => AMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL`"]
    #[inline]
    pub fn is_general_call(&self) -> bool {
        *self == AMR::GENERAL_CALL
    }
    #[doc = "Checks if the value of the field is `BIT7_ADDRESS`"]
    #[inline]
    pub fn is_bit7_address(&self) -> bool {
        *self == AMR::BIT7_ADDRESS
    }
    #[doc = "Checks if the value of the field is `BIT10_ADDRESS`"]
    #[inline]
    pub fn is_bit10_address(&self) -> bool {
        *self == AMR::BIT10_ADDRESS
    }
}
#[doc = r" Value of the field"]
pub struct PR {
    bits: bool,
}
impl PR {
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
pub struct LENGTHR {
    bits: u16,
}
impl LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OP`"]
pub enum OPW {
    #[doc = "Indicates a master write operation"]
    MASTER_WRITE,
    #[doc = "Indicates a master read operation"]
    MASTER_READ,
}
impl OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPW::MASTER_WRITE => false,
            OPW::MASTER_READ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPW<'a> {
    w: &'a mut W,
}
impl<'a> _OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Indicates a master write operation"]
    #[inline]
    pub fn master_write(self) -> &'a mut W {
        self.variant(OPW::MASTER_WRITE)
    }
    #[doc = "Indicates a master read operation"]
    #[inline]
    pub fn master_read(self) -> &'a mut W {
        self.variant(OPW::MASTER_READ)
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
#[doc = r" Proxy"]
pub struct _A7W<'a> {
    w: &'a mut W,
}
impl<'a> _A7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EA10W<'a> {
    w: &'a mut W,
}
impl<'a> _EA10W<'a> {
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
pub struct _SBW<'a> {
    w: &'a mut W,
}
impl<'a> _SBW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AM`"]
pub enum AMW {
    #[doc = "The transaction is initiated by a general call command"]
    GENERAL_CALL,
    #[doc = "The transaction is initiated by the 7-bit address included in the A7 field"]
    BIT7_ADDRESS,
    #[doc = "The transaction is initiated by the 10-bit address included in the EA10 and A7 fields"]
    BIT10_ADDRESS,
}
impl AMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMW::GENERAL_CALL => 0,
            AMW::BIT7_ADDRESS => 1,
            AMW::BIT10_ADDRESS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMW<'a> {
    w: &'a mut W,
}
impl<'a> _AMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The transaction is initiated by a general call command"]
    #[inline]
    pub fn general_call(self) -> &'a mut W {
        self.variant(AMW::GENERAL_CALL)
    }
    #[doc = "The transaction is initiated by the 7-bit address included in the A7 field"]
    #[inline]
    pub fn bit7_address(self) -> &'a mut W {
        self.variant(AMW::BIT7_ADDRESS)
    }
    #[doc = "The transaction is initiated by the 10-bit address included in the EA10 and A7 fields"]
    #[inline]
    pub fn bit10_address(self) -> &'a mut W {
        self.variant(AMW::BIT10_ADDRESS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PW<'a> {
    w: &'a mut W,
}
impl<'a> _PW<'a> {
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
#[doc = r" Proxy"]
pub struct _LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Operation<ul><li>0: Indicates a master write operation.</li><li>1: Indicates a master read operation.</li></ul>"]
    #[inline]
    pub fn op(&self) -> OPR {
        OPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:7 - Address. Includes the 7-bit address or the LSB bits of the10-bit address used to initiate the current transaction"]
    #[inline]
    pub fn a7(&self) -> A7R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        A7R { bits }
    }
    #[doc = "Bits 8:10 - Extended address. Includes the extension (MSB bits) of the field A7 used to initiate the current transaction"]
    #[inline]
    pub fn ea10(&self) -> EA10R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EA10R { bits }
    }
    #[doc = "Bit 11 - Start byte:<ul><li>0: The start byte procedure is not applied to the current transaction.</li><li>1: The start byte procedure is prefixed to the current transaction.</li></ul>"]
    #[inline]
    pub fn sb(&self) -> SBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SBR { bits }
    }
    #[doc = "Bits 12:13 - Address type:<ul><li>00b: The transaction is initiated by a general call command. In this case the fields OP, A7, EA10 are \"don't care\".</li><li>01b: The transaction is initiated by the 7-bit address included in the A7 field.</li><li>10b: The transaction is initiated by the 10-bit address included in the EA10 and A7 fields.</li></ul>"]
    #[inline]
    pub fn am(&self) -> AMR {
        AMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Stop condition:<ul><li>0: The current transaction is not terminated by a STOP condition. A repeated START condition is generated on the next operation which is required to avoid to stall the I2C line.</li><li>1: The current transaction is terminated by a STOP condition.</li></ul>"]
    #[inline]
    pub fn p(&self) -> PR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PR { bits }
    }
    #[doc = "Bits 15:25 - Transaction length. Defines the length, in terms of the number of bytes to be transmitted (MW) or received (MR). In case of write operation, the payload is stored in the Tx FIFO. A transaction can be larger than the Tx FIFO size. In case of read operation the length refers to the number of bytes to be received before generating a not-acknowledge response. A transaction can be larger than the Rx FIFO size. The I2C clock line is stretched low until the data in Rx FIFO are consumed."]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LENGTHR { bits }
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
    #[doc = "Bit 0 - Operation<ul><li>0: Indicates a master write operation.</li><li>1: Indicates a master read operation.</li></ul>"]
    #[inline]
    pub fn op(&mut self) -> _OPW {
        _OPW { w: self }
    }
    #[doc = "Bits 1:7 - Address. Includes the 7-bit address or the LSB bits of the10-bit address used to initiate the current transaction"]
    #[inline]
    pub fn a7(&mut self) -> _A7W {
        _A7W { w: self }
    }
    #[doc = "Bits 8:10 - Extended address. Includes the extension (MSB bits) of the field A7 used to initiate the current transaction"]
    #[inline]
    pub fn ea10(&mut self) -> _EA10W {
        _EA10W { w: self }
    }
    #[doc = "Bit 11 - Start byte:<ul><li>0: The start byte procedure is not applied to the current transaction.</li><li>1: The start byte procedure is prefixed to the current transaction.</li></ul>"]
    #[inline]
    pub fn sb(&mut self) -> _SBW {
        _SBW { w: self }
    }
    #[doc = "Bits 12:13 - Address type:<ul><li>00b: The transaction is initiated by a general call command. In this case the fields OP, A7, EA10 are \"don't care\".</li><li>01b: The transaction is initiated by the 7-bit address included in the A7 field.</li><li>10b: The transaction is initiated by the 10-bit address included in the EA10 and A7 fields.</li></ul>"]
    #[inline]
    pub fn am(&mut self) -> _AMW {
        _AMW { w: self }
    }
    #[doc = "Bit 14 - Stop condition:<ul><li>0: The current transaction is not terminated by a STOP condition. A repeated START condition is generated on the next operation which is required to avoid to stall the I2C line.</li><li>1: The current transaction is terminated by a STOP condition.</li></ul>"]
    #[inline]
    pub fn p(&mut self) -> _PW {
        _PW { w: self }
    }
    #[doc = "Bits 15:25 - Transaction length. Defines the length, in terms of the number of bytes to be transmitted (MW) or received (MR). In case of write operation, the payload is stored in the Tx FIFO. A transaction can be larger than the Tx FIFO size. In case of read operation the length refers to the number of bytes to be received before generating a not-acknowledge response. A transaction can be larger than the Rx FIFO size. The I2C clock line is stretched low until the data in Rx FIFO are consumed."]
    #[inline]
    pub fn length(&mut self) -> _LENGTHW {
        _LENGTHW { w: self }
    }
}
