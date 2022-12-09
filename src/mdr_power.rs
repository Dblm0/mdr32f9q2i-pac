#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POWER Power Detector Control/Status Register"]
    pub pvdcs: PVDCS,
}
#[doc = "PVDCS (rw) register accessor: an alias for `Reg<PVDCS_SPEC>`"]
pub type PVDCS = crate::Reg<pvdcs::PVDCS_SPEC>;
#[doc = "POWER Power Detector Control/Status Register"]
pub mod pvdcs;
