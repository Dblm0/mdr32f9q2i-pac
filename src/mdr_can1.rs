#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - CAN Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - CAN Bittiming Register"]
    pub bittmng: BITTMNG,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CAN Interrupt enable Register"]
    pub int_en: INT_EN,
    _reserved4: [u8; 0x08],
    #[doc = "0x1c - "]
    pub over: OVER,
    #[doc = "0x20 - CAN Receive ID Register"]
    pub rxid: RXID,
    #[doc = "0x24 - CAN Receive DLC Register"]
    pub rxdlc: RXDLC,
    #[doc = "0x28 - CAN Receive Data low Register"]
    pub rxdatal: RXDATAL,
    #[doc = "0x2c - CAN Receive Data high Register"]
    pub rxdatah: RXDATAH,
    #[doc = "0x30 - CAN Transmit ID Register"]
    pub txid: TXID,
    #[doc = "0x34 - CAN Transmit DLC Register"]
    pub txdlc: TXDLC,
    #[doc = "0x38 - CAN Transmit Data low Register"]
    pub datal: DATAL,
    #[doc = "0x3c - CAN Transmit Data high Register"]
    pub datah: DATAH,
    #[doc = "0x40 - CAN Buffer Connection Register"]
    pub buf_con00: BUF_CON00,
    #[doc = "0x44 - CAN Buffer Connection Register"]
    pub buf_con01: BUF_CON01,
    #[doc = "0x48 - CAN Buffer Connection Register"]
    pub buf_con02: BUF_CON02,
    #[doc = "0x4c - CAN Buffer Connection Register"]
    pub buf_con03: BUF_CON03,
    #[doc = "0x50 - CAN Buffer Connection Register"]
    pub buf_con04: BUF_CON04,
    #[doc = "0x54 - CAN Buffer Connection Register"]
    pub buf_con05: BUF_CON05,
    #[doc = "0x58 - CAN Buffer Connection Register"]
    pub buf_con06: BUF_CON06,
    #[doc = "0x5c - CAN Buffer Connection Register"]
    pub buf_con07: BUF_CON07,
    #[doc = "0x60 - CAN Buffer Connection Register"]
    pub buf_con08: BUF_CON08,
    #[doc = "0x64 - CAN Buffer Connection Register"]
    pub buf_con09: BUF_CON09,
    #[doc = "0x68 - CAN Buffer Connection Register"]
    pub buf_con10: BUF_CON10,
    #[doc = "0x6c - CAN Buffer Connection Register"]
    pub buf_con11: BUF_CON11,
    #[doc = "0x70 - CAN Buffer Connection Register"]
    pub buf_con12: BUF_CON12,
    #[doc = "0x74 - CAN Buffer Connection Register"]
    pub buf_con13: BUF_CON13,
    #[doc = "0x78 - CAN Buffer Connection Register"]
    pub buf_con14: BUF_CON14,
    #[doc = "0x7c - CAN Buffer Connection Register"]
    pub buf_con15: BUF_CON15,
    #[doc = "0x80 - CAN Buffer Connection Register"]
    pub buf_con16: BUF_CON16,
    #[doc = "0x84 - CAN Buffer Connection Register"]
    pub buf_con17: BUF_CON17,
    #[doc = "0x88 - CAN Buffer Connection Register"]
    pub buf_con18: BUF_CON18,
    #[doc = "0x8c - CAN Buffer Connection Register"]
    pub buf_con19: BUF_CON19,
    #[doc = "0x90 - CAN Buffer Connection Register"]
    pub buf_con20: BUF_CON20,
    #[doc = "0x94 - CAN Buffer Connection Register"]
    pub buf_con21: BUF_CON21,
    #[doc = "0x98 - CAN Buffer Connection Register"]
    pub buf_con22: BUF_CON22,
    #[doc = "0x9c - CAN Buffer Connection Register"]
    pub buf_con23: BUF_CON23,
    #[doc = "0xa0 - CAN Buffer Connection Register"]
    pub buf_con24: BUF_CON24,
    #[doc = "0xa4 - CAN Buffer Connection Register"]
    pub buf_con25: BUF_CON25,
    #[doc = "0xa8 - CAN Buffer Connection Register"]
    pub buf_con26: BUF_CON26,
    #[doc = "0xac - CAN Buffer Connection Register"]
    pub buf_con27: BUF_CON27,
    #[doc = "0xb0 - CAN Buffer Connection Register"]
    pub buf_con28: BUF_CON28,
    #[doc = "0xb4 - CAN Buffer Connection Register"]
    pub buf_con29: BUF_CON29,
    #[doc = "0xb8 - CAN Buffer Connection Register"]
    pub buf_con30: BUF_CON30,
    #[doc = "0xbc - CAN Buffer Connection Register"]
    pub buf_con31: BUF_CON31,
    #[doc = "0xc0 - "]
    pub int_rx: INT_RX,
    #[doc = "0xc4 - "]
    pub rx: RX,
    #[doc = "0xc8 - "]
    pub int_tx: INT_TX,
    #[doc = "0xcc - "]
    pub tx: TX,
    _reserved49: [u8; 0x0130],
    #[doc = "0x200 - CAN Buffer ID Register"]
    pub buf_00_id: BUF_00_ID,
    #[doc = "0x204 - CAN Buffer DLC Register"]
    pub buf_00_dlc: BUF_00_DLC,
    #[doc = "0x208 - CAN Buffer Data low Register"]
    pub buf_00_datal: BUF_00_DATAL,
    #[doc = "0x20c - CAN Buffer Data high Register"]
    pub buf_00_datah: BUF_00_DATAH,
    #[doc = "0x210 - CAN Buffer ID Register"]
    pub buf_01_id: BUF_01_ID,
    #[doc = "0x214 - CAN Buffer DLC Register"]
    pub buf_01_dlc: BUF_01_DLC,
    #[doc = "0x218 - CAN Buffer Data low Register"]
    pub buf_01_datal: BUF_01_DATAL,
    #[doc = "0x21c - CAN Buffer Data high Register"]
    pub buf_01_datah: BUF_01_DATAH,
    #[doc = "0x220 - CAN Buffer ID Register"]
    pub buf_02_id: BUF_02_ID,
    #[doc = "0x224 - CAN Buffer DLC Register"]
    pub buf_02_dlc: BUF_02_DLC,
    #[doc = "0x228 - CAN Buffer Data low Register"]
    pub buf_02_datal: BUF_02_DATAL,
    #[doc = "0x22c - CAN Buffer Data high Register"]
    pub buf_02_datah: BUF_02_DATAH,
    #[doc = "0x230 - CAN Buffer ID Register"]
    pub buf_03_id: BUF_03_ID,
    #[doc = "0x234 - CAN Buffer DLC Register"]
    pub buf_03_dlc: BUF_03_DLC,
    #[doc = "0x238 - CAN Buffer Data low Register"]
    pub buf_03_datal: BUF_03_DATAL,
    #[doc = "0x23c - CAN Buffer Data high Register"]
    pub buf_03_datah: BUF_03_DATAH,
    #[doc = "0x240 - CAN Buffer ID Register"]
    pub buf_04_id: BUF_04_ID,
    #[doc = "0x244 - CAN Buffer DLC Register"]
    pub buf_04_dlc: BUF_04_DLC,
    #[doc = "0x248 - CAN Buffer Data low Register"]
    pub buf_04_datal: BUF_04_DATAL,
    #[doc = "0x24c - CAN Buffer Data high Register"]
    pub buf_04_datah: BUF_04_DATAH,
    #[doc = "0x250 - CAN Buffer ID Register"]
    pub buf_05_id: BUF_05_ID,
    #[doc = "0x254 - CAN Buffer DLC Register"]
    pub buf_05_dlc: BUF_05_DLC,
    #[doc = "0x258 - CAN Buffer Data low Register"]
    pub buf_05_datal: BUF_05_DATAL,
    #[doc = "0x25c - CAN Buffer Data high Register"]
    pub buf_05_datah: BUF_05_DATAH,
    #[doc = "0x260 - CAN Buffer ID Register"]
    pub buf_06_id: BUF_06_ID,
    #[doc = "0x264 - CAN Buffer DLC Register"]
    pub buf_06_dlc: BUF_06_DLC,
    #[doc = "0x268 - CAN Buffer Data low Register"]
    pub buf_06_datal: BUF_06_DATAL,
    #[doc = "0x26c - CAN Buffer Data high Register"]
    pub buf_06_datah: BUF_06_DATAH,
    #[doc = "0x270 - CAN Buffer ID Register"]
    pub buf_07_id: BUF_07_ID,
    #[doc = "0x274 - CAN Buffer DLC Register"]
    pub buf_07_dlc: BUF_07_DLC,
    #[doc = "0x278 - CAN Buffer Data low Register"]
    pub buf_07_datal: BUF_07_DATAL,
    #[doc = "0x27c - CAN Buffer Data high Register"]
    pub buf_07_datah: BUF_07_DATAH,
    #[doc = "0x280 - CAN Buffer ID Register"]
    pub buf_08_id: BUF_08_ID,
    #[doc = "0x284 - CAN Buffer DLC Register"]
    pub buf_08_dlc: BUF_08_DLC,
    #[doc = "0x288 - CAN Buffer Data low Register"]
    pub buf_08_datal: BUF_08_DATAL,
    #[doc = "0x28c - CAN Buffer Data high Register"]
    pub buf_08_datah: BUF_08_DATAH,
    #[doc = "0x290 - CAN Buffer ID Register"]
    pub buf_09_id: BUF_09_ID,
    #[doc = "0x294 - CAN Buffer DLC Register"]
    pub buf_09_dlc: BUF_09_DLC,
    #[doc = "0x298 - CAN Buffer Data low Register"]
    pub buf_09_datal: BUF_09_DATAL,
    #[doc = "0x29c - CAN Buffer Data high Register"]
    pub buf_09_datah: BUF_09_DATAH,
    #[doc = "0x2a0 - CAN Buffer ID Register"]
    pub buf_10_id: BUF_10_ID,
    #[doc = "0x2a4 - CAN Buffer DLC Register"]
    pub buf_10_dlc: BUF_10_DLC,
    #[doc = "0x2a8 - CAN Buffer Data low Register"]
    pub buf_10_datal: BUF_10_DATAL,
    #[doc = "0x2ac - CAN Buffer Data high Register"]
    pub buf_10_datah: BUF_10_DATAH,
    #[doc = "0x2b0 - CAN Buffer ID Register"]
    pub buf_11_id: BUF_11_ID,
    #[doc = "0x2b4 - CAN Buffer DLC Register"]
    pub buf_11_dlc: BUF_11_DLC,
    #[doc = "0x2b8 - CAN Buffer Data low Register"]
    pub buf_11_datal: BUF_11_DATAL,
    #[doc = "0x2bc - CAN Buffer Data high Register"]
    pub buf_11_datah: BUF_11_DATAH,
    #[doc = "0x2c0 - CAN Buffer ID Register"]
    pub buf_12_id: BUF_12_ID,
    #[doc = "0x2c4 - CAN Buffer DLC Register"]
    pub buf_12_dlc: BUF_12_DLC,
    #[doc = "0x2c8 - CAN Buffer Data low Register"]
    pub buf_12_datal: BUF_12_DATAL,
    #[doc = "0x2cc - CAN Buffer Data high Register"]
    pub buf_12_datah: BUF_12_DATAH,
    #[doc = "0x2d0 - CAN Buffer ID Register"]
    pub buf_13_id: BUF_13_ID,
    #[doc = "0x2d4 - CAN Buffer DLC Register"]
    pub buf_13_dlc: BUF_13_DLC,
    #[doc = "0x2d8 - CAN Buffer Data low Register"]
    pub buf_13_datal: BUF_13_DATAL,
    #[doc = "0x2dc - CAN Buffer Data high Register"]
    pub buf_13_datah: BUF_13_DATAH,
    #[doc = "0x2e0 - CAN Buffer ID Register"]
    pub buf_14_id: BUF_14_ID,
    #[doc = "0x2e4 - CAN Buffer DLC Register"]
    pub buf_14_dlc: BUF_14_DLC,
    #[doc = "0x2e8 - CAN Buffer Data low Register"]
    pub buf_14_datal: BUF_14_DATAL,
    #[doc = "0x2ec - CAN Buffer Data high Register"]
    pub buf_14_datah: BUF_14_DATAH,
    #[doc = "0x2f0 - CAN Buffer ID Register"]
    pub buf_15_id: BUF_15_ID,
    #[doc = "0x2f4 - CAN Buffer DLC Register"]
    pub buf_15_dlc: BUF_15_DLC,
    #[doc = "0x2f8 - CAN Buffer Data low Register"]
    pub buf_15_datal: BUF_15_DATAL,
    #[doc = "0x2fc - CAN Buffer Data high Register"]
    pub buf_15_datah: BUF_15_DATAH,
    #[doc = "0x300 - CAN Buffer ID Register"]
    pub buf_16_id: BUF_16_ID,
    #[doc = "0x304 - CAN Buffer DLC Register"]
    pub buf_16_dlc: BUF_16_DLC,
    #[doc = "0x308 - CAN Buffer Data low Register"]
    pub buf_16_datal: BUF_16_DATAL,
    #[doc = "0x30c - CAN Buffer Data high Register"]
    pub buf_16_datah: BUF_16_DATAH,
    #[doc = "0x310 - CAN Buffer ID Register"]
    pub buf_17_id: BUF_17_ID,
    #[doc = "0x314 - CAN Buffer DLC Register"]
    pub buf_17_dlc: BUF_17_DLC,
    #[doc = "0x318 - CAN Buffer Data low Register"]
    pub buf_17_datal: BUF_17_DATAL,
    #[doc = "0x31c - CAN Buffer Data high Register"]
    pub buf_17_datah: BUF_17_DATAH,
    #[doc = "0x320 - CAN Buffer ID Register"]
    pub buf_18_id: BUF_18_ID,
    #[doc = "0x324 - CAN Buffer DLC Register"]
    pub buf_18_dlc: BUF_18_DLC,
    #[doc = "0x328 - CAN Buffer Data low Register"]
    pub buf_18_datal: BUF_18_DATAL,
    #[doc = "0x32c - CAN Buffer Data high Register"]
    pub buf_18_datah: BUF_18_DATAH,
    #[doc = "0x330 - CAN Buffer ID Register"]
    pub buf_19_id: BUF_19_ID,
    #[doc = "0x334 - CAN Buffer DLC Register"]
    pub buf_19_dlc: BUF_19_DLC,
    #[doc = "0x338 - CAN Buffer Data low Register"]
    pub buf_19_datal: BUF_19_DATAL,
    #[doc = "0x33c - CAN Buffer Data high Register"]
    pub buf_19_datah: BUF_19_DATAH,
    #[doc = "0x340 - CAN Buffer ID Register"]
    pub buf_20_id: BUF_20_ID,
    #[doc = "0x344 - CAN Buffer DLC Register"]
    pub buf_20_dlc: BUF_20_DLC,
    #[doc = "0x348 - CAN Buffer Data low Register"]
    pub buf_20_datal: BUF_20_DATAL,
    #[doc = "0x34c - CAN Buffer Data high Register"]
    pub buf_20_datah: BUF_20_DATAH,
    #[doc = "0x350 - CAN Buffer ID Register"]
    pub buf_21_id: BUF_21_ID,
    #[doc = "0x354 - CAN Buffer DLC Register"]
    pub buf_21_dlc: BUF_21_DLC,
    #[doc = "0x358 - CAN Buffer Data low Register"]
    pub buf_21_datal: BUF_21_DATAL,
    #[doc = "0x35c - CAN Buffer Data high Register"]
    pub buf_21_datah: BUF_21_DATAH,
    #[doc = "0x360 - CAN Buffer ID Register"]
    pub buf_22_id: BUF_22_ID,
    #[doc = "0x364 - CAN Buffer DLC Register"]
    pub buf_22_dlc: BUF_22_DLC,
    #[doc = "0x368 - CAN Buffer Data low Register"]
    pub buf_22_datal: BUF_22_DATAL,
    #[doc = "0x36c - CAN Buffer Data high Register"]
    pub buf_22_datah: BUF_22_DATAH,
    #[doc = "0x370 - CAN Buffer ID Register"]
    pub buf_23_id: BUF_23_ID,
    #[doc = "0x374 - CAN Buffer DLC Register"]
    pub buf_23_dlc: BUF_23_DLC,
    #[doc = "0x378 - CAN Buffer Data low Register"]
    pub buf_23_datal: BUF_23_DATAL,
    #[doc = "0x37c - CAN Buffer Data high Register"]
    pub buf_23_datah: BUF_23_DATAH,
    #[doc = "0x380 - CAN Buffer ID Register"]
    pub buf_24_id: BUF_24_ID,
    #[doc = "0x384 - CAN Buffer DLC Register"]
    pub buf_24_dlc: BUF_24_DLC,
    #[doc = "0x388 - CAN Buffer Data low Register"]
    pub buf_24_datal: BUF_24_DATAL,
    #[doc = "0x38c - CAN Buffer Data high Register"]
    pub buf_24_datah: BUF_24_DATAH,
    #[doc = "0x390 - CAN Buffer ID Register"]
    pub buf_25_id: BUF_25_ID,
    #[doc = "0x394 - CAN Buffer DLC Register"]
    pub buf_25_dlc: BUF_25_DLC,
    #[doc = "0x398 - CAN Buffer Data low Register"]
    pub buf_25_datal: BUF_25_DATAL,
    #[doc = "0x39c - CAN Buffer Data high Register"]
    pub buf_25_datah: BUF_25_DATAH,
    #[doc = "0x3a0 - CAN Buffer ID Register"]
    pub buf_26_id: BUF_26_ID,
    #[doc = "0x3a4 - CAN Buffer DLC Register"]
    pub buf_26_dlc: BUF_26_DLC,
    #[doc = "0x3a8 - CAN Buffer Data low Register"]
    pub buf_26_datal: BUF_26_DATAL,
    #[doc = "0x3ac - CAN Buffer Data high Register"]
    pub buf_26_datah: BUF_26_DATAH,
    #[doc = "0x3b0 - CAN Buffer ID Register"]
    pub buf_27_id: BUF_27_ID,
    #[doc = "0x3b4 - CAN Buffer DLC Register"]
    pub buf_27_dlc: BUF_27_DLC,
    #[doc = "0x3b8 - CAN Buffer Data low Register"]
    pub buf_27_datal: BUF_27_DATAL,
    #[doc = "0x3bc - CAN Buffer Data high Register"]
    pub buf_27_datah: BUF_27_DATAH,
    #[doc = "0x3c0 - CAN Buffer ID Register"]
    pub buf_28_id: BUF_28_ID,
    #[doc = "0x3c4 - CAN Buffer DLC Register"]
    pub buf_28_dlc: BUF_28_DLC,
    #[doc = "0x3c8 - CAN Buffer Data low Register"]
    pub buf_28_datal: BUF_28_DATAL,
    #[doc = "0x3cc - CAN Buffer Data high Register"]
    pub buf_28_datah: BUF_28_DATAH,
    #[doc = "0x3d0 - CAN Buffer ID Register"]
    pub buf_29_id: BUF_29_ID,
    #[doc = "0x3d4 - CAN Buffer DLC Register"]
    pub buf_29_dlc: BUF_29_DLC,
    #[doc = "0x3d8 - CAN Buffer Data low Register"]
    pub buf_29_datal: BUF_29_DATAL,
    #[doc = "0x3dc - CAN Buffer Data high Register"]
    pub buf_29_datah: BUF_29_DATAH,
    #[doc = "0x3e0 - CAN Buffer ID Register"]
    pub buf_30_id: BUF_30_ID,
    #[doc = "0x3e4 - CAN Buffer DLC Register"]
    pub buf_30_dlc: BUF_30_DLC,
    #[doc = "0x3e8 - CAN Buffer Data low Register"]
    pub buf_30_datal: BUF_30_DATAL,
    #[doc = "0x3ec - CAN Buffer Data high Register"]
    pub buf_30_datah: BUF_30_DATAH,
    #[doc = "0x3f0 - CAN Buffer ID Register"]
    pub buf_31_id: BUF_31_ID,
    #[doc = "0x3f4 - CAN Buffer DLC Register"]
    pub buf_31_dlc: BUF_31_DLC,
    #[doc = "0x3f8 - CAN Buffer Data low Register"]
    pub buf_31_datal: BUF_31_DATAL,
    #[doc = "0x3fc - CAN Buffer Data high Register"]
    pub buf_31_datah: BUF_31_DATAH,
    _reserved177: [u8; 0x0100],
    #[doc = "0x500 - "]
    pub buf_filter00_mask: BUF_FILTER00_MASK,
    #[doc = "0x504 - "]
    pub buf_filter00_filter: BUF_FILTER00_FILTER,
    #[doc = "0x508 - "]
    pub buf_filter01_mask: BUF_FILTER01_MASK,
    #[doc = "0x50c - "]
    pub buf_filter01_filter: BUF_FILTER01_FILTER,
    #[doc = "0x510 - "]
    pub buf_filter02_mask: BUF_FILTER02_MASK,
    #[doc = "0x514 - "]
    pub buf_filter02_filter: BUF_FILTER02_FILTER,
    #[doc = "0x518 - "]
    pub buf_filter03_mask: BUF_FILTER03_MASK,
    #[doc = "0x51c - "]
    pub buf_filter03_filter: BUF_FILTER03_FILTER,
    #[doc = "0x520 - "]
    pub buf_filter04_mask: BUF_FILTER04_MASK,
    #[doc = "0x524 - "]
    pub buf_filter04_filter: BUF_FILTER04_FILTER,
    #[doc = "0x528 - "]
    pub buf_filter05_mask: BUF_FILTER05_MASK,
    #[doc = "0x52c - "]
    pub buf_filter05_filter: BUF_FILTER05_FILTER,
    #[doc = "0x530 - "]
    pub buf_filter06_mask: BUF_FILTER06_MASK,
    #[doc = "0x534 - "]
    pub buf_filter06_filter: BUF_FILTER06_FILTER,
    #[doc = "0x538 - "]
    pub buf_filter07_mask: BUF_FILTER07_MASK,
    #[doc = "0x53c - "]
    pub buf_filter07_filter: BUF_FILTER07_FILTER,
    #[doc = "0x540 - "]
    pub buf_filter08_mask: BUF_FILTER08_MASK,
    #[doc = "0x544 - "]
    pub buf_filter08_filter: BUF_FILTER08_FILTER,
    #[doc = "0x548 - "]
    pub buf_filter09_mask: BUF_FILTER09_MASK,
    #[doc = "0x54c - "]
    pub buf_filter09_filter: BUF_FILTER09_FILTER,
    #[doc = "0x550 - "]
    pub buf_filter10_mask: BUF_FILTER10_MASK,
    #[doc = "0x554 - "]
    pub buf_filter10_filter: BUF_FILTER10_FILTER,
    #[doc = "0x558 - "]
    pub buf_filter11_mask: BUF_FILTER11_MASK,
    #[doc = "0x55c - "]
    pub buf_filter11_filter: BUF_FILTER11_FILTER,
    #[doc = "0x560 - "]
    pub buf_filter12_mask: BUF_FILTER12_MASK,
    #[doc = "0x564 - "]
    pub buf_filter12_filter: BUF_FILTER12_FILTER,
    #[doc = "0x568 - "]
    pub buf_filter13_mask: BUF_FILTER13_MASK,
    #[doc = "0x56c - "]
    pub buf_filter13_filter: BUF_FILTER13_FILTER,
    #[doc = "0x570 - "]
    pub buf_filter14_mask: BUF_FILTER14_MASK,
    #[doc = "0x574 - "]
    pub buf_filter14_filter: BUF_FILTER14_FILTER,
    #[doc = "0x578 - "]
    pub buf_filter15_mask: BUF_FILTER15_MASK,
    #[doc = "0x57c - "]
    pub buf_filter15_filter: BUF_FILTER15_FILTER,
    #[doc = "0x580 - "]
    pub buf_filter16_mask: BUF_FILTER16_MASK,
    #[doc = "0x584 - "]
    pub buf_filter16_filter: BUF_FILTER16_FILTER,
    #[doc = "0x588 - "]
    pub buf_filter17_mask: BUF_FILTER17_MASK,
    #[doc = "0x58c - "]
    pub buf_filter17_filter: BUF_FILTER17_FILTER,
    #[doc = "0x590 - "]
    pub buf_filter18_mask: BUF_FILTER18_MASK,
    #[doc = "0x594 - "]
    pub buf_filter18_filter: BUF_FILTER18_FILTER,
    #[doc = "0x598 - "]
    pub buf_filter19_mask: BUF_FILTER19_MASK,
    #[doc = "0x59c - "]
    pub buf_filter19_filter: BUF_FILTER19_FILTER,
    #[doc = "0x5a0 - "]
    pub buf_filter20_mask: BUF_FILTER20_MASK,
    #[doc = "0x5a4 - "]
    pub buf_filter20_filter: BUF_FILTER20_FILTER,
    #[doc = "0x5a8 - "]
    pub buf_filter21_mask: BUF_FILTER21_MASK,
    #[doc = "0x5ac - "]
    pub buf_filter21_filter: BUF_FILTER21_FILTER,
    #[doc = "0x5b0 - "]
    pub buf_filter22_mask: BUF_FILTER22_MASK,
    #[doc = "0x5b4 - "]
    pub buf_filter22_filter: BUF_FILTER22_FILTER,
    #[doc = "0x5b8 - "]
    pub buf_filter23_mask: BUF_FILTER23_MASK,
    #[doc = "0x5bc - "]
    pub buf_filter23_filter: BUF_FILTER23_FILTER,
    #[doc = "0x5c0 - "]
    pub buf_filter24_mask: BUF_FILTER24_MASK,
    #[doc = "0x5c4 - "]
    pub buf_filter24_filter: BUF_FILTER24_FILTER,
    #[doc = "0x5c8 - "]
    pub buf_filter25_mask: BUF_FILTER25_MASK,
    #[doc = "0x5cc - "]
    pub buf_filter25_filter: BUF_FILTER25_FILTER,
    #[doc = "0x5d0 - "]
    pub buf_filter26_mask: BUF_FILTER26_MASK,
    #[doc = "0x5d4 - "]
    pub buf_filter26_filter: BUF_FILTER26_FILTER,
    #[doc = "0x5d8 - "]
    pub buf_filter27_mask: BUF_FILTER27_MASK,
    #[doc = "0x5dc - "]
    pub buf_filter27_filter: BUF_FILTER27_FILTER,
    #[doc = "0x5e0 - "]
    pub buf_filter28_mask: BUF_FILTER28_MASK,
    #[doc = "0x5e4 - "]
    pub buf_filter28_filter: BUF_FILTER28_FILTER,
    #[doc = "0x5e8 - "]
    pub buf_filter29_mask: BUF_FILTER29_MASK,
    #[doc = "0x5ec - "]
    pub buf_filter29_filter: BUF_FILTER29_FILTER,
    #[doc = "0x5f0 - "]
    pub buf_filter30_mask: BUF_FILTER30_MASK,
    #[doc = "0x5f4 - "]
    pub buf_filter30_filter: BUF_FILTER30_FILTER,
    #[doc = "0x5f8 - "]
    pub buf_filter31_mask: BUF_FILTER31_MASK,
    #[doc = "0x5fc - "]
    pub buf_filter31_filter: BUF_FILTER31_FILTER,
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "CAN Control Register"]
pub mod control;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "CAN Status Register"]
pub mod status;
#[doc = "BITTMNG (rw) register accessor: an alias for `Reg<BITTMNG_SPEC>`"]
pub type BITTMNG = crate::Reg<bittmng::BITTMNG_SPEC>;
#[doc = "CAN Bittiming Register"]
pub mod bittmng;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "CAN Interrupt enable Register"]
pub mod int_en;
#[doc = "OVER (rw) register accessor: an alias for `Reg<OVER_SPEC>`"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = ""]
pub mod over;
#[doc = "RXID (rw) register accessor: an alias for `Reg<RXID_SPEC>`"]
pub type RXID = crate::Reg<rxid::RXID_SPEC>;
#[doc = "CAN Receive ID Register"]
pub mod rxid;
#[doc = "RXDLC (rw) register accessor: an alias for `Reg<RXDLC_SPEC>`"]
pub type RXDLC = crate::Reg<rxdlc::RXDLC_SPEC>;
#[doc = "CAN Receive DLC Register"]
pub mod rxdlc;
#[doc = "RXDATAL (rw) register accessor: an alias for `Reg<RXDATAL_SPEC>`"]
pub type RXDATAL = crate::Reg<rxdatal::RXDATAL_SPEC>;
#[doc = "CAN Receive Data low Register"]
pub mod rxdatal;
#[doc = "RXDATAH (rw) register accessor: an alias for `Reg<RXDATAH_SPEC>`"]
pub type RXDATAH = crate::Reg<rxdatah::RXDATAH_SPEC>;
#[doc = "CAN Receive Data high Register"]
pub mod rxdatah;
#[doc = "TXID (rw) register accessor: an alias for `Reg<TXID_SPEC>`"]
pub type TXID = crate::Reg<txid::TXID_SPEC>;
#[doc = "CAN Transmit ID Register"]
pub mod txid;
#[doc = "TXDLC (rw) register accessor: an alias for `Reg<TXDLC_SPEC>`"]
pub type TXDLC = crate::Reg<txdlc::TXDLC_SPEC>;
#[doc = "CAN Transmit DLC Register"]
pub mod txdlc;
#[doc = "DATAL (rw) register accessor: an alias for `Reg<DATAL_SPEC>`"]
pub type DATAL = crate::Reg<datal::DATAL_SPEC>;
#[doc = "CAN Transmit Data low Register"]
pub mod datal;
#[doc = "DATAH (rw) register accessor: an alias for `Reg<DATAH_SPEC>`"]
pub type DATAH = crate::Reg<datah::DATAH_SPEC>;
#[doc = "CAN Transmit Data high Register"]
pub mod datah;
#[doc = "BUF_CON00 (rw) register accessor: an alias for `Reg<BUF_CON00_SPEC>`"]
pub type BUF_CON00 = crate::Reg<buf_con00::BUF_CON00_SPEC>;
#[doc = "CAN Buffer Connection Register"]
pub mod buf_con00;
pub use buf_con00 as buf_con01;
pub use buf_con00 as buf_con02;
pub use buf_con00 as buf_con03;
pub use buf_con00 as buf_con04;
pub use buf_con00 as buf_con05;
pub use buf_con00 as buf_con06;
pub use buf_con00 as buf_con07;
pub use buf_con00 as buf_con08;
pub use buf_con00 as buf_con09;
pub use buf_con00 as buf_con10;
pub use buf_con00 as buf_con11;
pub use buf_con00 as buf_con12;
pub use buf_con00 as buf_con13;
pub use buf_con00 as buf_con14;
pub use buf_con00 as buf_con15;
pub use buf_con00 as buf_con16;
pub use buf_con00 as buf_con17;
pub use buf_con00 as buf_con18;
pub use buf_con00 as buf_con19;
pub use buf_con00 as buf_con20;
pub use buf_con00 as buf_con21;
pub use buf_con00 as buf_con22;
pub use buf_con00 as buf_con23;
pub use buf_con00 as buf_con24;
pub use buf_con00 as buf_con25;
pub use buf_con00 as buf_con26;
pub use buf_con00 as buf_con27;
pub use buf_con00 as buf_con28;
pub use buf_con00 as buf_con29;
pub use buf_con00 as buf_con30;
pub use buf_con00 as buf_con31;
pub use BUF_CON00 as BUF_CON01;
pub use BUF_CON00 as BUF_CON02;
pub use BUF_CON00 as BUF_CON03;
pub use BUF_CON00 as BUF_CON04;
pub use BUF_CON00 as BUF_CON05;
pub use BUF_CON00 as BUF_CON06;
pub use BUF_CON00 as BUF_CON07;
pub use BUF_CON00 as BUF_CON08;
pub use BUF_CON00 as BUF_CON09;
pub use BUF_CON00 as BUF_CON10;
pub use BUF_CON00 as BUF_CON11;
pub use BUF_CON00 as BUF_CON12;
pub use BUF_CON00 as BUF_CON13;
pub use BUF_CON00 as BUF_CON14;
pub use BUF_CON00 as BUF_CON15;
pub use BUF_CON00 as BUF_CON16;
pub use BUF_CON00 as BUF_CON17;
pub use BUF_CON00 as BUF_CON18;
pub use BUF_CON00 as BUF_CON19;
pub use BUF_CON00 as BUF_CON20;
pub use BUF_CON00 as BUF_CON21;
pub use BUF_CON00 as BUF_CON22;
pub use BUF_CON00 as BUF_CON23;
pub use BUF_CON00 as BUF_CON24;
pub use BUF_CON00 as BUF_CON25;
pub use BUF_CON00 as BUF_CON26;
pub use BUF_CON00 as BUF_CON27;
pub use BUF_CON00 as BUF_CON28;
pub use BUF_CON00 as BUF_CON29;
pub use BUF_CON00 as BUF_CON30;
pub use BUF_CON00 as BUF_CON31;
#[doc = "INT_RX (rw) register accessor: an alias for `Reg<INT_RX_SPEC>`"]
pub type INT_RX = crate::Reg<int_rx::INT_RX_SPEC>;
#[doc = ""]
pub mod int_rx;
#[doc = "RX (rw) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = ""]
pub mod rx;
#[doc = "INT_TX (rw) register accessor: an alias for `Reg<INT_TX_SPEC>`"]
pub type INT_TX = crate::Reg<int_tx::INT_TX_SPEC>;
#[doc = ""]
pub mod int_tx;
#[doc = "TX (rw) register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = ""]
pub mod tx;
#[doc = "BUF_00_ID (rw) register accessor: an alias for `Reg<BUF_00_ID_SPEC>`"]
pub type BUF_00_ID = crate::Reg<buf_00_id::BUF_00_ID_SPEC>;
#[doc = "CAN Buffer ID Register"]
pub mod buf_00_id;
#[doc = "BUF_00_DLC (rw) register accessor: an alias for `Reg<BUF_00_DLC_SPEC>`"]
pub type BUF_00_DLC = crate::Reg<buf_00_dlc::BUF_00_DLC_SPEC>;
#[doc = "CAN Buffer DLC Register"]
pub mod buf_00_dlc;
#[doc = "BUF_00_DATAL (rw) register accessor: an alias for `Reg<BUF_00_DATAL_SPEC>`"]
pub type BUF_00_DATAL = crate::Reg<buf_00_datal::BUF_00_DATAL_SPEC>;
#[doc = "CAN Buffer Data low Register"]
pub mod buf_00_datal;
#[doc = "BUF_00_DATAH (rw) register accessor: an alias for `Reg<BUF_00_DATAH_SPEC>`"]
pub type BUF_00_DATAH = crate::Reg<buf_00_datah::BUF_00_DATAH_SPEC>;
#[doc = "CAN Buffer Data high Register"]
pub mod buf_00_datah;
pub use buf_00_datah as buf_01_datah;
pub use buf_00_datah as buf_02_datah;
pub use buf_00_datah as buf_03_datah;
pub use buf_00_datah as buf_04_datah;
pub use buf_00_datah as buf_05_datah;
pub use buf_00_datah as buf_06_datah;
pub use buf_00_datah as buf_07_datah;
pub use buf_00_datah as buf_08_datah;
pub use buf_00_datah as buf_09_datah;
pub use buf_00_datah as buf_10_datah;
pub use buf_00_datah as buf_11_datah;
pub use buf_00_datah as buf_12_datah;
pub use buf_00_datah as buf_13_datah;
pub use buf_00_datah as buf_14_datah;
pub use buf_00_datah as buf_15_datah;
pub use buf_00_datah as buf_16_datah;
pub use buf_00_datah as buf_17_datah;
pub use buf_00_datah as buf_18_datah;
pub use buf_00_datah as buf_19_datah;
pub use buf_00_datah as buf_20_datah;
pub use buf_00_datah as buf_21_datah;
pub use buf_00_datah as buf_22_datah;
pub use buf_00_datah as buf_23_datah;
pub use buf_00_datah as buf_24_datah;
pub use buf_00_datah as buf_25_datah;
pub use buf_00_datah as buf_26_datah;
pub use buf_00_datah as buf_27_datah;
pub use buf_00_datah as buf_28_datah;
pub use buf_00_datah as buf_29_datah;
pub use buf_00_datah as buf_30_datah;
pub use buf_00_datah as buf_31_datah;
pub use buf_00_datal as buf_01_datal;
pub use buf_00_datal as buf_02_datal;
pub use buf_00_datal as buf_03_datal;
pub use buf_00_datal as buf_04_datal;
pub use buf_00_datal as buf_05_datal;
pub use buf_00_datal as buf_06_datal;
pub use buf_00_datal as buf_07_datal;
pub use buf_00_datal as buf_08_datal;
pub use buf_00_datal as buf_09_datal;
pub use buf_00_datal as buf_10_datal;
pub use buf_00_datal as buf_11_datal;
pub use buf_00_datal as buf_12_datal;
pub use buf_00_datal as buf_13_datal;
pub use buf_00_datal as buf_14_datal;
pub use buf_00_datal as buf_15_datal;
pub use buf_00_datal as buf_16_datal;
pub use buf_00_datal as buf_17_datal;
pub use buf_00_datal as buf_18_datal;
pub use buf_00_datal as buf_19_datal;
pub use buf_00_datal as buf_20_datal;
pub use buf_00_datal as buf_21_datal;
pub use buf_00_datal as buf_22_datal;
pub use buf_00_datal as buf_23_datal;
pub use buf_00_datal as buf_24_datal;
pub use buf_00_datal as buf_25_datal;
pub use buf_00_datal as buf_26_datal;
pub use buf_00_datal as buf_27_datal;
pub use buf_00_datal as buf_28_datal;
pub use buf_00_datal as buf_29_datal;
pub use buf_00_datal as buf_30_datal;
pub use buf_00_datal as buf_31_datal;
pub use buf_00_dlc as buf_01_dlc;
pub use buf_00_dlc as buf_02_dlc;
pub use buf_00_dlc as buf_03_dlc;
pub use buf_00_dlc as buf_04_dlc;
pub use buf_00_dlc as buf_05_dlc;
pub use buf_00_dlc as buf_06_dlc;
pub use buf_00_dlc as buf_07_dlc;
pub use buf_00_dlc as buf_08_dlc;
pub use buf_00_dlc as buf_09_dlc;
pub use buf_00_dlc as buf_10_dlc;
pub use buf_00_dlc as buf_11_dlc;
pub use buf_00_dlc as buf_12_dlc;
pub use buf_00_dlc as buf_13_dlc;
pub use buf_00_dlc as buf_14_dlc;
pub use buf_00_dlc as buf_15_dlc;
pub use buf_00_dlc as buf_16_dlc;
pub use buf_00_dlc as buf_17_dlc;
pub use buf_00_dlc as buf_18_dlc;
pub use buf_00_dlc as buf_19_dlc;
pub use buf_00_dlc as buf_20_dlc;
pub use buf_00_dlc as buf_21_dlc;
pub use buf_00_dlc as buf_22_dlc;
pub use buf_00_dlc as buf_23_dlc;
pub use buf_00_dlc as buf_24_dlc;
pub use buf_00_dlc as buf_25_dlc;
pub use buf_00_dlc as buf_26_dlc;
pub use buf_00_dlc as buf_27_dlc;
pub use buf_00_dlc as buf_28_dlc;
pub use buf_00_dlc as buf_29_dlc;
pub use buf_00_dlc as buf_30_dlc;
pub use buf_00_dlc as buf_31_dlc;
pub use buf_00_id as buf_01_id;
pub use buf_00_id as buf_02_id;
pub use buf_00_id as buf_03_id;
pub use buf_00_id as buf_04_id;
pub use buf_00_id as buf_05_id;
pub use buf_00_id as buf_06_id;
pub use buf_00_id as buf_07_id;
pub use buf_00_id as buf_08_id;
pub use buf_00_id as buf_09_id;
pub use buf_00_id as buf_10_id;
pub use buf_00_id as buf_11_id;
pub use buf_00_id as buf_12_id;
pub use buf_00_id as buf_13_id;
pub use buf_00_id as buf_14_id;
pub use buf_00_id as buf_15_id;
pub use buf_00_id as buf_16_id;
pub use buf_00_id as buf_17_id;
pub use buf_00_id as buf_18_id;
pub use buf_00_id as buf_19_id;
pub use buf_00_id as buf_20_id;
pub use buf_00_id as buf_21_id;
pub use buf_00_id as buf_22_id;
pub use buf_00_id as buf_23_id;
pub use buf_00_id as buf_24_id;
pub use buf_00_id as buf_25_id;
pub use buf_00_id as buf_26_id;
pub use buf_00_id as buf_27_id;
pub use buf_00_id as buf_28_id;
pub use buf_00_id as buf_29_id;
pub use buf_00_id as buf_30_id;
pub use buf_00_id as buf_31_id;
pub use BUF_00_DATAH as BUF_01_DATAH;
pub use BUF_00_DATAH as BUF_02_DATAH;
pub use BUF_00_DATAH as BUF_03_DATAH;
pub use BUF_00_DATAH as BUF_04_DATAH;
pub use BUF_00_DATAH as BUF_05_DATAH;
pub use BUF_00_DATAH as BUF_06_DATAH;
pub use BUF_00_DATAH as BUF_07_DATAH;
pub use BUF_00_DATAH as BUF_08_DATAH;
pub use BUF_00_DATAH as BUF_09_DATAH;
pub use BUF_00_DATAH as BUF_10_DATAH;
pub use BUF_00_DATAH as BUF_11_DATAH;
pub use BUF_00_DATAH as BUF_12_DATAH;
pub use BUF_00_DATAH as BUF_13_DATAH;
pub use BUF_00_DATAH as BUF_14_DATAH;
pub use BUF_00_DATAH as BUF_15_DATAH;
pub use BUF_00_DATAH as BUF_16_DATAH;
pub use BUF_00_DATAH as BUF_17_DATAH;
pub use BUF_00_DATAH as BUF_18_DATAH;
pub use BUF_00_DATAH as BUF_19_DATAH;
pub use BUF_00_DATAH as BUF_20_DATAH;
pub use BUF_00_DATAH as BUF_21_DATAH;
pub use BUF_00_DATAH as BUF_22_DATAH;
pub use BUF_00_DATAH as BUF_23_DATAH;
pub use BUF_00_DATAH as BUF_24_DATAH;
pub use BUF_00_DATAH as BUF_25_DATAH;
pub use BUF_00_DATAH as BUF_26_DATAH;
pub use BUF_00_DATAH as BUF_27_DATAH;
pub use BUF_00_DATAH as BUF_28_DATAH;
pub use BUF_00_DATAH as BUF_29_DATAH;
pub use BUF_00_DATAH as BUF_30_DATAH;
pub use BUF_00_DATAH as BUF_31_DATAH;
pub use BUF_00_DATAL as BUF_01_DATAL;
pub use BUF_00_DATAL as BUF_02_DATAL;
pub use BUF_00_DATAL as BUF_03_DATAL;
pub use BUF_00_DATAL as BUF_04_DATAL;
pub use BUF_00_DATAL as BUF_05_DATAL;
pub use BUF_00_DATAL as BUF_06_DATAL;
pub use BUF_00_DATAL as BUF_07_DATAL;
pub use BUF_00_DATAL as BUF_08_DATAL;
pub use BUF_00_DATAL as BUF_09_DATAL;
pub use BUF_00_DATAL as BUF_10_DATAL;
pub use BUF_00_DATAL as BUF_11_DATAL;
pub use BUF_00_DATAL as BUF_12_DATAL;
pub use BUF_00_DATAL as BUF_13_DATAL;
pub use BUF_00_DATAL as BUF_14_DATAL;
pub use BUF_00_DATAL as BUF_15_DATAL;
pub use BUF_00_DATAL as BUF_16_DATAL;
pub use BUF_00_DATAL as BUF_17_DATAL;
pub use BUF_00_DATAL as BUF_18_DATAL;
pub use BUF_00_DATAL as BUF_19_DATAL;
pub use BUF_00_DATAL as BUF_20_DATAL;
pub use BUF_00_DATAL as BUF_21_DATAL;
pub use BUF_00_DATAL as BUF_22_DATAL;
pub use BUF_00_DATAL as BUF_23_DATAL;
pub use BUF_00_DATAL as BUF_24_DATAL;
pub use BUF_00_DATAL as BUF_25_DATAL;
pub use BUF_00_DATAL as BUF_26_DATAL;
pub use BUF_00_DATAL as BUF_27_DATAL;
pub use BUF_00_DATAL as BUF_28_DATAL;
pub use BUF_00_DATAL as BUF_29_DATAL;
pub use BUF_00_DATAL as BUF_30_DATAL;
pub use BUF_00_DATAL as BUF_31_DATAL;
pub use BUF_00_DLC as BUF_01_DLC;
pub use BUF_00_DLC as BUF_02_DLC;
pub use BUF_00_DLC as BUF_03_DLC;
pub use BUF_00_DLC as BUF_04_DLC;
pub use BUF_00_DLC as BUF_05_DLC;
pub use BUF_00_DLC as BUF_06_DLC;
pub use BUF_00_DLC as BUF_07_DLC;
pub use BUF_00_DLC as BUF_08_DLC;
pub use BUF_00_DLC as BUF_09_DLC;
pub use BUF_00_DLC as BUF_10_DLC;
pub use BUF_00_DLC as BUF_11_DLC;
pub use BUF_00_DLC as BUF_12_DLC;
pub use BUF_00_DLC as BUF_13_DLC;
pub use BUF_00_DLC as BUF_14_DLC;
pub use BUF_00_DLC as BUF_15_DLC;
pub use BUF_00_DLC as BUF_16_DLC;
pub use BUF_00_DLC as BUF_17_DLC;
pub use BUF_00_DLC as BUF_18_DLC;
pub use BUF_00_DLC as BUF_19_DLC;
pub use BUF_00_DLC as BUF_20_DLC;
pub use BUF_00_DLC as BUF_21_DLC;
pub use BUF_00_DLC as BUF_22_DLC;
pub use BUF_00_DLC as BUF_23_DLC;
pub use BUF_00_DLC as BUF_24_DLC;
pub use BUF_00_DLC as BUF_25_DLC;
pub use BUF_00_DLC as BUF_26_DLC;
pub use BUF_00_DLC as BUF_27_DLC;
pub use BUF_00_DLC as BUF_28_DLC;
pub use BUF_00_DLC as BUF_29_DLC;
pub use BUF_00_DLC as BUF_30_DLC;
pub use BUF_00_DLC as BUF_31_DLC;
pub use BUF_00_ID as BUF_01_ID;
pub use BUF_00_ID as BUF_02_ID;
pub use BUF_00_ID as BUF_03_ID;
pub use BUF_00_ID as BUF_04_ID;
pub use BUF_00_ID as BUF_05_ID;
pub use BUF_00_ID as BUF_06_ID;
pub use BUF_00_ID as BUF_07_ID;
pub use BUF_00_ID as BUF_08_ID;
pub use BUF_00_ID as BUF_09_ID;
pub use BUF_00_ID as BUF_10_ID;
pub use BUF_00_ID as BUF_11_ID;
pub use BUF_00_ID as BUF_12_ID;
pub use BUF_00_ID as BUF_13_ID;
pub use BUF_00_ID as BUF_14_ID;
pub use BUF_00_ID as BUF_15_ID;
pub use BUF_00_ID as BUF_16_ID;
pub use BUF_00_ID as BUF_17_ID;
pub use BUF_00_ID as BUF_18_ID;
pub use BUF_00_ID as BUF_19_ID;
pub use BUF_00_ID as BUF_20_ID;
pub use BUF_00_ID as BUF_21_ID;
pub use BUF_00_ID as BUF_22_ID;
pub use BUF_00_ID as BUF_23_ID;
pub use BUF_00_ID as BUF_24_ID;
pub use BUF_00_ID as BUF_25_ID;
pub use BUF_00_ID as BUF_26_ID;
pub use BUF_00_ID as BUF_27_ID;
pub use BUF_00_ID as BUF_28_ID;
pub use BUF_00_ID as BUF_29_ID;
pub use BUF_00_ID as BUF_30_ID;
pub use BUF_00_ID as BUF_31_ID;
#[doc = "BUF_FILTER00_MASK (rw) register accessor: an alias for `Reg<BUF_FILTER00_MASK_SPEC>`"]
pub type BUF_FILTER00_MASK = crate::Reg<buf_filter00_mask::BUF_FILTER00_MASK_SPEC>;
#[doc = ""]
pub mod buf_filter00_mask;
#[doc = "BUF_FILTER00_FILTER (rw) register accessor: an alias for `Reg<BUF_FILTER00_FILTER_SPEC>`"]
pub type BUF_FILTER00_FILTER = crate::Reg<buf_filter00_filter::BUF_FILTER00_FILTER_SPEC>;
#[doc = ""]
pub mod buf_filter00_filter;
pub use buf_filter00_filter as buf_filter01_filter;
pub use buf_filter00_filter as buf_filter02_filter;
pub use buf_filter00_filter as buf_filter03_filter;
pub use buf_filter00_filter as buf_filter04_filter;
pub use buf_filter00_filter as buf_filter05_filter;
pub use buf_filter00_filter as buf_filter06_filter;
pub use buf_filter00_filter as buf_filter07_filter;
pub use buf_filter00_filter as buf_filter08_filter;
pub use buf_filter00_filter as buf_filter09_filter;
pub use buf_filter00_filter as buf_filter10_filter;
pub use buf_filter00_filter as buf_filter11_filter;
pub use buf_filter00_filter as buf_filter12_filter;
pub use buf_filter00_filter as buf_filter13_filter;
pub use buf_filter00_filter as buf_filter14_filter;
pub use buf_filter00_filter as buf_filter15_filter;
pub use buf_filter00_filter as buf_filter16_filter;
pub use buf_filter00_filter as buf_filter17_filter;
pub use buf_filter00_filter as buf_filter18_filter;
pub use buf_filter00_filter as buf_filter19_filter;
pub use buf_filter00_filter as buf_filter20_filter;
pub use buf_filter00_filter as buf_filter21_filter;
pub use buf_filter00_filter as buf_filter22_filter;
pub use buf_filter00_filter as buf_filter23_filter;
pub use buf_filter00_filter as buf_filter24_filter;
pub use buf_filter00_filter as buf_filter25_filter;
pub use buf_filter00_filter as buf_filter26_filter;
pub use buf_filter00_filter as buf_filter27_filter;
pub use buf_filter00_filter as buf_filter28_filter;
pub use buf_filter00_filter as buf_filter29_filter;
pub use buf_filter00_filter as buf_filter30_filter;
pub use buf_filter00_filter as buf_filter31_filter;
pub use buf_filter00_mask as buf_filter01_mask;
pub use buf_filter00_mask as buf_filter02_mask;
pub use buf_filter00_mask as buf_filter03_mask;
pub use buf_filter00_mask as buf_filter04_mask;
pub use buf_filter00_mask as buf_filter05_mask;
pub use buf_filter00_mask as buf_filter06_mask;
pub use buf_filter00_mask as buf_filter07_mask;
pub use buf_filter00_mask as buf_filter08_mask;
pub use buf_filter00_mask as buf_filter09_mask;
pub use buf_filter00_mask as buf_filter10_mask;
pub use buf_filter00_mask as buf_filter11_mask;
pub use buf_filter00_mask as buf_filter12_mask;
pub use buf_filter00_mask as buf_filter13_mask;
pub use buf_filter00_mask as buf_filter14_mask;
pub use buf_filter00_mask as buf_filter15_mask;
pub use buf_filter00_mask as buf_filter16_mask;
pub use buf_filter00_mask as buf_filter17_mask;
pub use buf_filter00_mask as buf_filter18_mask;
pub use buf_filter00_mask as buf_filter19_mask;
pub use buf_filter00_mask as buf_filter20_mask;
pub use buf_filter00_mask as buf_filter21_mask;
pub use buf_filter00_mask as buf_filter22_mask;
pub use buf_filter00_mask as buf_filter23_mask;
pub use buf_filter00_mask as buf_filter24_mask;
pub use buf_filter00_mask as buf_filter25_mask;
pub use buf_filter00_mask as buf_filter26_mask;
pub use buf_filter00_mask as buf_filter27_mask;
pub use buf_filter00_mask as buf_filter28_mask;
pub use buf_filter00_mask as buf_filter29_mask;
pub use buf_filter00_mask as buf_filter30_mask;
pub use buf_filter00_mask as buf_filter31_mask;
pub use BUF_FILTER00_FILTER as BUF_FILTER01_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER02_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER03_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER04_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER05_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER06_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER07_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER08_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER09_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER10_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER11_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER12_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER13_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER14_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER15_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER16_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER17_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER18_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER19_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER20_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER21_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER22_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER23_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER24_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER25_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER26_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER27_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER28_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER29_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER30_FILTER;
pub use BUF_FILTER00_FILTER as BUF_FILTER31_FILTER;
pub use BUF_FILTER00_MASK as BUF_FILTER01_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER02_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER03_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER04_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER05_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER06_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER07_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER08_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER09_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER10_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER11_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER12_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER13_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER14_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER15_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER16_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER17_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER18_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER19_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER20_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER21_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER22_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER23_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER24_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER25_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER26_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER27_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER28_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER29_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER30_MASK;
pub use BUF_FILTER00_MASK as BUF_FILTER31_MASK;
