#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - "]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `CTS` writer - "]
pub type CTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `DSR` reader - "]
pub type DSR_R = crate::BitReader<bool>;
#[doc = "Field `DSR` writer - "]
pub type DSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `DCD` reader - "]
pub type DCD_R = crate::BitReader<bool>;
#[doc = "Field `DCD` writer - "]
pub type DCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - "]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - "]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `RXFE` reader - "]
pub type RXFE_R = crate::BitReader<bool>;
#[doc = "Field `RXFE` writer - "]
pub type RXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `TXFF` reader - "]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` writer - "]
pub type TXFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `RXFF` reader - "]
pub type RXFF_R = crate::BitReader<bool>;
#[doc = "Field `RXFF` writer - "]
pub type RXFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `TXFE` reader - "]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `TXFE` writer - "]
pub type TXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
#[doc = "Field `RI` reader - "]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - "]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<0> {
        CTS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DSR_W<1> {
        DSR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DCD_W<2> {
        DCD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<3> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RXFE_W<4> {
        RXFE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TXFF_W<5> {
        TXFF_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rxff(&mut self) -> RXFF_W<6> {
        RXFF_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TXFE_W<7> {
        TXFE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<8> {
        RI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
