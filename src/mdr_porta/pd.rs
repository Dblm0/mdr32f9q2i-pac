#[doc = "Register `PD` reader"]
pub struct R(crate::R<PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD` writer"]
pub struct W(crate::W<PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_SPEC>;
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
impl From<crate::W<PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_0` reader - "]
pub type PD_0_R = crate::BitReader<bool>;
#[doc = "Field `PD_0` writer - "]
pub type PD_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_1` reader - "]
pub type PD_1_R = crate::BitReader<bool>;
#[doc = "Field `PD_1` writer - "]
pub type PD_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_2` reader - "]
pub type PD_2_R = crate::BitReader<bool>;
#[doc = "Field `PD_2` writer - "]
pub type PD_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_3` reader - "]
pub type PD_3_R = crate::BitReader<bool>;
#[doc = "Field `PD_3` writer - "]
pub type PD_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_4` reader - "]
pub type PD_4_R = crate::BitReader<bool>;
#[doc = "Field `PD_4` writer - "]
pub type PD_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_5` reader - "]
pub type PD_5_R = crate::BitReader<bool>;
#[doc = "Field `PD_5` writer - "]
pub type PD_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_6` reader - "]
pub type PD_6_R = crate::BitReader<bool>;
#[doc = "Field `PD_6` writer - "]
pub type PD_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_7` reader - "]
pub type PD_7_R = crate::BitReader<bool>;
#[doc = "Field `PD_7` writer - "]
pub type PD_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_8` reader - "]
pub type PD_8_R = crate::BitReader<bool>;
#[doc = "Field `PD_8` writer - "]
pub type PD_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_9` reader - "]
pub type PD_9_R = crate::BitReader<bool>;
#[doc = "Field `PD_9` writer - "]
pub type PD_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_10` reader - "]
pub type PD_10_R = crate::BitReader<bool>;
#[doc = "Field `PD_10` writer - "]
pub type PD_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_11` reader - "]
pub type PD_11_R = crate::BitReader<bool>;
#[doc = "Field `PD_11` writer - "]
pub type PD_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_12` reader - "]
pub type PD_12_R = crate::BitReader<bool>;
#[doc = "Field `PD_12` writer - "]
pub type PD_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_13` reader - "]
pub type PD_13_R = crate::BitReader<bool>;
#[doc = "Field `PD_13` writer - "]
pub type PD_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_14` reader - "]
pub type PD_14_R = crate::BitReader<bool>;
#[doc = "Field `PD_14` writer - "]
pub type PD_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD_15` reader - "]
pub type PD_15_R = crate::BitReader<bool>;
#[doc = "Field `PD_15` writer - "]
pub type PD_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_0` reader - "]
pub type SHM_0_R = crate::BitReader<bool>;
#[doc = "Field `SHM_0` writer - "]
pub type SHM_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_1` reader - "]
pub type SHM_1_R = crate::BitReader<bool>;
#[doc = "Field `SHM_1` writer - "]
pub type SHM_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_2` reader - "]
pub type SHM_2_R = crate::BitReader<bool>;
#[doc = "Field `SHM_2` writer - "]
pub type SHM_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_3` reader - "]
pub type SHM_3_R = crate::BitReader<bool>;
#[doc = "Field `SHM_3` writer - "]
pub type SHM_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_4` reader - "]
pub type SHM_4_R = crate::BitReader<bool>;
#[doc = "Field `SHM_4` writer - "]
pub type SHM_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_5` reader - "]
pub type SHM_5_R = crate::BitReader<bool>;
#[doc = "Field `SHM_5` writer - "]
pub type SHM_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_6` reader - "]
pub type SHM_6_R = crate::BitReader<bool>;
#[doc = "Field `SHM_6` writer - "]
pub type SHM_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_7` reader - "]
pub type SHM_7_R = crate::BitReader<bool>;
#[doc = "Field `SHM_7` writer - "]
pub type SHM_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_8` reader - "]
pub type SHM_8_R = crate::BitReader<bool>;
#[doc = "Field `SHM_8` writer - "]
pub type SHM_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_9` reader - "]
pub type SHM_9_R = crate::BitReader<bool>;
#[doc = "Field `SHM_9` writer - "]
pub type SHM_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_10` reader - "]
pub type SHM_10_R = crate::BitReader<bool>;
#[doc = "Field `SHM_10` writer - "]
pub type SHM_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_11` reader - "]
pub type SHM_11_R = crate::BitReader<bool>;
#[doc = "Field `SHM_11` writer - "]
pub type SHM_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_12` reader - "]
pub type SHM_12_R = crate::BitReader<bool>;
#[doc = "Field `SHM_12` writer - "]
pub type SHM_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_13` reader - "]
pub type SHM_13_R = crate::BitReader<bool>;
#[doc = "Field `SHM_13` writer - "]
pub type SHM_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_14` reader - "]
pub type SHM_14_R = crate::BitReader<bool>;
#[doc = "Field `SHM_14` writer - "]
pub type SHM_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `SHM_15` reader - "]
pub type SHM_15_R = crate::BitReader<bool>;
#[doc = "Field `SHM_15` writer - "]
pub type SHM_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd_0(&self) -> PD_0_R {
        PD_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pd_1(&self) -> PD_1_R {
        PD_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pd_2(&self) -> PD_2_R {
        PD_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pd_3(&self) -> PD_3_R {
        PD_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd_4(&self) -> PD_4_R {
        PD_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pd_5(&self) -> PD_5_R {
        PD_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pd_6(&self) -> PD_6_R {
        PD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pd_7(&self) -> PD_7_R {
        PD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pd_8(&self) -> PD_8_R {
        PD_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pd_9(&self) -> PD_9_R {
        PD_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pd_10(&self) -> PD_10_R {
        PD_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pd_11(&self) -> PD_11_R {
        PD_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pd_12(&self) -> PD_12_R {
        PD_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pd_13(&self) -> PD_13_R {
        PD_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_14(&self) -> PD_14_R {
        PD_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_15(&self) -> PD_15_R {
        PD_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shm_0(&self) -> SHM_0_R {
        SHM_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn shm_1(&self) -> SHM_1_R {
        SHM_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn shm_2(&self) -> SHM_2_R {
        SHM_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn shm_3(&self) -> SHM_3_R {
        SHM_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn shm_4(&self) -> SHM_4_R {
        SHM_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn shm_5(&self) -> SHM_5_R {
        SHM_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn shm_6(&self) -> SHM_6_R {
        SHM_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn shm_7(&self) -> SHM_7_R {
        SHM_7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn shm_8(&self) -> SHM_8_R {
        SHM_8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn shm_9(&self) -> SHM_9_R {
        SHM_9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn shm_10(&self) -> SHM_10_R {
        SHM_10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn shm_11(&self) -> SHM_11_R {
        SHM_11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn shm_12(&self) -> SHM_12_R {
        SHM_12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn shm_13(&self) -> SHM_13_R {
        SHM_13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn shm_14(&self) -> SHM_14_R {
        SHM_14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn shm_15(&self) -> SHM_15_R {
        SHM_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd_0(&mut self) -> PD_0_W<0> {
        PD_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pd_1(&mut self) -> PD_1_W<1> {
        PD_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pd_2(&mut self) -> PD_2_W<2> {
        PD_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pd_3(&mut self) -> PD_3_W<3> {
        PD_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pd_4(&mut self) -> PD_4_W<4> {
        PD_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pd_5(&mut self) -> PD_5_W<5> {
        PD_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pd_6(&mut self) -> PD_6_W<6> {
        PD_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pd_7(&mut self) -> PD_7_W<7> {
        PD_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pd_8(&mut self) -> PD_8_W<8> {
        PD_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pd_9(&mut self) -> PD_9_W<9> {
        PD_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pd_10(&mut self) -> PD_10_W<10> {
        PD_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pd_11(&mut self) -> PD_11_W<11> {
        PD_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pd_12(&mut self) -> PD_12_W<12> {
        PD_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pd_13(&mut self) -> PD_13_W<13> {
        PD_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pd_14(&mut self) -> PD_14_W<14> {
        PD_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pd_15(&mut self) -> PD_15_W<15> {
        PD_15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn shm_0(&mut self) -> SHM_0_W<16> {
        SHM_0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn shm_1(&mut self) -> SHM_1_W<17> {
        SHM_1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn shm_2(&mut self) -> SHM_2_W<18> {
        SHM_2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn shm_3(&mut self) -> SHM_3_W<19> {
        SHM_3_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn shm_4(&mut self) -> SHM_4_W<20> {
        SHM_4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn shm_5(&mut self) -> SHM_5_W<21> {
        SHM_5_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn shm_6(&mut self) -> SHM_6_W<22> {
        SHM_6_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn shm_7(&mut self) -> SHM_7_W<23> {
        SHM_7_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn shm_8(&mut self) -> SHM_8_W<24> {
        SHM_8_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn shm_9(&mut self) -> SHM_9_W<25> {
        SHM_9_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn shm_10(&mut self) -> SHM_10_W<26> {
        SHM_10_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn shm_11(&mut self) -> SHM_11_W<27> {
        SHM_11_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn shm_12(&mut self) -> SHM_12_W<28> {
        SHM_12_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn shm_13(&mut self) -> SHM_13_W<29> {
        SHM_13_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn shm_14(&mut self) -> SHM_14_W<30> {
        SHM_14_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn shm_15(&mut self) -> SHM_15_W<31> {
        SHM_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Driver Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](index.html) module"]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd::R](R) reader structure"]
impl crate::Readable for PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd::W](W) writer structure"]
impl crate::Writable for PD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
