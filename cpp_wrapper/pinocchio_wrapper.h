#pragma once
#ifdef __cplusplus
extern "C" {
#endif

typedef struct PinHandle PinHandle;

PinHandle* pin_create_manipulator();
void       pin_destroy(PinHandle* h);
void       pin_set_state(PinHandle* h, const double* q, const double* v, const double* a);
void       pin_compute_rnea(PinHandle* h, double* tau_out);
int        pin_get_nv(PinHandle* h);

#ifdef __cplusplus
}
#endif

