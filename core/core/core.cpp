#pragma comment(lib, "legacy_stdio_definitions.lib")

#include <Windows.h>
#include "hde64.h"

#ifdef __cplusplus
extern "C" {
#endif

    void helloworld();

#ifdef __cplusplus
};
#endif

extern "C" hde64s sethook(uintptr_t fn_addr) {
}

extern "C" unsigned int disasm_c(void* fn_addr,hde64s* hs) {
    return hde64_disasm((void*)fn_addr, hs);
}

extern "C" void fnasm()
{
    helloworld();
}
