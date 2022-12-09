#[doc = "Register `HRXS` reader"]
pub struct R(crate::R<HRXS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXS` writer"]
pub struct W(crate::W<HRXS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXS_SPEC>;
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
impl From<crate::W<HRXS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCERR` reader - "]
pub type CRCERR_R = crate::BitReader<bool>;
#[doc = "Field `CRCERR` writer - "]
pub type CRCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `BSERR` reader - "]
pub type BSERR_R = crate::BitReader<bool>;
#[doc = "Field `BSERR` writer - "]
pub type BSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `RXOF` reader - "]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - "]
pub type RXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `RXTO` reader - "]
pub type RXTO_R = crate::BitReader<bool>;
#[doc = "Field `RXTO` writer - "]
pub type RXTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `NAKRXED` reader - "]
pub type NAKRXED_R = crate::BitReader<bool>;
#[doc = "Field `NAKRXED` writer - "]
pub type NAKRXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `STALLRXED` reader - "]
pub type STALLRXED_R = crate::BitReader<bool>;
#[doc = "Field `STALLRXED` writer - "]
pub type STALLRXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `ACKRXED` reader - "]
pub type ACKRXED_R = crate::BitReader<bool>;
#[doc = "Field `ACKRXED` writer - "]
pub type ACKRXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
#[doc = "Field `DATASEQ` reader - "]
pub type DATASEQ_R = crate::BitReader<bool>;
#[doc = "Field `DATASEQ` writer - "]
pub type DATASEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRXS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bserr(&self) -> BSERR_R {
        BSERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nakrxed(&self) -> NAKRXED_R {
        NAKRXED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stallrxed(&self) -> STALLRXED_R {
        STALLRXED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ackrxed(&self) -> ACKRXED_R {
        ACKRXED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dataseq(&self) -> DATASEQ_R {
        DATASEQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<0> {
        CRCERR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bserr(&mut self) -> BSERR_W<1> {
        BSERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<2> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxto(&mut self) -> RXTO_W<3> {
        RXTO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nakrxed(&mut self) -> NAKRXED_W<4> {
        NAKRXED_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stallrxed(&mut self) -> STALLRXED_W<5> {
        STALLRXED_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ackrxed(&mut self) -> ACKRXED_W<6> {
        ACKRXED_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dataseq(&mut self) -> DATASEQ_W<7> {
        DATASEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HRXS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxs](index.html) module"]
pub struct HRXS_SPEC;
impl crate::RegisterSpec for HRXS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxs::R](R) reader structure"]
impl crate::Readable for HRXS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxs::W](W) writer structure"]
impl crate::Writable for HRXS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRXS to value 0"]
impl crate::Resettable for HRXS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
