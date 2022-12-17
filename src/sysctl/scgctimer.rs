# [doc = "Register `SCGCTIMER` reader"] pub struct R (crate :: R < SCGCTIMER_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < SCGCTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < SCGCTIMER_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < SCGCTIMER_SPEC >) -> Self { R (reader) } } # [doc = "Register `SCGCTIMER` writer"] pub struct W (crate :: W < SCGCTIMER_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < SCGCTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < SCGCTIMER_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < SCGCTIMER_SPEC >) -> Self { W (writer) } } # [doc = "Field `SYSCTL_SCGCTIMER_S0` reader - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S0_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S0` writer - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S0_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SCGCTIMER_S1` reader - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S1_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S1` writer - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S1_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SCGCTIMER_S2` reader - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S2_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S2` writer - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S2_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SCGCTIMER_S3` reader - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S3_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S3` writer - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S3_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SCGCTIMER_S4` reader - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S4_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S4` writer - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S4_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SCGCTIMER_S5` reader - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S5_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SCGCTIMER_S5` writer - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"] pub type SYSCTL_SCGCTIMER_S5_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SCGCTIMER_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s0 (& self) -> SYSCTL_SCGCTIMER_S0_R { SYSCTL_SCGCTIMER_S0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s1 (& self) -> SYSCTL_SCGCTIMER_S1_R { SYSCTL_SCGCTIMER_S1_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s2 (& self) -> SYSCTL_SCGCTIMER_S2_R { SYSCTL_SCGCTIMER_S2_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s3 (& self) -> SYSCTL_SCGCTIMER_S3_R { SYSCTL_SCGCTIMER_S3_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s4 (& self) -> SYSCTL_SCGCTIMER_S4_R { SYSCTL_SCGCTIMER_S4_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"] # [inline (always)] pub fn sysctl_scgctimer_s5 (& self) -> SYSCTL_SCGCTIMER_S5_R { SYSCTL_SCGCTIMER_S5_R :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s0 (& mut self) -> SYSCTL_SCGCTIMER_S0_W < 0 > { SYSCTL_SCGCTIMER_S0_W :: new (self) } # [doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s1 (& mut self) -> SYSCTL_SCGCTIMER_S1_W < 1 > { SYSCTL_SCGCTIMER_S1_W :: new (self) } # [doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s2 (& mut self) -> SYSCTL_SCGCTIMER_S2_W < 2 > { SYSCTL_SCGCTIMER_S2_W :: new (self) } # [doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s3 (& mut self) -> SYSCTL_SCGCTIMER_S3_W < 3 > { SYSCTL_SCGCTIMER_S3_W :: new (self) } # [doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s4 (& mut self) -> SYSCTL_SCGCTIMER_S4_W < 4 > { SYSCTL_SCGCTIMER_S4_W :: new (self) } # [doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Sleep Mode Clock Gating Control"] # [inline (always)] # [must_use] pub fn sysctl_scgctimer_s5 (& mut self) -> SYSCTL_SCGCTIMER_S5_W < 5 > { SYSCTL_SCGCTIMER_S5_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgctimer](index.html) module"] pub struct SCGCTIMER_SPEC ; impl crate :: RegisterSpec for SCGCTIMER_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [scgctimer::R](R) reader structure"] impl crate :: Readable for SCGCTIMER_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [scgctimer::W](W) writer structure"] impl crate :: Writable for SCGCTIMER_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets SCGCTIMER to value 0"] impl crate :: Resettable for SCGCTIMER_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }