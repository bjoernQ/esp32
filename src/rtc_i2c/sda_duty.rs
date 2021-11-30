#[doc = "Register `SDA_DUTY` reader"]
pub struct R(crate::R<SDA_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA_DUTY` writer"]
pub struct W(crate::W<SDA_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA_DUTY_SPEC>;
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
impl From<crate::W<SDA_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_DUTY` reader - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
pub struct SDA_DUTY_R(crate::FieldReader<u32, u32>);
impl SDA_DUTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SDA_DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_DUTY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_DUTY` writer - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
pub struct SDA_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
    #[inline(always)]
    pub fn sda_duty(&self) -> SDA_DUTY_R {
        SDA_DUTY_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL"]
    #[inline(always)]
    pub fn sda_duty(&mut self) -> SDA_DUTY_W {
        SDA_DUTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_duty](index.html) module"]
pub struct SDA_DUTY_SPEC;
impl crate::RegisterSpec for SDA_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda_duty::R](R) reader structure"]
impl crate::Readable for SDA_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda_duty::W](W) writer structure"]
impl crate::Writable for SDA_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDA_DUTY to value 0"]
impl crate::Resettable for SDA_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
