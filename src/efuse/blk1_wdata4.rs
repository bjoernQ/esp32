#[doc = "Register `BLK1_WDATA4` reader"]
pub struct R(crate::R<BLK1_WDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_WDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_WDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_WDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK1_WDATA4` writer"]
pub struct W(crate::W<BLK1_WDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK1_WDATA4_SPEC>;
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
impl From<crate::W<BLK1_WDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK1_WDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK1_DIN4` reader - program for BLOCK1"]
pub struct BLK1_DIN4_R(crate::FieldReader<u32, u32>);
impl BLK1_DIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK1_DIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK1_DIN4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK1_DIN4` writer - program for BLOCK1"]
pub struct BLK1_DIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK1_DIN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK1"]
    #[inline(always)]
    pub fn blk1_din4(&self) -> BLK1_DIN4_R {
        BLK1_DIN4_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK1"]
    #[inline(always)]
    pub fn blk1_din4(&mut self) -> BLK1_DIN4_W {
        BLK1_DIN4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_wdata4](index.html) module"]
pub struct BLK1_WDATA4_SPEC;
impl crate::RegisterSpec for BLK1_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_wdata4::R](R) reader structure"]
impl crate::Readable for BLK1_WDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk1_wdata4::W](W) writer structure"]
impl crate::Writable for BLK1_WDATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK1_WDATA4 to value 0"]
impl crate::Resettable for BLK1_WDATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
