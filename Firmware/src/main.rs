#![no_std]
#![no_main]
#[macro_use(entry,exception)]
extern crate cortex_m_rt;

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;

extern crate stm32l0;
use stm32l0::stm32l0x1;

entry!(main);
exception!(SysTick, tick);

fn main() -> ! {
	let peripherals = stm32l0x1::Peripherals::take().unwrap();
	let gpioa = &peripherals.GPIOA;
	gpioa.odr.modify(|_, w| w.od1().set_bit());
	
	loop {
        cortex_m::asm::wfi();
    }
}

fn tick() {
    
}