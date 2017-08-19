# Credits to Elliot Williams (hexagon5un) for this wonderfully written makefile
# Small makefile modifications by Arko

MCU   = atmega328p
F_CPU = 1000000UL  
BAUD  = 9600UL

LIBDIR = /

PROGRAMMER_TYPE = dragon_isp
PROGRAMMER_ARGS = 	

CC = avr-gcc
OBJCOPY = avr-objcopy
OBJDUMP = avr-objdump
AVRSIZE = avr-size
AVRDUDE = avrdude

TARGET = habexpico

SOURCES=$(wildcard *.c $(LIBDIR)/*.c)
OBJECTS=$(SOURCES:.c=.o)
HEADERS=$(SOURCES:.c=.h)

CPPFLAGS = -DF_CPU=$(F_CPU) -DBAUD=$(BAUD) -I. -I$(LIBDIR)
CFLAGS = -Os -g -std=gnu99 -Wall 
CFLAGS += -funsigned-char -funsigned-bitfields -fpack-struct -fshort-enums 
CFLAGS += -ffunction-sections -fdata-sections 
LDFLAGS = -Wl,-Map,$(TARGET).map 
LDFLAGS += -Wl,--gc-sections 
TARGET_ARCH = -mmcu=$(MCU)

%.o: %.c $(HEADERS) Makefile
	 $(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c -o $@ $<;

$(TARGET).elf: $(OBJECTS)
	$(CC) $(LDFLAGS) $(TARGET_ARCH) $^ $(LDLIBS) -o $@

%.hex: %.elf
	 $(OBJCOPY) -j .text -j .data -O ihex $< $@

%.eeprom: %.elf
	$(OBJCOPY) -j .eeprom --change-section-lma .eeprom=0 -O ihex $< $@ 

%.lst: %.elf
	$(OBJDUMP) -S $< > $@

.PHONY: all disassemble disasm eeprom size clean squeaky_clean flash fuses

all: $(TARGET).hex 

debug:
	@echo
	@echo "Source files:"   $(SOURCES)
	@echo "MCU, F_CPU, BAUD:"  $(MCU), $(F_CPU), $(BAUD)
	@echo	

disassemble: $(TARGET).lst

disasm: disassemble

size:  $(TARGET).elf
	$(AVRSIZE) -C --mcu=$(MCU) $(TARGET).elf

clean:
	rm -f $(TARGET).elf $(TARGET).hex $(TARGET).obj \
	$(TARGET).o $(TARGET).d $(TARGET).eep $(TARGET).lst \
	$(TARGET).lss $(TARGET).sym $(TARGET).map $(TARGET)~ \
	$(TARGET).eeprom

cleanall:
	rm -f *.elf *.hex *.obj *.o *.d *.eep *.lst *.lss *.sym *.map *~ *.eeprom

flash: $(TARGET).hex 
	$(AVRDUDE) -c $(PROGRAMMER_TYPE) -p $(MCU) $(PROGRAMMER_ARGS) -U flash:w:$<

program: flash

flash_eeprom: $(TARGET).eeprom
	$(AVRDUDE) -c $(PROGRAMMER_TYPE) -p $(MCU) $(PROGRAMMER_ARGS) -U eeprom:w:$<

avrdude_terminal:
	$(AVRDUDE) -c $(PROGRAMMER_TYPE) -p $(MCU) $(PROGRAMMER_ARGS) -nt

flash_usbtiny: PROGRAMMER_TYPE = usbtiny
flash_usbtiny: PROGRAMMER_ARGS =  # USBTiny works with no further arguments
flash_usbtiny: flash

flash_usbasp: PROGRAMMER_TYPE = usbasp
flash_usbasp: PROGRAMMER_ARGS =  # USBasp works with no further arguments
flash_usbasp: flash

flash_dragon: PROGRAMMER_TYPE = dragon_avr
flash_dragon: PROGRAMMER_ARGS =  # Dragon AVR works with no further arguments
flash_dragon: flash

flash_arduinoISP: PROGRAMMER_TYPE = avrisp
flash_arduinoISP: PROGRAMMER_ARGS = -b 19200 -P /dev/ttyACM0 
flash_arduinoISP: flash

flash_109: PROGRAMMER_TYPE = avr109
flash_109: PROGRAMMER_ARGS = -b 9600 -P /dev/ttyUSB0
flash_109: flash


LFUSE = 0x62
HFUSE = 0xD8
EFUSE = 0xFF

FUSE_STRING = -U lfuse:w:$(LFUSE):m -U hfuse:w:$(HFUSE):m -U efuse:w:$(EFUSE):m 

fuses: 
	$(AVRDUDE) -c $(PROGRAMMER_TYPE) -p $(MCU) \
	           $(PROGRAMMER_ARGS) $(FUSE_STRING)
show_fuses:
	$(AVRDUDE) -c $(PROGRAMMER_TYPE) -p $(MCU) $(PROGRAMMER_ARGS) -nv	

set_default_fuses:  FUSE_STRING = -U lfuse:w:$(LFUSE):m -U hfuse:w:$(HFUSE):m -U efuse:w:$(EFUSE):m 
set_default_fuses:  fuses

set_fast_fuse: LFUSE = 0xE2
set_fast_fuse: FUSE_STRING = -U lfuse:w:$(LFUSE):m 
set_fast_fuse: fuses

set_eeprom_save_fuse: HFUSE = 0xD7
set_eeprom_save_fuse: FUSE_STRING = -U hfuse:w:$(HFUSE):m
set_eeprom_save_fuse: fuses

clear_eeprom_save_fuse: FUSE_STRING = -U hfuse:w:$(HFUSE):m
clear_eeprom_save_fuse: fuses