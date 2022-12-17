# [doc = "Register `PPPWM` reader"] pub struct R (crate :: R < PPPWM_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < PPPWM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < PPPWM_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < PPPWM_SPEC >) -> Self { R (reader) } } # [doc = "Register `PPPWM` writer"] pub struct W (crate :: W < PPPWM_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < PPPWM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < PPPWM_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < PPPWM_SPEC >) -> Self { W (writer) } } # [doc = "Field `SYSCTL_PPPWM_P0` reader - PWM Module 0 Present"] pub type SYSCTL_PPPWM_P0_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPPWM_P0` writer - PWM Module 0 Present"] pub type SYSCTL_PPPWM_P0_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPPWM_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPPWM_P1` reader - PWM Module 1 Present"] pub type SYSCTL_PPPWM_P1_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPPWM_P1` writer - PWM Module 1 Present"] pub type SYSCTL_PPPWM_P1_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPPWM_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - PWM Module 0 Present"] # [inline (always)] pub fn sysctl_pppwm_p0 (& self) -> SYSCTL_PPPWM_P0_R { SYSCTL_PPPWM_P0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - PWM Module 1 Present"] # [inline (always)] pub fn sysctl_pppwm_p1 (& self) -> SYSCTL_PPPWM_P1_R { SYSCTL_PPPWM_P1_R :: new (((self . bits >> 1) & 1) != 0) } } impl W { # [doc = "Bit 0 - PWM Module 0 Present"] # [inline (always)] # [must_use] pub fn sysctl_pppwm_p0 (& mut self) -> SYSCTL_PPPWM_P0_W < 0 > { SYSCTL_PPPWM_P0_W :: new (self) } # [doc = "Bit 1 - PWM Module 1 Present"] # [inline (always)] # [must_use] pub fn sysctl_pppwm_p1 (& mut self) -> SYSCTL_PPPWM_P1_W < 1 > { SYSCTL_PPPWM_P1_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Pulse Width Modulator Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pppwm](index.html) module"] pub struct PPPWM_SPEC ; impl crate :: RegisterSpec for PPPWM_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [pppwm::R](R) reader structure"] impl crate :: Readable for PPPWM_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [pppwm::W](W) writer structure"] impl crate :: Writable for PPPWM_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets PPPWM to value 0"] impl crate :: Resettable for PPPWM_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }