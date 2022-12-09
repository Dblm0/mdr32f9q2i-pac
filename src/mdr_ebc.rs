#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x50],
    #[doc = "0x50 - EBC NAND Timing Register"]
    pub nand_cycles: NAND_CYCLES,
    #[doc = "0x54 - EBC Control Register"]
    pub control: CONTROL,
}
#[doc = "NAND_CYCLES (rw) register accessor: an alias for `Reg<NAND_CYCLES_SPEC>`"]
pub type NAND_CYCLES = crate::Reg<nand_cycles::NAND_CYCLES_SPEC>;
#[doc = "EBC NAND Timing Register"]
pub mod nand_cycles;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "EBC Control Register"]
pub mod control;
