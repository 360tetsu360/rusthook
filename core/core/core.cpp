#pragma comment(lib, "legacy_stdio_definitions.lib")

#include "framework.h"

#ifdef __cplusplus
extern "C" {
#endif

    void helloworld();

#ifdef __cplusplus
};
#endif

extern "C" void fncore()
{
    helloworld();
}
