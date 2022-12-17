# [doc = "Register `SRWTIMER` reader"] pub struct R (crate :: R < SRWTIMER_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < SRWTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < SRWTIMER_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < SRWTIMER_SPEC >) -> Self { R (reader) } } # [doc = "Register `SRWTIMER` writer"] pub struct W (crate :: W < SRWTIMER_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < SRWTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < SRWTIMER_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < SRWTIMER_SPEC >) -> Self { W (writer) } } # [doc = "Field `SYSCTL_SRWTIMER_R0` reader - 32/64-Bit Wide General-Purpose Timer 0 Software Reset"] pub type SYSCTL_SRWTIMER_R0_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R0` writer - 32/64-Bit Wide General-Purpose Timer 0 Software Reset"] pub type SYSCTL_SRWTIMER_R0_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SRWTIMER_R1` reader - 32/64-Bit Wide General-Purpose Timer 1 Software Reset"] pub type SYSCTL_SRWTIMER_R1_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R1` writer - 32/64-Bit Wide General-Purpose Timer 1 Software Reset"] pub type SYSCTL_SRWTIMER_R1_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SRWTIMER_R2` reader - 32/64-Bit Wide General-Purpose Timer 2 Software Reset"] pub type SYSCTL_SRWTIMER_R2_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R2` writer - 32/64-Bit Wide General-Purpose Timer 2 Software Reset"] pub type SYSCTL_SRWTIMER_R2_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SRWTIMER_R3` reader - 32/64-Bit Wide General-Purpose Timer 3 Software Reset"] pub type SYSCTL_SRWTIMER_R3_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R3` writer - 32/64-Bit Wide General-Purpose Timer 3 Software Reset"] pub type SYSCTL_SRWTIMER_R3_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SRWTIMER_R4` reader - 32/64-Bit Wide General-Purpose Timer 4 Software Reset"] pub type SYSCTL_SRWTIMER_R4_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R4` writer - 32/64-Bit Wide General-Purpose Timer 4 Software Reset"] pub type SYSCTL_SRWTIMER_R4_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_SRWTIMER_R5` reader - 32/64-Bit Wide General-Purpose Timer 5 Software Reset"] pub type SYSCTL_SRWTIMER_R5_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_SRWTIMER_R5` writer - 32/64-Bit Wide General-Purpose Timer 5 Software Reset"] pub type SYSCTL_SRWTIMER_R5_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SRWTIMER_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r0 (& self) -> SYSCTL_SRWTIMER_R0_R { SYSCTL_SRWTIMER_R0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r1 (& self) -> SYSCTL_SRWTIMER_R1_R { SYSCTL_SRWTIMER_R1_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r2 (& self) -> SYSCTL_SRWTIMER_R2_R { SYSCTL_SRWTIMER_R2_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r3 (& self) -> SYSCTL_SRWTIMER_R3_R { SYSCTL_SRWTIMER_R3_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r4 (& self) -> SYSCTL_SRWTIMER_R4_R { SYSCTL_SRWTIMER_R4_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Software Reset"] # [inline (always)] pub fn sysctl_srwtimer_r5 (& self) -> SYSCTL_SRWTIMER_R5_R { SYSCTL_SRWTIMER_R5_R :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r0 (& mut self) -> SYSCTL_SRWTIMER_R0_W < 0 > { SYSCTL_SRWTIMER_R0_W :: new (self) } # [doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r1 (& mut self) -> SYSCTL_SRWTIMER_R1_W < 1 > { SYSCTL_SRWTIMER_R1_W :: new (self) } # [doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r2 (& mut self) -> SYSCTL_SRWTIMER_R2_W < 2 > { SYSCTL_SRWTIMER_R2_W :: new (self) } # [doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r3 (& mut self) -> SYSCTL_SRWTIMER_R3_W < 3 > { SYSCTL_SRWTIMER_R3_W :: new (self) } # [doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r4 (& mut self) -> SYSCTL_SRWTIMER_R4_W < 4 > { SYSCTL_SRWTIMER_R4_W :: new (self) } # [doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Software Reset"] # [inline (always)] # [must_use] pub fn sysctl_srwtimer_r5 (& mut self) -> SYSCTL_SRWTIMER_R5_W < 5 > { SYSCTL_SRWTIMER_R5_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "32/64-Bit Wide General-Purpose Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srwtimer](index.html) module"] pub struct SRWTIMER_SPEC ; impl crate :: RegisterSpec for SRWTIMER_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [srwtimer::R](R) reader structure"] impl crate :: Readable for SRWTIMER_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [srwtimer::W](W) writer structure"] impl crate :: Writable for SRWTIMER_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets SRWTIMER to value 0"] impl crate :: Resettable for SRWTIMER_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }