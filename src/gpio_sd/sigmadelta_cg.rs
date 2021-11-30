#[doc = "Register `SIGMADELTA_CG` reader"]
pub struct R(crate::R<SIGMADELTA_CG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_CG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_CG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_CG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_CG` writer"]
pub struct W(crate::W<SIGMADELTA_CG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_CG_SPEC>;
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
impl From<crate::W<SIGMADELTA_CG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_CG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SD_CLK_EN` reader - "]
pub struct GPIO_SD_CLK_EN_R(crate::FieldReader<bool, bool>);
impl GPIO_SD_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_SD_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_SD_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SD_CLK_EN` writer - "]
pub struct GPIO_SD_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_sd_clk_en(&self) -> GPIO_SD_CLK_EN_R {
        GPIO_SD_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_sd_clk_en(&mut self) -> GPIO_SD_CLK_EN_W {
        GPIO_SD_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_cg](index.html) module"]
pub struct SIGMADELTA_CG_SPEC;
impl crate::RegisterSpec for SIGMADELTA_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_cg::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_CG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_cg::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_CG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA_CG to value 0"]
impl crate::Resettable for SIGMADELTA_CG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
