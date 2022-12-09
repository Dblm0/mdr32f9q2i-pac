#[doc = "Register `HSVR` reader"]
pub struct R(crate::R<HSVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSVR` writer"]
pub struct W(crate::W<HSVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSVR_SPEC>;
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
impl From<crate::W<HSVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - "]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VERSION` writer - "]
pub type VERSION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSVR_SPEC, u8, u8, 4, O>;
#[doc = "Field `REVISION` reader - "]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` writer - "]
pub type REVISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSVR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn version(&mut self) -> VERSION_W<0> {
        VERSION_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<4> {
        REVISION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HSVR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsvr](index.html) module"]
pub struct HSVR_SPEC;
impl crate::RegisterSpec for HSVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsvr::R](R) reader structure"]
impl crate::Readable for HSVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsvr::W](W) writer structure"]
impl crate::Writable for HSVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSVR to value 0"]
impl crate::Resettable for HSVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
