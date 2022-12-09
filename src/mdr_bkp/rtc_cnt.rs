#[doc = "Register `RTC_CNT` reader"]
pub struct R(crate::R<RTC_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNT` writer"]
pub struct W(crate::W<RTC_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNT_SPEC>;
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
impl From<crate::W<RTC_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CNT` reader - "]
pub type RTC_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_CNT` writer - "]
pub type RTC_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_CNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RTC_CNT_R {
        RTC_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cnt(&mut self) -> RTC_CNT_W<0> {
        RTC_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Realtime Clock Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cnt](index.html) module"]
pub struct RTC_CNT_SPEC;
impl crate::RegisterSpec for RTC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cnt::R](R) reader structure"]
impl crate::Readable for RTC_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cnt::W](W) writer structure"]
impl crate::Writable for RTC_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_CNT to value 0"]
impl crate::Resettable for RTC_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}