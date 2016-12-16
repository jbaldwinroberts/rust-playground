#![feature(asm, lang_items)]
#![no_main]
#![no_std]

/// Start address of the GPIOA register block
const GPIOA: u32 = 0x4002_0000;

/// Offset address of the BSRR register
const GPIOA_BSRR: u32 = 0x18;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    power_on_gpioa();
    put_pa5_in_output_mode();

    let ticks = 25_000;

    loop {
        set_pa5_high();
        delay(ticks);
        set_pa5_low();
        delay(ticks);
    }
}

fn delay(n: u32) {
    for _ in 0..n {}
}

fn power_on_gpioa() {
    /// Start address of the RCC register block
    const RCC: u32 = 0x4002_3800;

    /// Offset address of the AHB1ENR register
    const RCC_AHB1ENR: u32 = 0x30;

    /// GPIOAEN bit mask
    const RCC_AHB1ENR_GPIOAEN: u32 = 1 << 0;

    unsafe {
        // Pointer to the AHB1ENR register
        let ahb1enr = (RCC + RCC_AHB1ENR) as *mut u32;

        // GPIOAEN = 1
        *ahb1enr |= RCC_AHB1ENR_GPIOAEN;
    }
}

fn put_pa5_in_output_mode() {
    /// Offset address of the CRH register
    const GPIOA_MODER: u32 = 0x0;

    unsafe {
        // Pointer to the MODER register
        let moder = (GPIOA + GPIOA_MODER) as *mut u32;

        // MODER5 = 0b01
        *moder = (*moder & !(0b11 << 10)) | (0b01 << 10);
    }
}

fn set_pa5_high() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOA + GPIOA_BSRR) as *mut u32;

        // BS5 = 1
        *bsrr = 1 << 5;
    }
}

fn set_pa5_low() {
    unsafe {
        // Pointer to the BSRR register
        let bsrr = (GPIOA + GPIOA_BSRR) as *mut u32;

        // BR5 = 1
        *bsrr = 1 << (16 + 5);
    }
}

mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}

mod exception {
    pub extern "C" fn handler() {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }

    #[export_name = "_EXCEPTIONS"]
    pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [Some(handler), // NMI
                                                            Some(handler), // Hard fault
                                                            Some(handler), // Memmanage fault
                                                            Some(handler), // Bus fault
                                                            Some(handler), // Usage fault
                                                            None, // Reserved
                                                            None, // Reserved
                                                            None, // Reserved
                                                            None, // Reserved
                                                            Some(handler), // SVCall
                                                            None, // Reserved for Debug
                                                            None, // Reserved
                                                            Some(handler), // PendSV
                                                            Some(handler)]; // Systick
}
