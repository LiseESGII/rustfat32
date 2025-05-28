.global _start
.section .text
_start:
    call rust_main
    cli
    hlt

// Optionnel : point d’entrée Rust
.global rust_main
rust_main:
    // Appelle la fonction _start de Rust
    call _start
    ret
