#[doc = "Register `RTC_DIV` reader"]
pub struct R(crate::R<RTC_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DIV` writer"]
pub struct W(crate::W<RTC_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DIV_SPEC>;
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
impl From<crate::W<RTC_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DIV` reader - "]
pub type RTC_DIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_DIV` writer - "]
pub type RTC_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_DIV_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RTC_DIV_R {
        RTC_DIV_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_div(&mut self) -> RTC_DIV_W<0> {
        RTC_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Realtime Prescaler Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_div](index.html) module"]
pub struct RTC_DIV_SPEC;
impl crate::RegisterSpec for RTC_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_div::R](R) reader structure"]
impl crate::Readable for RTC_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_div::W](W) writer structure"]
impl crate::Writable for RTC_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_DIV to value 0"]
impl crate::Resettable for RTC_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
