#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct GIF0R {
    bits: bool,
}
impl GIF0R {
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
pub struct TCIF0R {
    bits: bool,
}
impl TCIF0R {
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
pub struct HTIF0R {
    bits: bool,
}
impl HTIF0R {
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
pub struct TEIF0R {
    bits: bool,
}
impl TEIF0R {
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
pub struct GIF1R {
    bits: bool,
}
impl GIF1R {
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
pub struct TCIF1R {
    bits: bool,
}
impl TCIF1R {
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
pub struct HTIF1R {
    bits: bool,
}
impl HTIF1R {
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
pub struct TEIF1R {
    bits: bool,
}
impl TEIF1R {
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
pub struct GIF2R {
    bits: bool,
}
impl GIF2R {
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
pub struct TCIF2R {
    bits: bool,
}
impl TCIF2R {
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
pub struct HTIF2R {
    bits: bool,
}
impl HTIF2R {
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
pub struct TEIF2R {
    bits: bool,
}
impl TEIF2R {
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
pub struct GIF3R {
    bits: bool,
}
impl GIF3R {
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
pub struct TCIF3R {
    bits: bool,
}
impl TCIF3R {
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
pub struct HTIF3R {
    bits: bool,
}
impl HTIF3R {
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
pub struct TEIF3R {
    bits: bool,
}
impl TEIF3R {
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
pub struct GIF4R {
    bits: bool,
}
impl GIF4R {
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
pub struct TCIF4R {
    bits: bool,
}
impl TCIF4R {
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
pub struct HTIF4R {
    bits: bool,
}
impl HTIF4R {
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
pub struct TEIF4R {
    bits: bool,
}
impl TEIF4R {
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
pub struct GIF5R {
    bits: bool,
}
impl GIF5R {
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
pub struct TCIF5R {
    bits: bool,
}
impl TCIF5R {
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
pub struct HTIF5R {
    bits: bool,
}
impl HTIF5R {
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
pub struct TEIF5R {
    bits: bool,
}
impl TEIF5R {
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
pub struct GIF6R {
    bits: bool,
}
impl GIF6R {
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
pub struct TCIF6R {
    bits: bool,
}
impl TCIF6R {
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
pub struct HTIF6R {
    bits: bool,
}
impl HTIF6R {
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
pub struct TEIF6R {
    bits: bool,
}
impl TEIF6R {
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
pub struct GIF7R {
    bits: bool,
}
impl GIF7R {
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
pub struct TCIF7R {
    bits: bool,
}
impl TCIF7R {
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
pub struct HTIF7R {
    bits: bool,
}
impl HTIF7R {
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
pub struct TEIF7R {
    bits: bool,
}
impl TEIF7R {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 0.</li><li>1: A TE, HT or TC event occurred on channel 0.</li></ul>"]
    #[inline]
    pub fn gif0(&self) -> GIF0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF0R { bits }
    }
    #[doc = "Bit 1 - Channel 0 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 0.</li><li>1: A transfer complete (TC) occurred on channel 0.</li></ul>"]
    #[inline]
    pub fn tcif0(&self) -> TCIF0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF0R { bits }
    }
    #[doc = "Bit 2 - Channel 0 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 0.</li><li>1: A half transfer (HT) event occurred on channel 0.</li></ul>"]
    #[inline]
    pub fn htif0(&self) -> HTIF0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF0R { bits }
    }
    #[doc = "Bit 3 - Channel 0 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 0.</li><li>1: A transfer error (TE) occurred on channel 0.</li></ul>"]
    #[inline]
    pub fn teif0(&self) -> TEIF0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF0R { bits }
    }
    #[doc = "Bit 4 - Channel 1 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 1.</li><li>1: A TE, HT or TC event occurred on channel 1.</li></ul>"]
    #[inline]
    pub fn gif1(&self) -> GIF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF1R { bits }
    }
    #[doc = "Bit 5 - Channel 1 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 1.</li><li>1: A transfer complete (TC) occurred on channel 1.</li></ul>"]
    #[inline]
    pub fn tcif1(&self) -> TCIF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF1R { bits }
    }
    #[doc = "Bit 6 - Channel 1 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 1.</li><li>1: A half transfer (HT) event occurred on channel 1.</li></ul>"]
    #[inline]
    pub fn htif1(&self) -> HTIF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF1R { bits }
    }
    #[doc = "Bit 7 - Channel 1 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 1.</li><li>1: A transfer error (TE) occurred on channel 1.</li></ul>"]
    #[inline]
    pub fn teif1(&self) -> TEIF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF1R { bits }
    }
    #[doc = "Bit 8 - Channel 2 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 2.</li><li>1: A TE, HT or TC event occurred on channel 2.</li></ul>"]
    #[inline]
    pub fn gif2(&self) -> GIF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF2R { bits }
    }
    #[doc = "Bit 9 - Channel 2 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 2.</li><li>1: A transfer complete (TC) occurred on channel 2.</li></ul>"]
    #[inline]
    pub fn tcif2(&self) -> TCIF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF2R { bits }
    }
    #[doc = "Bit 10 - Channel 2 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 2.</li><li>1: A half transfer (HT) event occurred on channel 2.</li></ul>"]
    #[inline]
    pub fn htif2(&self) -> HTIF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF2R { bits }
    }
    #[doc = "Bit 11 - Channel 2 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 2.</li><li>1: A transfer error (TE) occurred on channel 2.</li></ul>"]
    #[inline]
    pub fn teif2(&self) -> TEIF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF2R { bits }
    }
    #[doc = "Bit 12 - Channel 3 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 3.</li><li>1: A TE, HT or TC event occurred on channel 3.</li></ul>"]
    #[inline]
    pub fn gif3(&self) -> GIF3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF3R { bits }
    }
    #[doc = "Bit 13 - Channel 3 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 3.</li><li>1: A transfer complete (TC) occurred on channel 3.</li></ul>"]
    #[inline]
    pub fn tcif3(&self) -> TCIF3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF3R { bits }
    }
    #[doc = "Bit 14 - Channel 3 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 3.</li><li>1: A half transfer (HT) event occurred on channel 3.</li></ul>"]
    #[inline]
    pub fn htif3(&self) -> HTIF3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF3R { bits }
    }
    #[doc = "Bit 15 - Channel 3 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 3.</li><li>1: A transfer error (TE) occurred on channel 3.</li></ul>"]
    #[inline]
    pub fn teif3(&self) -> TEIF3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF3R { bits }
    }
    #[doc = "Bit 16 - Channel 4 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 4.</li><li>1: A TE, HT or TC event occurred on channel 4.</li></ul>"]
    #[inline]
    pub fn gif4(&self) -> GIF4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF4R { bits }
    }
    #[doc = "Bit 17 - Channel 4 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 4.</li><li>1: A transfer complete (TC) occurred on channel 4.</li></ul>"]
    #[inline]
    pub fn tcif4(&self) -> TCIF4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF4R { bits }
    }
    #[doc = "Bit 18 - Channel 4 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 4.</li><li>1: A half transfer (HT) event occurred on channel 4.</li></ul>"]
    #[inline]
    pub fn htif4(&self) -> HTIF4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF4R { bits }
    }
    #[doc = "Bit 19 - Channel 4 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 4.</li><li>1: A transfer error (TE) occurred on channel 4.</li></ul>"]
    #[inline]
    pub fn teif4(&self) -> TEIF4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF4R { bits }
    }
    #[doc = "Bit 20 - Channel 5 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 5.</li><li>1: A TE, HT or TC event occurred on channel 5.</li></ul>"]
    #[inline]
    pub fn gif5(&self) -> GIF5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF5R { bits }
    }
    #[doc = "Bit 21 - Channel 5 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 5.</li><li>1: A transfer complete (TC) occurred on channel 5.</li></ul>"]
    #[inline]
    pub fn tcif5(&self) -> TCIF5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF5R { bits }
    }
    #[doc = "Bit 22 - Channel 5 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 5.</li><li>1: A half transfer (HT) event occurred on channel 5.</li></ul>"]
    #[inline]
    pub fn htif5(&self) -> HTIF5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF5R { bits }
    }
    #[doc = "Bit 23 - Channel 5 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 5.</li><li>1: A transfer error (TE) occurred on channel 5.</li></ul>"]
    #[inline]
    pub fn teif5(&self) -> TEIF5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF5R { bits }
    }
    #[doc = "Bit 24 - Channel 6 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 6.</li><li>1: A TE, HT or TC event occurred on channel 6.</li></ul>"]
    #[inline]
    pub fn gif6(&self) -> GIF6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF6R { bits }
    }
    #[doc = "Bit 25 - Channel 6 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 6.</li><li>1: A transfer complete (TC) occurred on channel 6.</li></ul>"]
    #[inline]
    pub fn tcif6(&self) -> TCIF6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF6R { bits }
    }
    #[doc = "Bit 26 - Channel 6 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 6.</li><li>1: A half transfer (HT) event occurred on channel 6.</li></ul>"]
    #[inline]
    pub fn htif6(&self) -> HTIF6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF6R { bits }
    }
    #[doc = "Bit 27 - Channel 6 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 6.</li><li>1: A transfer error (TE) occurred on channel 6.</li></ul>"]
    #[inline]
    pub fn teif6(&self) -> TEIF6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF6R { bits }
    }
    #[doc = "Bit 28 - Channel 7 global interrupt flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No TE, HT or TC event on channel 7.</li><li>1: A TE, HT or TC event occurred on channel 7.</li></ul>"]
    #[inline]
    pub fn gif7(&self) -> GIF7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GIF7R { bits }
    }
    #[doc = "Bit 29 - Channel 7 transfer complete flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer complete (TC) on channel 7.</li><li>1: A transfer complete (TC) occurred on channel 7.</li></ul>"]
    #[inline]
    pub fn tcif7(&self) -> TCIF7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCIF7R { bits }
    }
    #[doc = "Bit 30 - Channel 7 half transfer flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No half transfer (HT) event on channel 7.</li><li>1: A half transfer (HT) event occurred on channel 7.</li></ul>"]
    #[inline]
    pub fn htif7(&self) -> HTIF7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HTIF7R { bits }
    }
    #[doc = "Bit 31 - Channel 7 transfer error flag. This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the IFCR register.<ul><li>0: No transfer error (TE) event on channel 7.</li><li>1: A transfer error (TE) occurred on channel 7.</li></ul>"]
    #[inline]
    pub fn teif7(&self) -> TEIF7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEIF7R { bits }
    }
}
