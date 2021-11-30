#[doc = "Register `TX_STATUS` reader"]
pub struct R(crate::R<TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_TX_FULL` reader - "]
pub struct SLC0_TX_FULL_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_EMPTY` reader - "]
pub struct SLC0_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_FULL` reader - "]
pub struct SLC1_TX_FULL_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_EMPTY` reader - "]
pub struct SLC1_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_full(&self) -> SLC0_TX_FULL_R {
        SLC0_TX_FULL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_empty(&self) -> SLC0_TX_EMPTY_R {
        SLC0_TX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_full(&self) -> SLC1_TX_FULL_R {
        SLC1_TX_FULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_empty(&self) -> SLC1_TX_EMPTY_R {
        SLC1_TX_EMPTY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_status](index.html) module"]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_status::R](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_STATUS to value 0x0002_0002"]
impl crate::Resettable for TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0002
    }
}
