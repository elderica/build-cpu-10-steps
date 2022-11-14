all:
	@echo make flash

flash: impl/pnr/build-cpu-10-steps.fs
	openFPGALoader -b tangnano9k -f impl/pnr/build-cpu-10-steps.fs
.PHONY: flash
