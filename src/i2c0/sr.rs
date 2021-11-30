#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACK_REC` reader - This register stores the value of ACK bit."]
pub struct ACK_REC_R(crate::FieldReader<bool, bool>);
impl ACK_REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_REC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_RW` reader - when in slave mode 1: master read slave 0: master write slave."]
pub struct SLAVE_RW_R(crate::FieldReader<bool, bool>);
impl SLAVE_RW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT` reader - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
pub struct TIME_OUT_R(crate::FieldReader<bool, bool>);
impl TIME_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST` reader - when I2C lost control of SDA line this register changes to high level."]
pub struct ARB_LOST_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_BUSY` reader - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
pub struct BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl BUS_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDRESSED` reader - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
pub struct SLAVE_ADDRESSED_R(crate::FieldReader<bool, bool>);
impl SLAVE_ADDRESSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_ADDRESSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDRESSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_TRANS` reader - This register changes to high level when one byte is transferred."]
pub struct BYTE_TRANS_R(crate::FieldReader<bool, bool>);
impl BYTE_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_CNT` reader - This register represent the amount of data need to send."]
pub struct RXFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_CNT` reader - This register stores the amount of received data in ram."]
pub struct TXFIFO_CNT_R(crate::FieldReader<u8, u8>);
impl TXFIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
pub struct SCL_MAIN_STATE_LAST_R(crate::FieldReader<u8, u8>);
impl SCL_MAIN_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_MAIN_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_STATE_LAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_STATE_LAST` reader - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
pub struct SCL_STATE_LAST_R(crate::FieldReader<u8, u8>);
impl SCL_STATE_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_STATE_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_STATE_LAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
