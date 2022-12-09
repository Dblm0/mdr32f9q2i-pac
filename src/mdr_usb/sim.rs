#[doc = "Register `SIM` reader"]
pub struct R(crate::R<SIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIM` writer"]
pub struct W(crate::W<SIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIM_SPEC>;
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
impl From<crate::W<SIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTDONEIE` reader - "]
pub type SCTDONEIE_R = crate::BitReader<bool>;
#[doc = "Field `SCTDONEIE` writer - "]
pub type SCTDONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIM_SPEC, bool, O>;
#[doc = "Field `SCRESUMEIE` reader - "]
pub type SCRESUMEIE_R = crate::BitReader<bool>;
#[doc = "Field `SCRESUMEIE` writer - "]
pub type SCRESUMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIM_SPEC, bool, O>;
#[doc = "Field `SCRESETEVIE` reader - "]
pub type SCRESETEVIE_R = crate::BitReader<bool>;
#[doc = "Field `SCRESETEVIE` writer - "]
pub type SCRESETEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIM_SPEC, bool, O>;
#[doc = "Field `SCSOFRECIE` reader - "]
pub type SCSOFRECIE_R = crate::BitReader<bool>;
#[doc = "Field `SCSOFRECIE` writer - "]
pub type SCSOFRECIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIM_SPEC, bool, O>;
#[doc = "Field `SCNAKSENTIE` reader - "]
pub type SCNAKSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `SCNAKSENTIE` writer - "]
pub type SCNAKSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sctdoneie(&self) -> SCTDONEIE_R {
        SCTDONEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresumeie(&self) -> SCRESUMEIE_R {
        SCRESUMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetevie(&self) -> SCRESETEVIE_R {
        SCRESETEVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrecie(&self) -> SCSOFRECIE_R {
        SCSOFRECIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksentie(&self) -> SCNAKSENTIE_R {
        SCNAKSENTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sctdoneie(&mut self) -> SCTDONEIE_W<0> {
        SCTDONEIE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn scresumeie(&mut self) -> SCRESUMEIE_W<1> {
        SCRESUMEIE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn scresetevie(&mut self) -> SCRESETEVIE_W<2> {
        SCRESETEVIE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn scsofrecie(&mut self) -> SCSOFRECIE_W<3> {
        SCSOFRECIE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn scnaksentie(&mut self) -> SCNAKSENTIE_W<4> {
        SCNAKSENTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SIM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sim](index.html) module"]
pub struct SIM_SPEC;
impl crate::RegisterSpec for SIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sim::R](R) reader structure"]
impl crate::Readable for SIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sim::W](W) writer structure"]
impl crate::Writable for SIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIM to value 0"]
impl crate::Resettable for SIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
