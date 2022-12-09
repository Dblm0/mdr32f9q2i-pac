#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIRMIS` reader - "]
pub type RIRMIS_R = crate::BitReader<bool>;
#[doc = "Field `RIRMIS` writer - "]
pub type RIRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `CTSRMIS` reader - "]
pub type CTSRMIS_R = crate::BitReader<bool>;
#[doc = "Field `CTSRMIS` writer - "]
pub type CTSRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `DCDRMIS` reader - "]
pub type DCDRMIS_R = crate::BitReader<bool>;
#[doc = "Field `DCDRMIS` writer - "]
pub type DCDRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `DSRRMIS` reader - "]
pub type DSRRMIS_R = crate::BitReader<bool>;
#[doc = "Field `DSRRMIS` writer - "]
pub type DSRRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RXRIS` reader - "]
pub type RXRIS_R = crate::BitReader<bool>;
#[doc = "Field `RXRIS` writer - "]
pub type RXRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `TXRIS` reader - "]
pub type TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `TXRIS` writer - "]
pub type TXRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `RTRIS` reader - "]
pub type RTRIS_R = crate::BitReader<bool>;
#[doc = "Field `RTRIS` writer - "]
pub type RTRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `FERIS` reader - "]
pub type FERIS_R = crate::BitReader<bool>;
#[doc = "Field `FERIS` writer - "]
pub type FERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `PERIS` reader - "]
pub type PERIS_R = crate::BitReader<bool>;
#[doc = "Field `PERIS` writer - "]
pub type PERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `BERIS` reader - "]
pub type BERIS_R = crate::BitReader<bool>;
#[doc = "Field `BERIS` writer - "]
pub type BERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
#[doc = "Field `OERIS` reader - "]
pub type OERIS_R = crate::BitReader<bool>;
#[doc = "Field `OERIS` writer - "]
pub type OERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rirmis(&self) -> RIRMIS_R {
        RIRMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CTSRMIS_R {
        CTSRMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DCDRMIS_R {
        DCDRMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DSRRMIS_R {
        DSRRMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rirmis(&mut self) -> RIRMIS_W<0> {
        RIRMIS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctsrmis(&mut self) -> CTSRMIS_W<1> {
        CTSRMIS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcdrmis(&mut self) -> DCDRMIS_W<2> {
        DCDRMIS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dsrrmis(&mut self) -> DSRRMIS_W<3> {
        DSRRMIS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxris(&mut self) -> RXRIS_W<4> {
        RXRIS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn txris(&mut self) -> TXRIS_W<5> {
        TXRIS_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rtris(&mut self) -> RTRIS_W<6> {
        RTRIS_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn feris(&mut self) -> FERIS_W<7> {
        FERIS_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn peris(&mut self) -> PERIS_W<8> {
        PERIS_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn beris(&mut self) -> BERIS_W<9> {
        BERIS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn oeris(&mut self) -> OERIS_W<10> {
        OERIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
