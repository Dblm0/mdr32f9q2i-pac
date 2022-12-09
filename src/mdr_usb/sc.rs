#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCGEN` reader - "]
pub type SCGEN_R = crate::BitReader<bool>;
#[doc = "Field `SCGEN` writer - "]
pub type SCGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `SCTXLS` reader - "]
pub type SCTXLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCTXLS` writer - "]
pub type SCTXLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCDC` reader - "]
pub type SCDC_R = crate::BitReader<bool>;
#[doc = "Field `SCDC` writer - "]
pub type SCDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `SCFSP` reader - "]
pub type SCFSP_R = crate::BitReader<bool>;
#[doc = "Field `SCFSP` writer - "]
pub type SCFSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `SCFSR` reader - "]
pub type SCFSR_R = crate::BitReader<bool>;
#[doc = "Field `SCFSR` writer - "]
pub type SCFSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scgen(&self) -> SCGEN_R {
        SCGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sctxls(&self) -> SCTXLS_R {
        SCTXLS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scdc(&self) -> SCDC_R {
        SCDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scfsp(&self) -> SCFSP_R {
        SCFSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scfsr(&self) -> SCFSR_R {
        SCFSR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn scgen(&mut self) -> SCGEN_W<0> {
        SCGEN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn sctxls(&mut self) -> SCTXLS_W<1> {
        SCTXLS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn scdc(&mut self) -> SCDC_W<3> {
        SCDC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn scfsp(&mut self) -> SCFSP_W<4> {
        SCFSP_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn scfsr(&mut self) -> SCFSR_W<5> {
        SCFSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
