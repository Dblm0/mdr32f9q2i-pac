#[doc = "Register `RTC_CS` reader"]
pub struct R(crate::R<RTC_CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CS` writer"]
pub struct W(crate::W<RTC_CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CS_SPEC>;
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
impl From<crate::W<RTC_CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OWF` reader - "]
pub type OWF_R = crate::BitReader<bool>;
#[doc = "Field `OWF` writer - "]
pub type OWF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `SECF` reader - "]
pub type SECF_R = crate::BitReader<bool>;
#[doc = "Field `SECF` writer - "]
pub type SECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `ALRF` reader - "]
pub type ALRF_R = crate::BitReader<bool>;
#[doc = "Field `ALRF` writer - "]
pub type ALRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `OWF_IE` reader - "]
pub type OWF_IE_R = crate::BitReader<bool>;
#[doc = "Field `OWF_IE` writer - "]
pub type OWF_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `SECF_IE` reader - "]
pub type SECF_IE_R = crate::BitReader<bool>;
#[doc = "Field `SECF_IE` writer - "]
pub type SECF_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `ALRF_IE` reader - "]
pub type ALRF_IE_R = crate::BitReader<bool>;
#[doc = "Field `ALRF_IE` writer - "]
pub type ALRF_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
#[doc = "Field `WEC` reader - "]
pub type WEC_R = crate::BitReader<bool>;
#[doc = "Field `WEC` writer - "]
pub type WEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_CS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn owf_ie(&self) -> OWF_IE_R {
        OWF_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn secf_ie(&self) -> SECF_IE_R {
        SECF_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alrf_ie(&self) -> ALRF_IE_R {
        ALRF_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wec(&self) -> WEC_R {
        WEC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OWF_W<0> {
        OWF_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SECF_W<1> {
        SECF_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> ALRF_W<2> {
        ALRF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn owf_ie(&mut self) -> OWF_IE_W<3> {
        OWF_IE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn secf_ie(&mut self) -> SECF_IE_W<4> {
        SECF_IE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn alrf_ie(&mut self) -> ALRF_IE_W<5> {
        ALRF_IE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn wec(&mut self) -> WEC_W<6> {
        WEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Realtime clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cs](index.html) module"]
pub struct RTC_CS_SPEC;
impl crate::RegisterSpec for RTC_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cs::R](R) reader structure"]
impl crate::Readable for RTC_CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cs::W](W) writer structure"]
impl crate::Writable for RTC_CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_CS to value 0"]
impl crate::Resettable for RTC_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
