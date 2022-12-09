#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Prescaler (low byte) Register"]
    pub prl: PRL,
    #[doc = "0x04 - I2C Prescaler (high byte) Register"]
    pub prh: PRH,
    #[doc = "0x08 - I2C Control Register"]
    pub ctr: CTR,
    #[doc = "0x0c - I2C Received Data Register"]
    pub rxd: RXD,
    #[doc = "0x10 - I2C Status Register"]
    pub sta: STA,
    #[doc = "0x14 - I2C Transmitted Data Register"]
    pub txd: TXD,
    #[doc = "0x18 - I2C Command Register"]
    pub cmd: CMD,
}
#[doc = "PRL (rw) register accessor: an alias for `Reg<PRL_SPEC>`"]
pub type PRL = crate::Reg<prl::PRL_SPEC>;
#[doc = "I2C Prescaler (low byte) Register"]
pub mod prl;
#[doc = "PRH (rw) register accessor: an alias for `Reg<PRH_SPEC>`"]
pub type PRH = crate::Reg<prh::PRH_SPEC>;
#[doc = "I2C Prescaler (high byte) Register"]
pub mod prh;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "I2C Control Register"]
pub mod ctr;
#[doc = "RXD (rw) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "I2C Received Data Register"]
pub mod rxd;
#[doc = "STA (rw) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "I2C Status Register"]
pub mod sta;
#[doc = "TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "I2C Transmitted Data Register"]
pub mod txd;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "I2C Command Register"]
pub mod cmd;
