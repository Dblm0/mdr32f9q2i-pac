#[doc = "Register `HIM` reader"]
pub struct R(crate::R<HIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIM` writer"]
pub struct W(crate::W<HIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIM_SPEC>;
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
impl From<crate::W<HIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDONEIE` reader - "]
pub type TDONEIE_R = crate::BitReader<bool>;
#[doc = "Field `TDONEIE` writer - "]
pub type TDONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIM_SPEC, bool, O>;
#[doc = "Field `RESUMEIE` reader - "]
pub type RESUMEIE_R = crate::BitReader<bool>;
#[doc = "Field `RESUMEIE` writer - "]
pub type RESUMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIM_SPEC, bool, O>;
#[doc = "Field `CONEVIE` reader - "]
pub type CONEVIE_R = crate::BitReader<bool>;
#[doc = "Field `CONEVIE` writer - "]
pub type CONEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIM_SPEC, bool, O>;
#[doc = "Field `SOFIE` reader - "]
pub type SOFIE_R = crate::BitReader<bool>;
#[doc = "Field `SOFIE` writer - "]
pub type SOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoneie(&self) -> TDONEIE_R {
        TDONEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conevie(&self) -> CONEVIE_R {
        CONEVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdoneie(&mut self) -> TDONEIE_W<0> {
        TDONEIE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn resumeie(&mut self) -> RESUMEIE_W<1> {
        RESUMEIE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn conevie(&mut self) -> CONEVIE_W<2> {
        CONEVIE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<3> {
        SOFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HIM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [him](index.html) module"]
pub struct HIM_SPEC;
impl crate::RegisterSpec for HIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [him::R](R) reader structure"]
impl crate::Readable for HIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [him::W](W) writer structure"]
impl crate::Writable for HIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIM to value 0"]
impl crate::Resettable for HIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
