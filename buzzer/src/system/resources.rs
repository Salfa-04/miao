//!
//! # System Resources
//!
//! ## Reserved Resources
//! - PA13: SWDIO
//! - PA14: SWCLK
//!
//! - PH0: OSC_IN
//! - PH1: OSC_OUT
//!

use super::private::*;

assign_resources! {
    /// for `Blinky` task.
    blinky: BlinkySrc {
        spi_p: SPI6,
        led_pin: PA7,
        dma: BDMA_CH0,
    }

    buzzer: BuzzerSrc {
        tim_p: TIM12,
        buzz_pin: PB15, // CH2
    }

    usb: UsbSrc {
        usb_p: USB_OTG_HS,
        usb_dm: PA11,
        usb_dp: PA12,
    }

    bat: BatSrc {
        adc_p: ADC1,
        vbat: PC4, // IN4 /11
        dma: DMA1_CH7,

        power_5v_en: PC15,
        power_up_en: PC14,
        power_down_en: PC13,

        user_key: PA15,
    }

    sbus: SbusSrc {
        uart_p: UART5,
        uart_rx: PD2,
        dma: DMA2_CH7,
    }

    flash: FlashSrc {
        qspi_p: OCTOSPI1,
        qspi_ncs: PE11,
        qspi_clk: PB2,
        qspi_io0: PD11,
        qspi_io1: PB0,
        qspi_io2: PA3,
        qspi_io3: PA1,
        // dma: MDMA_CH0
    }

    pwm: PwmSrc {
        tim1_p: TIM1,
        pwm_1: PE13, // CH3
        pwm_2: PE9, // CH1

        tim2_p: TIM2,
        pwm_3: PA2, // CH3
        pwm_4: PA0, // CH1
    }

    fdcan: FdCanSrc {
        fdcan1_p: FDCAN1,
        fdcan1_rx: PD0,
        fdcan1_tx: PD1,

        fdcan2_p: FDCAN2,
        fdcan2_rx: PB5,
        fdcan2_tx: PB6,

        fdcan3_p: FDCAN3,
        fdcan3_rx: PD12,
        fdcan3_tx: PD13,
    }

    imu: ImuSrc {
        spi_p: SPI2,
        spi_sck: PB13,
        spi_mosi: PC1,
        spi_miso: PC2,
        dma_rx: DMA1_CH0,
        dma_tx: DMA2_CH0,

        heat_p: TIM3,
        heat_pin: PB1, // CH4

        acc_int: PE10,
        acc_exti: EXTI10,
        acc_cs: PC0,

        gyro_int: PE12,
        gyro_exti: EXTI12,
        gyro_cs: PC3,
    }

    uart1: Uart1Src {
        usart_p: USART1,
        usart_rx: PA10,
        usart_tx: PA9,
        dma_rx: DMA1_CH1,
        dma_tx: DMA2_CH1,
    }

    uart7: Uart7Src {
        uart_p: UART7,
        uart_rx: PE7,
        uart_tx: PE8,
        dma_rx: DMA1_CH2,
        dma_tx: DMA2_CH2,
    }

    uart10: Uart10Src {
        usart_p: USART10,
        usart_rx: PE2,
        usart_tx: PE3,
        dma_rx: DMA1_CH3,
        dma_tx: DMA2_CH3,
    }

    rs485u2: Rs485U2Src {
        usart_p: USART2,
        usart_rx: PD6,
        usart_tx: PD5,
        usart_de: PD4,
        dma_rx: DMA1_CH4,
        dma_tx: DMA2_CH4,
    }

    rs485u3: Rs485U3Src {
        usart_p: USART3,
        usart_rx: PD9,
        usart_tx: PD8,
        usart_de: PB14,
        dma_rx: DMA1_CH5,
        dma_tx: DMA2_CH5,
    }
}
