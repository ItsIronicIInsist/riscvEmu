# riscvEmu
riscv emulator in rust

Project to become more familiar with both rust and riscv assembly

Aiming to have full support for RV64G (IMAFD+csr) + some hardware peripherals

Single threaded so fence and atomic instructions arent proper implementations


Currently working on:

RV64-CSR (testing)

Implementing privilege levels

RV64-F (implementing edge stuff)

Currently have support for:
RV64I (Exluding fence, ecall, ebreak)

RV64M

RV64A (Emulator is single threaded so not 'truly' atomic
