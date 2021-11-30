#[doc = "Register `HOST_SLCHOST_CONF` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_FRC_SDIO11` reader - "]
pub struct HOST_FRC_SDIO11_R(crate::FieldReader<u8, u8>);
impl HOST_FRC_SDIO11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_FRC_SDIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FRC_SDIO11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FRC_SDIO11` writer - "]
pub struct HOST_FRC_SDIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_SDIO11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `HOST_FRC_SDIO20` reader - "]
pub struct HOST_FRC_SDIO20_R(crate::FieldReader<u8, u8>);
impl HOST_FRC_SDIO20_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_FRC_SDIO20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FRC_SDIO20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FRC_SDIO20` writer - "]
pub struct HOST_FRC_SDIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_SDIO20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `HOST_FRC_NEG_SAMP` reader - "]
pub struct HOST_FRC_NEG_SAMP_R(crate::FieldReader<u8, u8>);
impl HOST_FRC_NEG_SAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_FRC_NEG_SAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FRC_NEG_SAMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FRC_NEG_SAMP` writer - "]
pub struct HOST_FRC_NEG_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_NEG_SAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `HOST_FRC_POS_SAMP` reader - "]
pub struct HOST_FRC_POS_SAMP_R(crate::FieldReader<u8, u8>);
impl HOST_FRC_POS_SAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_FRC_POS_SAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FRC_POS_SAMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FRC_POS_SAMP` writer - "]
pub struct HOST_FRC_POS_SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_POS_SAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `HOST_FRC_QUICK_IN` reader - "]
pub struct HOST_FRC_QUICK_IN_R(crate::FieldReader<u8, u8>);
impl HOST_FRC_QUICK_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_FRC_QUICK_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_FRC_QUICK_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_FRC_QUICK_IN` writer - "]
pub struct HOST_FRC_QUICK_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FRC_QUICK_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `HOST_SDIO20_INT_DELAY` reader - "]
pub struct HOST_SDIO20_INT_DELAY_R(crate::FieldReader<bool, bool>);
impl HOST_SDIO20_INT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SDIO20_INT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SDIO20_INT_DELAY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SDIO20_INT_DELAY` writer - "]
pub struct HOST_SDIO20_INT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO20_INT_DELAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `HOST_SDIO_PAD_PULLUP` reader - "]
pub struct HOST_SDIO_PAD_PULLUP_R(crate::FieldReader<bool, bool>);
impl HOST_SDIO_PAD_PULLUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SDIO_PAD_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SDIO_PAD_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SDIO_PAD_PULLUP` writer - "]
pub struct HOST_SDIO_PAD_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SDIO_PAD_PULLUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `HOST_HSPEED_CON_EN` reader - "]
pub struct HOST_HSPEED_CON_EN_R(crate::FieldReader<bool, bool>);
impl HOST_HSPEED_CON_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_HSPEED_CON_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_HSPEED_CON_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_HSPEED_CON_EN` writer - "]
pub struct HOST_HSPEED_CON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_HSPEED_CON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&self) -> HOST_FRC_SDIO11_R {
        HOST_FRC_SDIO11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&self) -> HOST_FRC_SDIO20_R {
        HOST_FRC_SDIO20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&self) -> HOST_FRC_NEG_SAMP_R {
        HOST_FRC_NEG_SAMP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&self) -> HOST_FRC_POS_SAMP_R {
        HOST_FRC_POS_SAMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&self) -> HOST_FRC_QUICK_IN_R {
        HOST_FRC_QUICK_IN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&self) -> HOST_SDIO20_INT_DELAY_R {
        HOST_SDIO20_INT_DELAY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&self) -> HOST_SDIO_PAD_PULLUP_R {
        HOST_SDIO_PAD_PULLUP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&self) -> HOST_HSPEED_CON_EN_R {
        HOST_HSPEED_CON_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_frc_sdio11(&mut self) -> HOST_FRC_SDIO11_W {
        HOST_FRC_SDIO11_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_frc_sdio20(&mut self) -> HOST_FRC_SDIO20_W {
        HOST_FRC_SDIO20_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_frc_neg_samp(&mut self) -> HOST_FRC_NEG_SAMP_W {
        HOST_FRC_NEG_SAMP_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn host_frc_pos_samp(&mut self) -> HOST_FRC_POS_SAMP_W {
        HOST_FRC_POS_SAMP_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn host_frc_quick_in(&mut self) -> HOST_FRC_QUICK_IN_W {
        HOST_FRC_QUICK_IN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_sdio20_int_delay(&mut self) -> HOST_SDIO20_INT_DELAY_W {
        HOST_SDIO20_INT_DELAY_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn host_sdio_pad_pullup(&mut self) -> HOST_SDIO_PAD_PULLUP_W {
        HOST_SDIO_PAD_PULLUP_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn host_hspeed_con_en(&mut self) -> HOST_HSPEED_CON_EN_W {
        HOST_HSPEED_CON_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf](index.html) module"]
pub struct HOST_SLCHOST_CONF_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
