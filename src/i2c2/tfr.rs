#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TFR {
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
pub struct TDATAR {
    bits: u8,
}
impl TDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Transmission Data. TDATA contains the payload related to a master write or read-from-slave operation to be written in the Tx FIFO. TDATA(0) is the first LSB bit transmitted over the I2C line.<p>In case of master write operation, the Tx FIFO shall be preloaded otherwise the I2C controller cannot start the operation until data are available.</p><p>In case of read-from-slave operation, when the slave is addressed, the interrupt RISR:RFSR bit is asserted and the CPU shall download the data in the FIFO. If the FIFO is empty and the I2C master is still requiring data, a new request (RISR:RFSE interrupt bit) is asserted to require additional data to the CPU. The slave controller stretches the I2C clock line when no data are available for transmission. Since the Tx FIFO could contain some pending data related to the previous transfer (the transfer length may be unknown to the slave controller), the Tx FIFO is self-flushed before asserting the I2C_RISR:RFSR bit. Upon completion of the read-from-slave operation the interrupt bit I2C_RISR:STD is asserted and the related status of the operation is stored in the I2C_SR register. In CPU mode, the FIFO management shall be based on the assertion of the interrupt bit RISR:TXFNE, related to the nearly-empty threshold.</p><p>In DMA mode, the single/burst requests are automatically executed based on the number of entries available in the TX FIFO and the related destination burst size programmed in the I2C_DMAR:DBSIZE_TX register field. The DMA requests are terminated at the end of the I2C read operation (notacknowledge received by the master) by a dummy last single/burst request.</p>"]
    #[inline]
    pub fn tdata(&self) -> TDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        TDATAR { bits }
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
    #[doc = "Bits 0:7 - Transmission Data. TDATA contains the payload related to a master write or read-from-slave operation to be written in the Tx FIFO. TDATA(0) is the first LSB bit transmitted over the I2C line.<p>In case of master write operation, the Tx FIFO shall be preloaded otherwise the I2C controller cannot start the operation until data are available.</p><p>In case of read-from-slave operation, when the slave is addressed, the interrupt RISR:RFSR bit is asserted and the CPU shall download the data in the FIFO. If the FIFO is empty and the I2C master is still requiring data, a new request (RISR:RFSE interrupt bit) is asserted to require additional data to the CPU. The slave controller stretches the I2C clock line when no data are available for transmission. Since the Tx FIFO could contain some pending data related to the previous transfer (the transfer length may be unknown to the slave controller), the Tx FIFO is self-flushed before asserting the I2C_RISR:RFSR bit. Upon completion of the read-from-slave operation the interrupt bit I2C_RISR:STD is asserted and the related status of the operation is stored in the I2C_SR register. In CPU mode, the FIFO management shall be based on the assertion of the interrupt bit RISR:TXFNE, related to the nearly-empty threshold.</p><p>In DMA mode, the single/burst requests are automatically executed based on the number of entries available in the TX FIFO and the related destination burst size programmed in the I2C_DMAR:DBSIZE_TX register field. The DMA requests are terminated at the end of the I2C read operation (notacknowledge received by the master) by a dummy last single/burst request.</p>"]
    #[inline]
    pub fn tdata(&mut self) -> _TDATAW {
        _TDATAW { w: self }
    }
}
