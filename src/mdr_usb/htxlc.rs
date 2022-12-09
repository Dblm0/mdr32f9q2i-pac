#[doc = "Register `HTXLC` reader"]
pub struct R(crate::R<HTXLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXLC` writer"]
pub struct W(crate::W<HTXLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXLC_SPEC>;
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
impl From<crate::W<HTXLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXLC` reader - "]
pub type TXLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXLC` writer - "]
pub type TXLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTXLC_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC` reader - "]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - "]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXLC_SPEC, bool, O>;
#[doc = "Field `FSPL` reader - "]
pub type FSPL_R = crate::BitReader<bool>;
#[doc = "Field `FSPL` writer - "]
pub type FSPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXLC_SPEC, bool, O>;
#[doc = "Field `FSLR` reader - "]
pub type FSLR_R = crate::BitReader<bool>;
#[doc = "Field `FSLR` writer - "]
pub type FSLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXLC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txlc(&self) -> TXLC_R {
        TXLC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fspl(&self) -> FSPL_R {
        FSPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fslr(&self) -> FSLR_R {
        FSLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn txlc(&mut self) -> TXLC_W<0> {
        TXLC_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<2> {
        DC_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fspl(&mut self) -> FSPL_W<3> {
        FSPL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fslr(&mut self) -> FSLR_W<4> {
        FSLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXLC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxlc](index.html) module"]
pub struct HTXLC_SPEC;
impl crate::RegisterSpec for HTXLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxlc::R](R) reader structure"]
impl crate::Readable for HTXLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxlc::W](W) writer structure"]
impl crate::Writable for HTXLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXLC to value 0"]
impl crate::Resettable for HTXLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
