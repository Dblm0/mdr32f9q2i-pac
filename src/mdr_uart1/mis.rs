#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMMIS` reader - "]
pub type RIMMIS_R = crate::BitReader<bool>;
#[doc = "Field `RIMMIS` writer - "]
pub type RIMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `CTSMMIS` reader - "]
pub type CTSMMIS_R = crate::BitReader<bool>;
#[doc = "Field `CTSMMIS` writer - "]
pub type CTSMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `DCDMMIS` reader - "]
pub type DCDMMIS_R = crate::BitReader<bool>;
#[doc = "Field `DCDMMIS` writer - "]
pub type DCDMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `DSRMMIS` reader - "]
pub type DSRMMIS_R = crate::BitReader<bool>;
#[doc = "Field `DSRMMIS` writer - "]
pub type DSRMMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RXMIS` reader - "]
pub type RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `RXMIS` writer - "]
pub type RXMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `TXMIS` reader - "]
pub type TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMIS` writer - "]
pub type TXMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `RTMIS` reader - "]
pub type RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `RTMIS` writer - "]
pub type RTMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `FEMIS` reader - "]
pub type FEMIS_R = crate::BitReader<bool>;
#[doc = "Field `FEMIS` writer - "]
pub type FEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `PEMIS` reader - "]
pub type PEMIS_R = crate::BitReader<bool>;
#[doc = "Field `PEMIS` writer - "]
pub type PEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `BEMIS` reader - "]
pub type BEMIS_R = crate::BitReader<bool>;
#[doc = "Field `BEMIS` writer - "]
pub type BEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
#[doc = "Field `OEMIS` reader - "]
pub type OEMIS_R = crate::BitReader<bool>;
#[doc = "Field `OEMIS` writer - "]
pub type OEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rimmis(&mut self) -> RIMMIS_W<0> {
        RIMMIS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmmis(&mut self) -> CTSMMIS_W<1> {
        CTSMMIS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmmis(&mut self) -> DCDMMIS_W<2> {
        DCDMMIS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmmis(&mut self) -> DSRMMIS_W<3> {
        DSRMMIS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxmis(&mut self) -> RXMIS_W<4> {
        RXMIS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn txmis(&mut self) -> TXMIS_W<5> {
        TXMIS_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rtmis(&mut self) -> RTMIS_W<6> {
        RTMIS_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn femis(&mut self) -> FEMIS_W<7> {
        FEMIS_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pemis(&mut self) -> PEMIS_W<8> {
        PEMIS_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bemis(&mut self) -> BEMIS_W<9> {
        BEMIS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn oemis(&mut self) -> OEMIS_W<10> {
        OEMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Masked Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
