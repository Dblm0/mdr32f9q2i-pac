#[doc = "Register `HFN_H` reader"]
pub struct R(crate::R<HFN_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFN_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFN_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFN_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFN_H` writer"]
pub struct W(crate::W<HFN_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFN_H_SPEC>;
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
impl From<crate::W<HFN_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFN_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FNUM` reader - "]
pub type FNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FNUM` writer - "]
pub type FNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFN_H_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<0> {
        FNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfn_h](index.html) module"]
pub struct HFN_H_SPEC;
impl crate::RegisterSpec for HFN_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfn_h::R](R) reader structure"]
impl crate::Readable for HFN_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfn_h::W](W) writer structure"]
impl crate::Writable for HFN_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFN_H to value 0"]
impl crate::Resettable for HFN_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
