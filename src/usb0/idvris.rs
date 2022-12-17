# [doc = "Register `IDVRIS` reader"] pub struct R (crate :: R < IDVRIS_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < IDVRIS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < IDVRIS_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < IDVRIS_SPEC >) -> Self { R (reader) } } # [doc = "Register `IDVRIS` writer"] pub struct W (crate :: W < IDVRIS_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < IDVRIS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < IDVRIS_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < IDVRIS_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_IDVRIS_ID` reader - ID Valid Detect Raw Interrupt Status"] pub type USB_IDVRIS_ID_R = crate :: BitReader < bool > ; # [doc = "Field `USB_IDVRIS_ID` writer - ID Valid Detect Raw Interrupt Status"] pub type USB_IDVRIS_ID_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IDVRIS_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - ID Valid Detect Raw Interrupt Status"] # [inline (always)] pub fn usb_idvris_id (& self) -> USB_IDVRIS_ID_R { USB_IDVRIS_ID_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - ID Valid Detect Raw Interrupt Status"] # [inline (always)] # [must_use] pub fn usb_idvris_id (& mut self) -> USB_IDVRIS_ID_W < 0 > { USB_IDVRIS_ID_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB ID Valid Detect Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idvris](index.html) module"] pub struct IDVRIS_SPEC ; impl crate :: RegisterSpec for IDVRIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [idvris::R](R) reader structure"] impl crate :: Readable for IDVRIS_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [idvris::W](W) writer structure"] impl crate :: Writable for IDVRIS_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IDVRIS to value 0"] impl crate :: Resettable for IDVRIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }