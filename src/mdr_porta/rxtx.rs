#[doc = "Register `RXTX` reader"]
pub struct R(crate::R<RXTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXTX` writer"]
pub struct W(crate::W<RXTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXTX_SPEC>;
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
impl From<crate::W<RXTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTX_0` reader - "]
pub type RXTX_0_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_0` writer - "]
pub type RXTX_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_1` reader - "]
pub type RXTX_1_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_1` writer - "]
pub type RXTX_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_2` reader - "]
pub type RXTX_2_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_2` writer - "]
pub type RXTX_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_3` reader - "]
pub type RXTX_3_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_3` writer - "]
pub type RXTX_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_4` reader - "]
pub type RXTX_4_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_4` writer - "]
pub type RXTX_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_5` reader - "]
pub type RXTX_5_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_5` writer - "]
pub type RXTX_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_6` reader - "]
pub type RXTX_6_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_6` writer - "]
pub type RXTX_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_7` reader - "]
pub type RXTX_7_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_7` writer - "]
pub type RXTX_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_8` reader - "]
pub type RXTX_8_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_8` writer - "]
pub type RXTX_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_9` reader - "]
pub type RXTX_9_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_9` writer - "]
pub type RXTX_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_10` reader - "]
pub type RXTX_10_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_10` writer - "]
pub type RXTX_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_11` reader - "]
pub type RXTX_11_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_11` writer - "]
pub type RXTX_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_12` reader - "]
pub type RXTX_12_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_12` writer - "]
pub type RXTX_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_13` reader - "]
pub type RXTX_13_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_13` writer - "]
pub type RXTX_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_14` reader - "]
pub type RXTX_14_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_14` writer - "]
pub type RXTX_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
#[doc = "Field `RXTX_15` reader - "]
pub type RXTX_15_R = crate::BitReader<bool>;
#[doc = "Field `RXTX_15` writer - "]
pub type RXTX_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXTX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxtx_0(&self) -> RXTX_0_R {
        RXTX_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxtx_1(&self) -> RXTX_1_R {
        RXTX_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxtx_2(&self) -> RXTX_2_R {
        RXTX_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxtx_3(&self) -> RXTX_3_R {
        RXTX_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxtx_4(&self) -> RXTX_4_R {
        RXTX_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxtx_5(&self) -> RXTX_5_R {
        RXTX_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxtx_6(&self) -> RXTX_6_R {
        RXTX_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rxtx_7(&self) -> RXTX_7_R {
        RXTX_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxtx_8(&self) -> RXTX_8_R {
        RXTX_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rxtx_9(&self) -> RXTX_9_R {
        RXTX_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rxtx_10(&self) -> RXTX_10_R {
        RXTX_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rxtx_11(&self) -> RXTX_11_R {
        RXTX_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxtx_12(&self) -> RXTX_12_R {
        RXTX_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rxtx_13(&self) -> RXTX_13_R {
        RXTX_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rxtx_14(&self) -> RXTX_14_R {
        RXTX_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxtx_15(&self) -> RXTX_15_R {
        RXTX_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_0(&mut self) -> RXTX_0_W<0> {
        RXTX_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_1(&mut self) -> RXTX_1_W<1> {
        RXTX_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_2(&mut self) -> RXTX_2_W<2> {
        RXTX_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_3(&mut self) -> RXTX_3_W<3> {
        RXTX_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_4(&mut self) -> RXTX_4_W<4> {
        RXTX_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_5(&mut self) -> RXTX_5_W<5> {
        RXTX_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_6(&mut self) -> RXTX_6_W<6> {
        RXTX_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_7(&mut self) -> RXTX_7_W<7> {
        RXTX_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_8(&mut self) -> RXTX_8_W<8> {
        RXTX_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_9(&mut self) -> RXTX_9_W<9> {
        RXTX_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_10(&mut self) -> RXTX_10_W<10> {
        RXTX_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_11(&mut self) -> RXTX_11_W<11> {
        RXTX_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_12(&mut self) -> RXTX_12_W<12> {
        RXTX_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_13(&mut self) -> RXTX_13_W<13> {
        RXTX_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_14(&mut self) -> RXTX_14_W<14> {
        RXTX_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rxtx_15(&mut self) -> RXTX_15_W<15> {
        RXTX_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtx](index.html) module"]
pub struct RXTX_SPEC;
impl crate::RegisterSpec for RXTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtx::R](R) reader structure"]
impl crate::Readable for RXTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxtx::W](W) writer structure"]
impl crate::Writable for RXTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXTX to value 0"]
impl crate::Resettable for RXTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
