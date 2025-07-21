#ifndef UXN11_H
#define UXN11_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Register a Rust callback for port events
void register_port_callback(void *cb);

// Called from emu_deo for all port events
void rust_deo_all_ports_hook(uint8_t port, uint8_t value);

// Entry point for running the emulator
int uxn11_entry(int argc, char **argv);

#ifdef __cplusplus
}
#endif

#endif // UXN11_H
