#[doc = "Register `ERR_CLR` reader"]
pub struct R(crate::R<ERR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR_CLR` writer"]
pub struct W(crate::W<ERR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_CLR_SPEC>;
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
impl From<crate::W<ERR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR_CLR` reader - "]
pub type ERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CLR` writer - "]
pub type ERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERR_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn err_clr(&self) -> ERR_CLR_R {
        ERR_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn err_clr(&mut self) -> ERR_CLR_W<0> {
        ERR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err_clr](index.html) module"]
pub struct ERR_CLR_SPEC;
impl crate::RegisterSpec for ERR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err_clr::R](R) reader structure"]
impl crate::Readable for ERR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err_clr::W](W) writer structure"]
impl crate::Writable for ERR_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERR_CLR to value 0"]
impl crate::Resettable for ERR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}