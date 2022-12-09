#[doc = "Register `HSCR` reader"]
pub struct R(crate::R<HSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCR` writer"]
pub struct W(crate::W<HSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCR_SPEC>;
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
impl From<crate::W<HSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_MODE` reader - "]
pub type HOST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `HOST_MODE` writer - "]
pub type HOST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `RESET_CORE` reader - "]
pub type RESET_CORE_R = crate::BitReader<bool>;
#[doc = "Field `RESET_CORE` writer - "]
pub type RESET_CORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `EN_TX` reader - "]
pub type EN_TX_R = crate::BitReader<bool>;
#[doc = "Field `EN_TX` writer - "]
pub type EN_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `EN_RX` reader - "]
pub type EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `EN_RX` writer - "]
pub type EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `DP_PULLUP` reader - "]
pub type DP_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `DP_PULLUP` writer - "]
pub type DP_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `DP_PULLDOWN` reader - "]
pub type DP_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `DP_PULLDOWN` writer - "]
pub type DP_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `DM_PULLUP` reader - "]
pub type DM_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `DM_PULLUP` writer - "]
pub type DM_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
#[doc = "Field `DM_PULLDOWN` reader - "]
pub type DM_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `DM_PULLDOWN` writer - "]
pub type DM_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_mode(&self) -> HOST_MODE_R {
        HOST_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reset_core(&self) -> RESET_CORE_R {
        RESET_CORE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_tx(&self) -> EN_TX_R {
        EN_TX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn en_rx(&self) -> EN_RX_R {
        EN_RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn host_mode(&mut self) -> HOST_MODE_W<0> {
        HOST_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reset_core(&mut self) -> RESET_CORE_W<1> {
        RESET_CORE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn en_tx(&mut self) -> EN_TX_W<2> {
        EN_TX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn en_rx(&mut self) -> EN_RX_W<3> {
        EN_RX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W<4> {
        DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W<5> {
        DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W<6> {
        DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W<7> {
        DM_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HSCR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscr](index.html) module"]
pub struct HSCR_SPEC;
impl crate::RegisterSpec for HSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hscr::R](R) reader structure"]
impl crate::Readable for HSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hscr::W](W) writer structure"]
impl crate::Writable for HSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCR to value 0"]
impl crate::Resettable for HSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
