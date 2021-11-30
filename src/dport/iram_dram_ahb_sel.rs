#[doc = "Register `IRAM_DRAM_AHB_SEL` reader"]
pub struct R(crate::R<IRAM_DRAM_AHB_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRAM_DRAM_AHB_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRAM_DRAM_AHB_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRAM_DRAM_AHB_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRAM_DRAM_AHB_SEL` writer"]
pub struct W(crate::W<IRAM_DRAM_AHB_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRAM_DRAM_AHB_SEL_SPEC>;
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
impl From<crate::W<IRAM_DRAM_AHB_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRAM_DRAM_AHB_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_PRO_IRAM` reader - "]
pub struct MASK_PRO_IRAM_R(crate::FieldReader<bool, bool>);
impl MASK_PRO_IRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_PRO_IRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_PRO_IRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_PRO_IRAM` writer - "]
pub struct MASK_PRO_IRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_PRO_IRAM_W<'a> {
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
#[doc = "Field `MASK_APP_IRAM` reader - "]
pub struct MASK_APP_IRAM_R(crate::FieldReader<bool, bool>);
impl MASK_APP_IRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_APP_IRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_APP_IRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_APP_IRAM` writer - "]
pub struct MASK_APP_IRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_APP_IRAM_W<'a> {
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
#[doc = "Field `MASK_PRO_DRAM` reader - "]
pub struct MASK_PRO_DRAM_R(crate::FieldReader<bool, bool>);
impl MASK_PRO_DRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_PRO_DRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_PRO_DRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_PRO_DRAM` writer - "]
pub struct MASK_PRO_DRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_PRO_DRAM_W<'a> {
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
#[doc = "Field `MASK_APP_DRAM` reader - "]
pub struct MASK_APP_DRAM_R(crate::FieldReader<bool, bool>);
impl MASK_APP_DRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_APP_DRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_APP_DRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_APP_DRAM` writer - "]
pub struct MASK_APP_DRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_APP_DRAM_W<'a> {
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
#[doc = "Field `MASK_AHB` reader - "]
pub struct MASK_AHB_R(crate::FieldReader<bool, bool>);
impl MASK_AHB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_AHB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_AHB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_AHB` writer - "]
pub struct MASK_AHB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_AHB_W<'a> {
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
#[doc = "Field `MAC_DUMP_MODE` reader - "]
pub struct MAC_DUMP_MODE_R(crate::FieldReader<u8, u8>);
impl MAC_DUMP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAC_DUMP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_DUMP_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_DUMP_MODE` writer - "]
pub struct MAC_DUMP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_DUMP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mask_pro_iram(&self) -> MASK_PRO_IRAM_R {
        MASK_PRO_IRAM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mask_app_iram(&self) -> MASK_APP_IRAM_R {
        MASK_APP_IRAM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mask_pro_dram(&self) -> MASK_PRO_DRAM_R {
        MASK_PRO_DRAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mask_app_dram(&self) -> MASK_APP_DRAM_R {
        MASK_APP_DRAM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mask_ahb(&self) -> MASK_AHB_R {
        MASK_AHB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mac_dump_mode(&self) -> MAC_DUMP_MODE_R {
        MAC_DUMP_MODE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mask_pro_iram(&mut self) -> MASK_PRO_IRAM_W {
        MASK_PRO_IRAM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mask_app_iram(&mut self) -> MASK_APP_IRAM_W {
        MASK_APP_IRAM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mask_pro_dram(&mut self) -> MASK_PRO_DRAM_W {
        MASK_PRO_DRAM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mask_app_dram(&mut self) -> MASK_APP_DRAM_W {
        MASK_APP_DRAM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mask_ahb(&mut self) -> MASK_AHB_W {
        MASK_AHB_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mac_dump_mode(&mut self) -> MAC_DUMP_MODE_W {
        MAC_DUMP_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iram_dram_ahb_sel](index.html) module"]
pub struct IRAM_DRAM_AHB_SEL_SPEC;
impl crate::RegisterSpec for IRAM_DRAM_AHB_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iram_dram_ahb_sel::R](R) reader structure"]
impl crate::Readable for IRAM_DRAM_AHB_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iram_dram_ahb_sel::W](W) writer structure"]
impl crate::Writable for IRAM_DRAM_AHB_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRAM_DRAM_AHB_SEL to value 0"]
impl crate::Resettable for IRAM_DRAM_AHB_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
