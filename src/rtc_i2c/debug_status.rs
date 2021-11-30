#[doc = "Register `DEBUG_STATUS` reader"]
pub struct R(crate::R<DEBUG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_STATUS` writer"]
pub struct W(crate::W<DEBUG_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEBUG_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_VAL` reader - The value of an acknowledge signal on the bus"]
pub struct ACK_VAL_R(crate::FieldReader<bool, bool>);
impl ACK_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_VAL` writer - The value of an acknowledge signal on the bus"]
pub struct ACK_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_VAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SLAVE_RW` reader - When working as a slave, the value of R/W bit received"]
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
#[doc = "Field `SLAVE_RW` writer - When working as a slave, the value of R/W bit received"]
pub struct SLAVE_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_RW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TIMED_OUT` reader - Transfer has timed out"]
pub struct TIMED_OUT_R(crate::FieldReader<bool, bool>);
impl TIMED_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMED_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMED_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMED_OUT` writer - Transfer has timed out"]
pub struct TIMED_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMED_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ARB_LOST` reader - When working as a master, lost control of I2C bus"]
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
#[doc = "Field `ARB_LOST` writer - When working as a master, lost control of I2C bus"]
pub struct ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_LOST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BUS_BUSY` reader - operation is in progress"]
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
#[doc = "Field `BUS_BUSY` writer - operation is in progress"]
pub struct BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_BUSY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SLAVE_ADDR_MATCH` reader - When working as a slave, whether address was matched"]
pub struct SLAVE_ADDR_MATCH_R(crate::FieldReader<bool, bool>);
impl SLAVE_ADDR_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_ADDR_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_MATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDR_MATCH` writer - When working as a slave, whether address was matched"]
pub struct SLAVE_ADDR_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_MATCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BYTE_TRANS` reader - 8 bit transmit done"]
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
#[doc = "Field `BYTE_TRANS` writer - 8 bit transmit done"]
pub struct BYTE_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_TRANS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MAIN_STATE` reader - state of the main state machine"]
pub struct MAIN_STATE_R(crate::FieldReader<u8, u8>);
impl MAIN_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAIN_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE` writer - state of the main state machine"]
pub struct MAIN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `SCL_STATE` reader - state of SCL state machine"]
pub struct SCL_STATE_R(crate::FieldReader<u8, u8>);
impl SCL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_STATE` writer - state of SCL state machine"]
pub struct SCL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&self) -> ACK_VAL_R {
        ACK_VAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&self) -> TIMED_OUT_R {
        TIMED_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&self) -> SLAVE_ADDR_MATCH_R {
        SLAVE_ADDR_MATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&mut self) -> ACK_VAL_W {
        ACK_VAL_W { w: self }
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&mut self) -> SLAVE_RW_W {
        SLAVE_RW_W { w: self }
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&mut self) -> TIMED_OUT_W {
        TIMED_OUT_W { w: self }
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARB_LOST_W {
        ARB_LOST_W { w: self }
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&mut self) -> BUS_BUSY_W {
        BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&mut self) -> SLAVE_ADDR_MATCH_W {
        SLAVE_ADDR_MATCH_W { w: self }
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&mut self) -> BYTE_TRANS_W {
        BYTE_TRANS_W { w: self }
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&mut self) -> MAIN_STATE_W {
        MAIN_STATE_W { w: self }
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&mut self) -> SCL_STATE_W {
        SCL_STATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_status](index.html) module"]
pub struct DEBUG_STATUS_SPEC;
impl crate::RegisterSpec for DEBUG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_status::R](R) reader structure"]
impl crate::Readable for DEBUG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_status::W](W) writer structure"]
impl crate::Writable for DEBUG_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_STATUS to value 0"]
impl crate::Resettable for DEBUG_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
