global get_boot_information

section .text
bits 64

;; Get a boot information struct
;; from GRUB by type
get_boot_information:
    ;; Both the arg and return are
    ;; handled in the al register
    ; Arg: u32 type of the data structure
    ; Return: the start address of the struct
    inc al