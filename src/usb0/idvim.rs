# [doc = "Register `IDVIM` reader"] pub struct R (crate :: R < IDVIM_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < IDVIM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < IDVIM_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < IDVIM_SPEC >) -> Self { R (reader) } } # [doc = "Register `IDVIM` writer"] pub struct W (crate :: W < IDVIM_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < IDVIM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < IDVIM_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < IDVIM_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_IDVIM_ID` reader - ID Valid Detect Interrupt Mask"] pub type USB_IDVIM_ID_R = crate :: BitReader < bool > ; # [doc = "Field `USB_IDVIM_ID` writer - ID Valid Detect Interrupt Mask"] pub type USB_IDVIM_ID_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IDVIM_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - ID Valid Detect Interrupt Mask"] # [inline (always)] pub fn usb_idvim_id (& self) -> USB_IDVIM_ID_R { USB_IDVIM_ID_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - ID Valid Detect Interrupt Mask"] # [inline (always)] # [must_use] pub fn usb_idvim_id (& mut self) -> USB_IDVIM_ID_W < 0 > { USB_IDVIM_ID_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB ID Valid Detect Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idvim](index.html) module"] pub struct IDVIM_SPEC ; impl crate :: RegisterSpec for IDVIM_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [idvim::R](R) reader structure"] impl crate :: Readable for IDVIM_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [idvim::W](W) writer structure"] impl crate :: Writable for IDVIM_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IDVIM to value 0"] impl crate :: Resettable for IDVIM_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }