# [doc = "Register `GPCS` reader"] pub struct R (crate :: R < GPCS_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < GPCS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < GPCS_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < GPCS_SPEC >) -> Self { R (reader) } } # [doc = "Register `GPCS` writer"] pub struct W (crate :: W < GPCS_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < GPCS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < GPCS_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < GPCS_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_GPCS_DEVMOD` reader - Device Mode"] pub type USB_GPCS_DEVMOD_R = crate :: BitReader < bool > ; # [doc = "Field `USB_GPCS_DEVMOD` writer - Device Mode"] pub type USB_GPCS_DEVMOD_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , GPCS_SPEC , bool , O > ; # [doc = "Field `USB_GPCS_DEVMODOTG` reader - Enable Device Mode"] pub type USB_GPCS_DEVMODOTG_R = crate :: BitReader < bool > ; # [doc = "Field `USB_GPCS_DEVMODOTG` writer - Enable Device Mode"] pub type USB_GPCS_DEVMODOTG_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , GPCS_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Device Mode"] # [inline (always)] pub fn usb_gpcs_devmod (& self) -> USB_GPCS_DEVMOD_R { USB_GPCS_DEVMOD_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Enable Device Mode"] # [inline (always)] pub fn usb_gpcs_devmodotg (& self) -> USB_GPCS_DEVMODOTG_R { USB_GPCS_DEVMODOTG_R :: new (((self . bits >> 1) & 1) != 0) } } impl W { # [doc = "Bit 0 - Device Mode"] # [inline (always)] # [must_use] pub fn usb_gpcs_devmod (& mut self) -> USB_GPCS_DEVMOD_W < 0 > { USB_GPCS_DEVMOD_W :: new (self) } # [doc = "Bit 1 - Enable Device Mode"] # [inline (always)] # [must_use] pub fn usb_gpcs_devmodotg (& mut self) -> USB_GPCS_DEVMODOTG_W < 1 > { USB_GPCS_DEVMODOTG_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB General-Purpose Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcs](index.html) module"] pub struct GPCS_SPEC ; impl crate :: RegisterSpec for GPCS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [gpcs::R](R) reader structure"] impl crate :: Readable for GPCS_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [gpcs::W](W) writer structure"] impl crate :: Writable for GPCS_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets GPCS to value 0"] impl crate :: Resettable for GPCS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }