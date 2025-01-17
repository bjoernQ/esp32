#[doc = "Register `APB_SARADC_SAR1_PATT_TAB2` reader"]
pub struct R(crate::R<APB_SARADC_SAR1_PATT_TAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC_SAR1_PATT_TAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC_SAR1_PATT_TAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC_SAR1_PATT_TAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_SARADC_SAR1_PATT_TAB2` writer"]
pub struct W(crate::W<APB_SARADC_SAR1_PATT_TAB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_SARADC_SAR1_PATT_TAB2_SPEC>;
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
impl From<crate::W<APB_SARADC_SAR1_PATT_TAB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_SARADC_SAR1_PATT_TAB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB2` reader - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub struct SARADC_SAR1_PATT_TAB2_R(crate::FieldReader<u32, u32>);
impl SARADC_SAR1_PATT_TAB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SARADC_SAR1_PATT_TAB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR1_PATT_TAB2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB2` writer - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
pub struct SARADC_SAR1_PATT_TAB2_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_PATT_TAB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab2(&self) -> SARADC_SAR1_PATT_TAB2_R {
        SARADC_SAR1_PATT_TAB2_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab2(&mut self) -> SARADC_SAR1_PATT_TAB2_W {
        SARADC_SAR1_PATT_TAB2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar1_patt_tab2](index.html) module"]
pub struct APB_SARADC_SAR1_PATT_TAB2_SPEC;
impl crate::RegisterSpec for APB_SARADC_SAR1_PATT_TAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc_sar1_patt_tab2::R](R) reader structure"]
impl crate::Readable for APB_SARADC_SAR1_PATT_TAB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_saradc_sar1_patt_tab2::W](W) writer structure"]
impl crate::Writable for APB_SARADC_SAR1_PATT_TAB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_SARADC_SAR1_PATT_TAB2 to value 0x0f0f_0f0f"]
impl crate::Resettable for APB_SARADC_SAR1_PATT_TAB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f_0f0f
    }
}
