LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE := vma

LOCAL_C_INCLUDES += $(LOCAL_PATH)/../Vulkan-Headers/include
LOCAL_C_INCLUDES += $(LOCAL_PATH)/../VulkanMemoryAllocator/src

LOCAL_CPPFLAGS += -std=c++17
LOCAL_CPPFLAGS += -D VMA_STATIC_VULKAN_FUNCTIONS=0
LOCAL_CPP_FEATURES := rtti exceptions

LOCAL_SRC_FILES := vk_mem_alloc.cpp

include $(BUILD_STATIC_LIBRARY)
