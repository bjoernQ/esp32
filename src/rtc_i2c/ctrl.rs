#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FORCE_OUT` reader - SDA is push-pull (1) or open-drain (0)"]
pub struct SDA_FORCE_OUT_R(crate::FieldReader<bool, bool>);
impl SDA_FORCE_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_FORCE_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_FORCE_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_FORCE_OUT` writer - SDA is push-pull (1) or open-drain (0)"]
pub struct SDA_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FORCE_OUT_W<'a> {
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
#[doc = "Field `SCL_FORCE_OUT` reader - SCL is push-pull (1) or open-drain (0)"]
pub struct SCL_FORCE_OUT_R(crate::FieldReader<bool, bool>);
impl SCL_FORCE_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_FORCE_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_FORCE_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_FORCE_OUT` writer - SCL is push-pull (1) or open-drain (0)"]
pub struct SCL_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FORCE_OUT_W<'a> {
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
#[doc = "Field `MS_MODE` reader - Master (1) or slave (0)"]
pub struct MS_MODE_R(crate::FieldReader<bool, bool>);
impl MS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS_MODE` writer - Master (1) or slave (0)"]
pub struct MS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_MODE_W<'a> {
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
#[doc = "Field `TRANS_START` reader - Force to generate start condition"]
pub struct TRANS_START_R(crate::FieldReader<bool, bool>);
impl TRANS_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_START` writer - Force to generate start condition"]
pub struct TRANS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_START_W<'a> {
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
#[doc = "Field `TX_LSB_FIRST` reader - Send LSB first"]
pub struct TX_LSB_FIRST_R(crate::FieldReader<bool, bool>);
impl TX_LSB_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LSB_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LSB_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LSB_FIRST` writer - Send LSB first"]
pub struct TX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LSB_FIRST_W<'a> {
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
#[doc = "Field `RX_LSB_FIRST` reader - Receive LSB first"]
pub struct RX_LSB_FIRST_R(crate::FieldReader<bool, bool>);
impl RX_LSB_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_LSB_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LSB_FIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_LSB_FIRST` writer - Receive LSB first"]
pub struct RX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LSB_FIRST_W<'a> {
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
    #[doc = "Bit 0 - SDA is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master (1) or slave (0)"]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force to generate start condition"]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Send LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDA is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W {
        SDA_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 1 - SCL is push-pull (1) or open-drain (0)"]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W {
        SCL_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 4 - Master (1) or slave (0)"]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W {
        MS_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Force to generate start condition"]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W {
        TRANS_START_W { w: self }
    }
    #[doc = "Bit 6 - Send LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W {
        TX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 7 - Receive LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W {
        RX_LSB_FIRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
