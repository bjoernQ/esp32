#[doc = "Register `SW_CPU_STALL` reader"]
pub struct R(crate::R<SW_CPU_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CPU_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CPU_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CPU_STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CPU_STALL` writer"]
pub struct W(crate::W<SW_CPU_STALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CPU_STALL_SPEC>;
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
impl From<crate::W<SW_CPU_STALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CPU_STALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_APPCPU_C1` reader - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C1_R(crate::FieldReader<u8, u8>);
impl SW_STALL_APPCPU_C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_APPCPU_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_APPCPU_C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_APPCPU_C1` writer - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
#[doc = "Field `SW_STALL_PROCPU_C1` reader - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C1_R(crate::FieldReader<u8, u8>);
impl SW_STALL_PROCPU_C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_PROCPU_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_PROCPU_C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_PROCPU_C1` writer - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&self) -> SW_STALL_APPCPU_C1_R {
        SW_STALL_APPCPU_C1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&self) -> SW_STALL_PROCPU_C1_R {
        SW_STALL_PROCPU_C1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&mut self) -> SW_STALL_APPCPU_C1_W {
        SW_STALL_APPCPU_C1_W { w: self }
    }
    #[doc = "Bits 26:31 - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&mut self) -> SW_STALL_PROCPU_C1_W {
        SW_STALL_PROCPU_C1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cpu_stall](index.html) module"]
pub struct SW_CPU_STALL_SPEC;
impl crate::RegisterSpec for SW_CPU_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_cpu_stall::R](R) reader structure"]
impl crate::Readable for SW_CPU_STALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_cpu_stall::W](W) writer structure"]
impl crate::Writable for SW_CPU_STALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CPU_STALL to value 0"]
impl crate::Resettable for SW_CPU_STALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
