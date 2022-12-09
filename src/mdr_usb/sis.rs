#[doc = "Register `SIS` reader"]
pub struct R(crate::R<SIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIS` writer"]
pub struct W(crate::W<SIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIS_SPEC>;
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
impl From<crate::W<SIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTDONE` reader - "]
pub type SCTDONE_R = crate::BitReader<bool>;
#[doc = "Field `SCTDONE` writer - "]
pub type SCTDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIS_SPEC, bool, O>;
#[doc = "Field `SCRESUME` reader - "]
pub type SCRESUME_R = crate::BitReader<bool>;
#[doc = "Field `SCRESUME` writer - "]
pub type SCRESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIS_SPEC, bool, O>;
#[doc = "Field `SCRESETEV` reader - "]
pub type SCRESETEV_R = crate::BitReader<bool>;
#[doc = "Field `SCRESETEV` writer - "]
pub type SCRESETEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIS_SPEC, bool, O>;
#[doc = "Field `SCSOFREC` reader - "]
pub type SCSOFREC_R = crate::BitReader<bool>;
#[doc = "Field `SCSOFREC` writer - "]
pub type SCSOFREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIS_SPEC, bool, O>;
#[doc = "Field `SCNAKSENT` reader - "]
pub type SCNAKSENT_R = crate::BitReader<bool>;
#[doc = "Field `SCNAKSENT` writer - "]
pub type SCNAKSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sctdone(&self) -> SCTDONE_R {
        SCTDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresume(&self) -> SCRESUME_R {
        SCRESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetev(&self) -> SCRESETEV_R {
        SCRESETEV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrec(&self) -> SCSOFREC_R {
        SCSOFREC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&self) -> SCNAKSENT_R {
        SCNAKSENT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sctdone(&mut self) -> SCTDONE_W<0> {
        SCTDONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn scresume(&mut self) -> SCRESUME_W<1> {
        SCRESUME_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn scresetev(&mut self) -> SCRESETEV_W<2> {
        SCRESETEV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn scsofrec(&mut self) -> SCSOFREC_W<3> {
        SCSOFREC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn scnaksent(&mut self) -> SCNAKSENT_W<4> {
        SCNAKSENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SIS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sis](index.html) module"]
pub struct SIS_SPEC;
impl crate::RegisterSpec for SIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sis::R](R) reader structure"]
impl crate::Readable for SIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sis::W](W) writer structure"]
impl crate::Writable for SIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIS to value 0"]
impl crate::Resettable for SIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
