#[doc = "Register `FIFO_CONF` reader"]
pub struct R(crate::R<FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CONF` writer"]
pub struct W(crate::W<FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONF_SPEC>;
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
impl From<crate::W<FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` reader - "]
pub struct RXFIFO_FULL_THRHD_R(crate::FieldReader<u8, u8>);
impl RXFIFO_FULL_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_FULL_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_THRHD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` writer - "]
pub struct RXFIFO_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - Config txfifo empty threhd value when using apb fifo access"]
pub struct TXFIFO_EMPTY_THRHD_R(crate::FieldReader<u8, u8>);
impl TXFIFO_EMPTY_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_EMPTY_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_THRHD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - Config txfifo empty threhd value when using apb fifo access"]
pub struct TXFIFO_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enble apb nonfifo access."]
pub struct NONFIFO_EN_R(crate::FieldReader<bool, bool>);
impl NONFIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NONFIFO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NONFIFO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enble apb nonfifo access."]
pub struct NONFIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub struct FIFO_ADDR_CFG_EN_R(crate::FieldReader<bool, bool>);
impl FIFO_ADDR_CFG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_ADDR_CFG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_ADDR_CFG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub struct FIFO_ADDR_CFG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_ADDR_CFG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset rx fifo when using apb fifo access."]
pub struct RX_FIFO_RST_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset rx fifo when using apb fifo access."]
pub struct RX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset tx fifo when using apb fifo access."]
pub struct TX_FIFO_RST_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset tx fifo when using apb fifo access."]
pub struct TX_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `NONFIFO_RX_THRES` reader - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub struct NONFIFO_RX_THRES_R(crate::FieldReader<u8, u8>);
impl NONFIFO_RX_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NONFIFO_RX_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NONFIFO_RX_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONFIFO_RX_THRES` writer - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub struct NONFIFO_RX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_RX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
#[doc = "Field `NONFIFO_TX_THRES` reader - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub struct NONFIFO_TX_THRES_R(crate::FieldReader<u8, u8>);
impl NONFIFO_TX_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NONFIFO_TX_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NONFIFO_TX_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NONFIFO_TX_THRES` writer - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub struct NONFIFO_TX_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NONFIFO_TX_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&self) -> NONFIFO_RX_THRES_R {
        NONFIFO_RX_THRES_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn nonfifo_tx_thres(&self) -> NONFIFO_TX_THRES_R {
        NONFIFO_TX_THRES_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W {
        RXFIFO_FULL_THRHD_W { w: self }
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W {
        TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W {
        NONFIFO_EN_W { w: self }
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W {
        FIFO_ADDR_CFG_EN_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W {
        RX_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W {
        TX_FIFO_RST_W { w: self }
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&mut self) -> NONFIFO_RX_THRES_W {
        NONFIFO_RX_THRES_W { w: self }
    }
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn nonfifo_tx_thres(&mut self) -> NONFIFO_TX_THRES_W {
        NONFIFO_TX_THRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](index.html) module"]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_conf::R](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x0155_408b"]
impl crate::Resettable for FIFO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0155_408b
    }
}
