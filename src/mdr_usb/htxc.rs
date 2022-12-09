#[doc = "Register `HTXC` reader"]
pub struct R(crate::R<HTXC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXC` writer"]
pub struct W(crate::W<HTXC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXC_SPEC>;
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
impl From<crate::W<HTXC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TREQ` reader - "]
pub type TREQ_R = crate::BitReader<bool>;
#[doc = "Field `TREQ` writer - "]
pub type TREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXC_SPEC, bool, O>;
#[doc = "Field `SOFS` reader - "]
pub type SOFS_R = crate::BitReader<bool>;
#[doc = "Field `SOFS` writer - "]
pub type SOFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXC_SPEC, bool, O>;
#[doc = "Field `PREEN` reader - "]
pub type PREEN_R = crate::BitReader<bool>;
#[doc = "Field `PREEN` writer - "]
pub type PREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXC_SPEC, bool, O>;
#[doc = "Field `ISOEN` reader - "]
pub type ISOEN_R = crate::BitReader<bool>;
#[doc = "Field `ISOEN` writer - "]
pub type ISOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HTXC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sofs(&self) -> SOFS_R {
        SOFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn isoen(&self) -> ISOEN_R {
        ISOEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn treq(&mut self) -> TREQ_W<0> {
        TREQ_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sofs(&mut self) -> SOFS_W<1> {
        SOFS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<2> {
        PREEN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn isoen(&mut self) -> ISOEN_W<3> {
        ISOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxc](index.html) module"]
pub struct HTXC_SPEC;
impl crate::RegisterSpec for HTXC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxc::R](R) reader structure"]
impl crate::Readable for HTXC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxc::W](W) writer structure"]
impl crate::Writable for HTXC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTXC to value 0"]
impl crate::Resettable for HTXC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
