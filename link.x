PATH_SIZE   = 1024;
PARAMS_SIZE =  256;
STACK_SIZE  = 1024;

OUTPUT_FORMAT("binary")

SECTIONS
{
. = 0x24;
  .text : {
    *(.text)
    *(.text.*)
  }
END = .;
  .hdr : AT(0){
    LONG(0x554e454D);
    LONG(0x31305445);
    LONG(1);
    LONG(kol_main);
    LONG(END);
    LONG(FILE_END + PATH_SIZE + PARAMS_SIZE + STACK_SIZE);
    LONG(FILE_END + PATH_SIZE + PARAMS_SIZE + STACK_SIZE);
    LONG(FILE_END + PATH_SIZE);
    LONG(FILE_END);
  }
  .dat ALIGN(16) : {
    *(.rdata)
    *(const)
    *(CONST)
    *(.data)
    *(data)
  }
  .bss ALIGN(16) : {*(.bss)}
FILE_END = .;
}
