#include <atomic>
#include <vulkan/vulkan.h>

extern "C" {
    typedef void (*VpDebugMessageCallbackFn)(const char*);
}

static std::atomic<void*> ALEPH_ATOMIC_CALLBACK_SLOT{nullptr};

extern "C" {
    void vpAlephSetCallback(VpDebugMessageCallbackFn ptr) {
        ALEPH_ATOMIC_CALLBACK_SLOT.exchange((void*)ptr);
    }
}

void vpAlephDebugMessageTrampoline(const char* message) {
    auto* ptr = (VpDebugMessageCallbackFn)ALEPH_ATOMIC_CALLBACK_SLOT.load();
    if (ptr) {
        ptr(message);
    }
}