#[doc = "Register `BLK2_WDATA2` reader"]
pub struct R(crate::R<BLK2_WDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK2_WDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK2_WDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK2_WDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK2_WDATA2` writer"]
pub struct W(crate::W<BLK2_WDATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK2_WDATA2_SPEC>;
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
impl From<crate::W<BLK2_WDATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK2_WDATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK2_DIN2` reader - program for BLOCK2"]
pub struct BLK2_DIN2_R(crate::FieldReader<u32, u32>);
impl BLK2_DIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK2_DIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK2_DIN2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK2_DIN2` writer - program for BLOCK2"]
pub struct BLK2_DIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK2_DIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK2"]
    #[inline(always)]
    pub fn blk2_din2(&self) -> BLK2_DIN2_R {
        BLK2_DIN2_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK2"]
    #[inline(always)]
    pub fn blk2_din2(&mut self) -> BLK2_DIN2_W {
        BLK2_DIN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk2_wdata2](index.html) module"]
pub struct BLK2_WDATA2_SPEC;
impl crate::RegisterSpec for BLK2_WDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk2_wdata2::R](R) reader structure"]
impl crate::Readable for BLK2_WDATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk2_wdata2::W](W) writer structure"]
impl crate::Writable for BLK2_WDATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK2_WDATA2 to value 0"]
impl crate::Resettable for BLK2_WDATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
