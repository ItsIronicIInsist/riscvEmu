# riscvEmu
riscv emulator in rust

Project to become more familiar with both rust and assembly

Aiming to have full support for RV64G (IMAFD+csr) + some hardware peripherals

Single threader so fence and atomic instructions arent proper implementations


Currently working on:

RV64-M (testing)

RV64-F (testing)

Currently have support for:
RV64I (Exluding fence, ecall, ebreak)
