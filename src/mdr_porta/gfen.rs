#[doc = "Register `GFEN` reader"]
pub struct R(crate::R<GFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFEN` writer"]
pub struct W(crate::W<GFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFEN_SPEC>;
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
impl From<crate::W<GFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GFEN_0` reader - "]
pub type GFEN_0_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_0` writer - "]
pub type GFEN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_1` reader - "]
pub type GFEN_1_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_1` writer - "]
pub type GFEN_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_2` reader - "]
pub type GFEN_2_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_2` writer - "]
pub type GFEN_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_3` reader - "]
pub type GFEN_3_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_3` writer - "]
pub type GFEN_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_4` reader - "]
pub type GFEN_4_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_4` writer - "]
pub type GFEN_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_5` reader - "]
pub type GFEN_5_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_5` writer - "]
pub type GFEN_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_6` reader - "]
pub type GFEN_6_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_6` writer - "]
pub type GFEN_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_7` reader - "]
pub type GFEN_7_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_7` writer - "]
pub type GFEN_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_8` reader - "]
pub type GFEN_8_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_8` writer - "]
pub type GFEN_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_9` reader - "]
pub type GFEN_9_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_9` writer - "]
pub type GFEN_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_10` reader - "]
pub type GFEN_10_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_10` writer - "]
pub type GFEN_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_11` reader - "]
pub type GFEN_11_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_11` writer - "]
pub type GFEN_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_12` reader - "]
pub type GFEN_12_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_12` writer - "]
pub type GFEN_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_13` reader - "]
pub type GFEN_13_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_13` writer - "]
pub type GFEN_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_14` reader - "]
pub type GFEN_14_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_14` writer - "]
pub type GFEN_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
#[doc = "Field `GFEN_15` reader - "]
pub type GFEN_15_R = crate::BitReader<bool>;
#[doc = "Field `GFEN_15` writer - "]
pub type GFEN_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gfen_0(&self) -> GFEN_0_R {
        GFEN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gfen_1(&self) -> GFEN_1_R {
        GFEN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gfen_2(&self) -> GFEN_2_R {
        GFEN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gfen_3(&self) -> GFEN_3_R {
        GFEN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gfen_4(&self) -> GFEN_4_R {
        GFEN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gfen_5(&self) -> GFEN_5_R {
        GFEN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gfen_6(&self) -> GFEN_6_R {
        GFEN_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gfen_7(&self) -> GFEN_7_R {
        GFEN_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gfen_8(&self) -> GFEN_8_R {
        GFEN_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gfen_9(&self) -> GFEN_9_R {
        GFEN_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gfen_10(&self) -> GFEN_10_R {
        GFEN_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gfen_11(&self) -> GFEN_11_R {
        GFEN_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gfen_12(&self) -> GFEN_12_R {
        GFEN_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gfen_13(&self) -> GFEN_13_R {
        GFEN_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gfen_14(&self) -> GFEN_14_R {
        GFEN_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gfen_15(&self) -> GFEN_15_R {
        GFEN_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_0(&mut self) -> GFEN_0_W<0> {
        GFEN_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_1(&mut self) -> GFEN_1_W<1> {
        GFEN_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_2(&mut self) -> GFEN_2_W<2> {
        GFEN_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_3(&mut self) -> GFEN_3_W<3> {
        GFEN_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_4(&mut self) -> GFEN_4_W<4> {
        GFEN_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_5(&mut self) -> GFEN_5_W<5> {
        GFEN_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_6(&mut self) -> GFEN_6_W<6> {
        GFEN_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_7(&mut self) -> GFEN_7_W<7> {
        GFEN_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_8(&mut self) -> GFEN_8_W<8> {
        GFEN_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_9(&mut self) -> GFEN_9_W<9> {
        GFEN_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_10(&mut self) -> GFEN_10_W<10> {
        GFEN_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_11(&mut self) -> GFEN_11_W<11> {
        GFEN_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_12(&mut self) -> GFEN_12_W<12> {
        GFEN_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_13(&mut self) -> GFEN_13_W<13> {
        GFEN_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_14(&mut self) -> GFEN_14_W<14> {
        GFEN_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gfen_15(&mut self) -> GFEN_15_W<15> {
        GFEN_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfen](index.html) module"]
pub struct GFEN_SPEC;
impl crate::RegisterSpec for GFEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfen::R](R) reader structure"]
impl crate::Readable for GFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfen::W](W) writer structure"]
impl crate::Writable for GFEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GFEN to value 0"]
impl crate::Resettable for GFEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
