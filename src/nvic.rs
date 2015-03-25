use volatile::RW;

#[repr(C)]
pub struct Nvic {
    /// Interrupt set-enable registers
    /* 0x0004 */ iser0: RW<u32>,
    /* 0x0008 */ pub iser1: RW<iser1::Register>,
    /* 0x000C */ iser2: RW<u32>,
    /* 0x0010 */ _0: [u32; 29],
    /// Interrupt clear-enable registers
    /* 0x0004 */ icer0: RW<u32>,
    /* 0x0008 */ icer1: RW<u32>,
    /* 0x000C */ icer2: RW<u32>,
    /* 0x0010 */ _1: [u32; 29],
    /// Interrupt set-pending registers
    /* 0x0004 */ ispr0: RW<u32>,
    /* 0x0008 */ ispr1: RW<u32>,
    /* 0x000C */ ispr2: RW<u32>,
    /* 0x0010 */ _2: [u32; 29],
    /// Interrupt clear-pending registers
    /* 0x0004 */ icpr0: RW<u32>,
    /* 0x0008 */ icpr1: RW<u32>,
    /* 0x000C */ icpr2: RW<u32>,
    /* 0x0010 */ _3: [u32; 29],
}

reg!(iser1: u32 {
    TIM7: 23,
});
