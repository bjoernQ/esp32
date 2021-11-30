#[doc = "Register `RS485_CONF` reader"]
pub struct R(crate::R<RS485_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485_CONF` writer"]
pub struct W(crate::W<RS485_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485_CONF_SPEC>;
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
impl From<crate::W<RS485_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS485_EN` reader - Set this bit to choose rs485 mode."]
pub struct RS485_EN_R(crate::FieldReader<bool, bool>);
impl RS485_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_EN` writer - Set this bit to choose rs485 mode."]
pub struct RS485_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_EN_W<'a> {
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
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub struct DL0_EN_R(crate::FieldReader<bool, bool>);
impl DL0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DL0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub struct DL0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL0_EN_W<'a> {
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
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub struct DL1_EN_R(crate::FieldReader<bool, bool>);
impl DL1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DL1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub struct DL1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL1_EN_W<'a> {
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
#[doc = "Field `RS485TX_RX_EN` reader - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
pub struct RS485TX_RX_EN_R(crate::FieldReader<bool, bool>);
impl RS485TX_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485TX_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485TX_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485TX_RX_EN` writer - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
pub struct RS485TX_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485TX_RX_EN_W<'a> {
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
#[doc = "Field `RS485RXBY_TX_EN` reader - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
pub struct RS485RXBY_TX_EN_R(crate::FieldReader<bool, bool>);
impl RS485RXBY_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485RXBY_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485RXBY_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485RXBY_TX_EN` writer - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
pub struct RS485RXBY_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485RXBY_TX_EN_W<'a> {
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
#[doc = "Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal."]
pub struct RS485_RX_DLY_NUM_R(crate::FieldReader<bool, bool>);
impl RS485_RX_DLY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_RX_DLY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_RX_DLY_NUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal."]
pub struct RS485_RX_DLY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_RX_DLY_NUM_W<'a> {
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
#[doc = "Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal."]
pub struct RS485_TX_DLY_NUM_R(crate::FieldReader<u8, u8>);
impl RS485_TX_DLY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RS485_TX_DLY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_TX_DLY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal."]
pub struct RS485_TX_DLY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_TX_DLY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&self) -> RS485_EN_R {
        RS485_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> RS485TX_RX_EN_R {
        RS485TX_RX_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&self) -> RS485RXBY_TX_EN_R {
        RS485RXBY_TX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&self) -> RS485_RX_DLY_NUM_R {
        RS485_RX_DLY_NUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&self) -> RS485_TX_DLY_NUM_R {
        RS485_TX_DLY_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to choose rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&mut self) -> RS485_EN_W {
        RS485_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&mut self) -> DL0_EN_W {
        DL0_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&mut self) -> DL1_EN_W {
        DL1_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable loopback transmitter's output data signal to receiver's input data signal."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&mut self) -> RS485TX_RX_EN_W {
        RS485TX_RX_EN_W { w: self }
    }
    #[doc = "Bit 4 - 1: enable rs485's transmitter to send data when rs485's receiver is busy. 0:rs485's transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&mut self) -> RS485RXBY_TX_EN_W {
        RS485RXBY_TX_EN_W { w: self }
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&mut self) -> RS485_RX_DLY_NUM_W {
        RS485_RX_DLY_NUM_W { w: self }
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&mut self) -> RS485_TX_DLY_NUM_W {
        RS485_TX_DLY_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485_conf](index.html) module"]
pub struct RS485_CONF_SPEC;
impl crate::RegisterSpec for RS485_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485_conf::R](R) reader structure"]
impl crate::Readable for RS485_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485_conf::W](W) writer structure"]
impl crate::Writable for RS485_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for RS485_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
