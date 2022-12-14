#[doc = "Register `SFN_H` reader"]
pub struct R(crate::R<SFN_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFN_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFN_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFN_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFN_H` writer"]
pub struct W(crate::W<SFN_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFN_H_SPEC>;
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
impl From<crate::W<SFN_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFN_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMENUM` reader - "]
pub type FRAMENUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAMENUM` writer - "]
pub type FRAMENUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFN_H_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn framenum(&self) -> FRAMENUM_R {
        FRAMENUM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn framenum(&mut self) -> FRAMENUM_W<0> {
        FRAMENUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfn_h](index.html) module"]
pub struct SFN_H_SPEC;
impl crate::RegisterSpec for SFN_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfn_h::R](R) reader structure"]
impl crate::Readable for SFN_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfn_h::W](W) writer structure"]
impl crate::Writable for SFN_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFN_H to value 0"]
impl crate::Resettable for SFN_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
