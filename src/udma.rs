# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - DMA Status"] pub stat : STAT , # [doc = "0x04 - DMA Configuration"] pub cfg : CFG , # [doc = "0x08 - DMA Channel Control Base Pointer"] pub ctlbase : CTLBASE , # [doc = "0x0c - DMA Alternate Channel Control Base Pointer"] pub altbase : ALTBASE , # [doc = "0x10 - DMA Channel Wait-on-Request Status"] pub waitstat : WAITSTAT , # [doc = "0x14 - DMA Channel Software Request"] pub swreq : SWREQ , # [doc = "0x18 - DMA Channel Useburst Set"] pub useburstset : USEBURSTSET , # [doc = "0x1c - DMA Channel Useburst Clear"] pub useburstclr : USEBURSTCLR , # [doc = "0x20 - DMA Channel Request Mask Set"] pub reqmaskset : REQMASKSET , # [doc = "0x24 - DMA Channel Request Mask Clear"] pub reqmaskclr : REQMASKCLR , # [doc = "0x28 - DMA Channel Enable Set"] pub enaset : ENASET , # [doc = "0x2c - DMA Channel Enable Clear"] pub enaclr : ENACLR , # [doc = "0x30 - DMA Channel Primary Alternate Set"] pub altset : ALTSET , # [doc = "0x34 - DMA Channel Primary Alternate Clear"] pub altclr : ALTCLR , # [doc = "0x38 - DMA Channel Priority Set"] pub prioset : PRIOSET , # [doc = "0x3c - DMA Channel Priority Clear"] pub prioclr : PRIOCLR , _reserved16 : [u8 ; 0x0c] , # [doc = "0x4c - DMA Bus Error Clear"] pub errclr : ERRCLR , _reserved17 : [u8 ; 0x04b0] , # [doc = "0x500 - DMA Channel Assignment"] pub chasgn : CHASGN , # [doc = "0x504 - DMA Channel Interrupt Status"] pub chis : CHIS , _reserved19 : [u8 ; 0x08] , # [doc = "0x510 - DMA Channel Map Select 0"] pub chmap0 : CHMAP0 , # [doc = "0x514 - DMA Channel Map Select 1"] pub chmap1 : CHMAP1 , # [doc = "0x518 - DMA Channel Map Select 2"] pub chmap2 : CHMAP2 , # [doc = "0x51c - DMA Channel Map Select 3"] pub chmap3 : CHMAP3 , } # [doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"] pub type STAT = crate :: Reg < stat :: STAT_SPEC > ; # [doc = "DMA Status"] pub mod stat ; # [doc = "CFG (w) register accessor: an alias for `Reg<CFG_SPEC>`"] pub type CFG = crate :: Reg < cfg :: CFG_SPEC > ; # [doc = "DMA Configuration"] pub mod cfg ; # [doc = "CTLBASE (rw) register accessor: an alias for `Reg<CTLBASE_SPEC>`"] pub type CTLBASE = crate :: Reg < ctlbase :: CTLBASE_SPEC > ; # [doc = "DMA Channel Control Base Pointer"] pub mod ctlbase ; # [doc = "ALTBASE (rw) register accessor: an alias for `Reg<ALTBASE_SPEC>`"] pub type ALTBASE = crate :: Reg < altbase :: ALTBASE_SPEC > ; # [doc = "DMA Alternate Channel Control Base Pointer"] pub mod altbase ; # [doc = "WAITSTAT (rw) register accessor: an alias for `Reg<WAITSTAT_SPEC>`"] pub type WAITSTAT = crate :: Reg < waitstat :: WAITSTAT_SPEC > ; # [doc = "DMA Channel Wait-on-Request Status"] pub mod waitstat ; # [doc = "SWREQ (w) register accessor: an alias for `Reg<SWREQ_SPEC>`"] pub type SWREQ = crate :: Reg < swreq :: SWREQ_SPEC > ; # [doc = "DMA Channel Software Request"] pub mod swreq ; # [doc = "USEBURSTSET (rw) register accessor: an alias for `Reg<USEBURSTSET_SPEC>`"] pub type USEBURSTSET = crate :: Reg < useburstset :: USEBURSTSET_SPEC > ; # [doc = "DMA Channel Useburst Set"] pub mod useburstset ; # [doc = "USEBURSTCLR (w) register accessor: an alias for `Reg<USEBURSTCLR_SPEC>`"] pub type USEBURSTCLR = crate :: Reg < useburstclr :: USEBURSTCLR_SPEC > ; # [doc = "DMA Channel Useburst Clear"] pub mod useburstclr ; # [doc = "REQMASKSET (rw) register accessor: an alias for `Reg<REQMASKSET_SPEC>`"] pub type REQMASKSET = crate :: Reg < reqmaskset :: REQMASKSET_SPEC > ; # [doc = "DMA Channel Request Mask Set"] pub mod reqmaskset ; # [doc = "REQMASKCLR (w) register accessor: an alias for `Reg<REQMASKCLR_SPEC>`"] pub type REQMASKCLR = crate :: Reg < reqmaskclr :: REQMASKCLR_SPEC > ; # [doc = "DMA Channel Request Mask Clear"] pub mod reqmaskclr ; # [doc = "ENASET (rw) register accessor: an alias for `Reg<ENASET_SPEC>`"] pub type ENASET = crate :: Reg < enaset :: ENASET_SPEC > ; # [doc = "DMA Channel Enable Set"] pub mod enaset ; # [doc = "ENACLR (w) register accessor: an alias for `Reg<ENACLR_SPEC>`"] pub type ENACLR = crate :: Reg < enaclr :: ENACLR_SPEC > ; # [doc = "DMA Channel Enable Clear"] pub mod enaclr ; # [doc = "ALTSET (rw) register accessor: an alias for `Reg<ALTSET_SPEC>`"] pub type ALTSET = crate :: Reg < altset :: ALTSET_SPEC > ; # [doc = "DMA Channel Primary Alternate Set"] pub mod altset ; # [doc = "ALTCLR (w) register accessor: an alias for `Reg<ALTCLR_SPEC>`"] pub type ALTCLR = crate :: Reg < altclr :: ALTCLR_SPEC > ; # [doc = "DMA Channel Primary Alternate Clear"] pub mod altclr ; # [doc = "PRIOSET (rw) register accessor: an alias for `Reg<PRIOSET_SPEC>`"] pub type PRIOSET = crate :: Reg < prioset :: PRIOSET_SPEC > ; # [doc = "DMA Channel Priority Set"] pub mod prioset ; # [doc = "PRIOCLR (w) register accessor: an alias for `Reg<PRIOCLR_SPEC>`"] pub type PRIOCLR = crate :: Reg < prioclr :: PRIOCLR_SPEC > ; # [doc = "DMA Channel Priority Clear"] pub mod prioclr ; # [doc = "ERRCLR (rw) register accessor: an alias for `Reg<ERRCLR_SPEC>`"] pub type ERRCLR = crate :: Reg < errclr :: ERRCLR_SPEC > ; # [doc = "DMA Bus Error Clear"] pub mod errclr ; # [doc = "CHASGN (rw) register accessor: an alias for `Reg<CHASGN_SPEC>`"] pub type CHASGN = crate :: Reg < chasgn :: CHASGN_SPEC > ; # [doc = "DMA Channel Assignment"] pub mod chasgn ; # [doc = "CHIS (rw) register accessor: an alias for `Reg<CHIS_SPEC>`"] pub type CHIS = crate :: Reg < chis :: CHIS_SPEC > ; # [doc = "DMA Channel Interrupt Status"] pub mod chis ; # [doc = "CHMAP0 (rw) register accessor: an alias for `Reg<CHMAP0_SPEC>`"] pub type CHMAP0 = crate :: Reg < chmap0 :: CHMAP0_SPEC > ; # [doc = "DMA Channel Map Select 0"] pub mod chmap0 ; # [doc = "CHMAP1 (rw) register accessor: an alias for `Reg<CHMAP1_SPEC>`"] pub type CHMAP1 = crate :: Reg < chmap1 :: CHMAP1_SPEC > ; # [doc = "DMA Channel Map Select 1"] pub mod chmap1 ; # [doc = "CHMAP2 (rw) register accessor: an alias for `Reg<CHMAP2_SPEC>`"] pub type CHMAP2 = crate :: Reg < chmap2 :: CHMAP2_SPEC > ; # [doc = "DMA Channel Map Select 2"] pub mod chmap2 ; # [doc = "CHMAP3 (rw) register accessor: an alias for `Reg<CHMAP3_SPEC>`"] pub type CHMAP3 = crate :: Reg < chmap3 :: CHMAP3_SPEC > ; # [doc = "DMA Channel Map Select 3"] pub mod chmap3 ;