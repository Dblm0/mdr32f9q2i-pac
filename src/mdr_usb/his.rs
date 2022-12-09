#[doc = "Register `HIS` reader"]
pub struct R(crate::R<HIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIS` writer"]
pub struct W(crate::W<HIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIS_SPEC>;
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
impl From<crate::W<HIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDONE` reader - "]
pub type TDONE_R = crate::BitReader<bool>;
#[doc = "Field `TDONE` writer - "]
pub type TDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIS_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - "]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - "]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIS_SPEC, bool, O>;
#[doc = "Field `CONEV` reader - "]
pub type CONEV_R = crate::BitReader<bool>;
#[doc = "Field `CONEV` writer - "]
pub type CONEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIS_SPEC, bool, O>;
#[doc = "Field `SOFS` reader - "]
pub type SOFS_R = crate::BitReader<bool>;
#[doc = "Field `SOFS` writer - "]
pub type SOFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdone(&self) -> TDONE_R {
        TDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conev(&self) -> CONEV_R {
        CONEV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofs(&self) -> SOFS_R {
        SOFS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdone(&mut self) -> TDONE_W<0> {
        TDONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<1> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn conev(&mut self) -> CONEV_W<2> {
        CONEV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sofs(&mut self) -> SOFS_W<3> {
        SOFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HIS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [his](index.html) module"]
pub struct HIS_SPEC;
impl crate::RegisterSpec for HIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [his::R](R) reader structure"]
impl crate::Readable for HIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [his::W](W) writer structure"]
impl crate::Writable for HIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIS to value 0"]
impl crate::Resettable for HIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
