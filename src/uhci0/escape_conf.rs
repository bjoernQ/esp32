#[doc = "Register `ESCAPE_CONF` reader"]
pub struct R(crate::R<ESCAPE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCAPE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCAPE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCAPE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCAPE_CONF` writer"]
pub struct W(crate::W<ESCAPE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCAPE_CONF_SPEC>;
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
impl From<crate::W<ESCAPE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCAPE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_C0_ESC_EN` reader - Set this bit to enable 0xc0 char decode when DMA receives data."]
pub struct TX_C0_ESC_EN_R(crate::FieldReader<bool, bool>);
impl TX_C0_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_C0_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_C0_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_C0_ESC_EN` writer - Set this bit to enable 0xc0 char decode when DMA receives data."]
pub struct TX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_C0_ESC_EN_W<'a> {
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
#[doc = "Field `TX_DB_ESC_EN` reader - Set this bit to enable 0xdb char decode when DMA receives data."]
pub struct TX_DB_ESC_EN_R(crate::FieldReader<bool, bool>);
impl TX_DB_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DB_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DB_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DB_ESC_EN` writer - Set this bit to enable 0xdb char decode when DMA receives data."]
pub struct TX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DB_ESC_EN_W<'a> {
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
#[doc = "Field `TX_11_ESC_EN` reader - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
pub struct TX_11_ESC_EN_R(crate::FieldReader<bool, bool>);
impl TX_11_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_11_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_11_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_11_ESC_EN` writer - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
pub struct TX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_11_ESC_EN_W<'a> {
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
#[doc = "Field `TX_13_ESC_EN` reader - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
pub struct TX_13_ESC_EN_R(crate::FieldReader<bool, bool>);
impl TX_13_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_13_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_13_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_13_ESC_EN` writer - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
pub struct TX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_13_ESC_EN_W<'a> {
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
#[doc = "Field `RX_C0_ESC_EN` reader - Set this bit to enable 0xc0 char replace when DMA sends data."]
pub struct RX_C0_ESC_EN_R(crate::FieldReader<bool, bool>);
impl RX_C0_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_C0_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_C0_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_C0_ESC_EN` writer - Set this bit to enable 0xc0 char replace when DMA sends data."]
pub struct RX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_C0_ESC_EN_W<'a> {
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
#[doc = "Field `RX_DB_ESC_EN` reader - Set this bit to enable 0xdb char replace when DMA sends data."]
pub struct RX_DB_ESC_EN_R(crate::FieldReader<bool, bool>);
impl RX_DB_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DB_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DB_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DB_ESC_EN` writer - Set this bit to enable 0xdb char replace when DMA sends data."]
pub struct RX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DB_ESC_EN_W<'a> {
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
#[doc = "Field `RX_11_ESC_EN` reader - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
pub struct RX_11_ESC_EN_R(crate::FieldReader<bool, bool>);
impl RX_11_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_11_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_11_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_11_ESC_EN` writer - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
pub struct RX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_11_ESC_EN_W<'a> {
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
#[doc = "Field `RX_13_ESC_EN` reader - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
pub struct RX_13_ESC_EN_R(crate::FieldReader<bool, bool>);
impl RX_13_ESC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_13_ESC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_13_ESC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_13_ESC_EN` writer - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
pub struct RX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_13_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to enable 0xc0 char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_c0_esc_en(&self) -> TX_C0_ESC_EN_R {
        TX_C0_ESC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable 0xdb char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_db_esc_en(&self) -> TX_DB_ESC_EN_R {
        TX_DB_ESC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_11_esc_en(&self) -> TX_11_ESC_EN_R {
        TX_11_ESC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_13_esc_en(&self) -> TX_13_ESC_EN_R {
        TX_13_ESC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable 0xc0 char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_c0_esc_en(&self) -> RX_C0_ESC_EN_R {
        RX_C0_ESC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable 0xdb char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_db_esc_en(&self) -> RX_DB_ESC_EN_R {
        RX_DB_ESC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_11_esc_en(&self) -> RX_11_ESC_EN_R {
        RX_11_ESC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_13_esc_en(&self) -> RX_13_ESC_EN_R {
        RX_13_ESC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable 0xc0 char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_c0_esc_en(&mut self) -> TX_C0_ESC_EN_W {
        TX_C0_ESC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable 0xdb char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_db_esc_en(&mut self) -> TX_DB_ESC_EN_W {
        TX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_11_esc_en(&mut self) -> TX_11_ESC_EN_W {
        TX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_13_esc_en(&mut self) -> TX_13_ESC_EN_W {
        TX_13_ESC_EN_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable 0xc0 char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_c0_esc_en(&mut self) -> RX_C0_ESC_EN_W {
        RX_C0_ESC_EN_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable 0xdb char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_db_esc_en(&mut self) -> RX_DB_ESC_EN_W {
        RX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_11_esc_en(&mut self) -> RX_11_ESC_EN_W {
        RX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_13_esc_en(&mut self) -> RX_13_ESC_EN_W {
        RX_13_ESC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escape_conf](index.html) module"]
pub struct ESCAPE_CONF_SPEC;
impl crate::RegisterSpec for ESCAPE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [escape_conf::R](R) reader structure"]
impl crate::Readable for ESCAPE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [escape_conf::W](W) writer structure"]
impl crate::Writable for ESCAPE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCAPE_CONF to value 0x33"]
impl crate::Resettable for ESCAPE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33
    }
}
