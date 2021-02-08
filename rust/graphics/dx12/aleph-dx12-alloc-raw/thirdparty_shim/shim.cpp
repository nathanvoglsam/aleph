//
// Created by Nathan on 8/02/2021.
//

#include "../thirdparty/D3D12MemoryAllocator/src/D3D12MemAlloc.h"

extern "C" {

//
// ALLOCATION
//
void D3D12MA_Allocation_Release(D3D12MA::Allocation* self) {
    self->Release();
}
UINT64 D3D12MA_Allocation_GetOffset(const D3D12MA::Allocation* self) {
    return self->GetOffset();
}
UINT64 D3D12MA_Allocation_GetSize(const D3D12MA::Allocation* self) {
    return self->GetSize();
}
ID3D12Resource* D3D12MA_Allocation_GetResource(const D3D12MA::Allocation* self) {
    return self->GetResource();
}
ID3D12Heap* D3D12MA_Allocation_GetHeap(const D3D12MA::Allocation* self) {
    return self->GetHeap();
}
void D3D12MA_Allocation_SetName(D3D12MA::Allocation* self, LPCWSTR Name) {
    self->SetName(Name);
}
LPCWSTR D3D12MA_Allocation_GetName(const D3D12MA::Allocation* self) {
    return self->GetName();
}
BOOL D3D12MA_Allocation_WasZeroInitialized(const D3D12MA::Allocation* self) {
    return self->WasZeroInitialized();
}

//
// POOL
//
void D3D12MA_Pool_Release(D3D12MA::Pool* self) {
    self->Release();
}
D3D12MA::POOL_DESC D3D12MA_Pool_GetDesc(const D3D12MA::Pool* self) {
    return self->GetDesc();
}
HRESULT D3D12MA_Pool_SetMinBytes(D3D12MA::Pool* self, UINT64 minBytes) {
    return self->SetMinBytes(minBytes);
}
void D3D12MA_Pool_CalculateStats(D3D12MA::Pool* self, D3D12MA::StatInfo* pStats) {
    self->CalculateStats(pStats);
}
void D3D12MA_Pool_SetName(D3D12MA::Pool* self, LPCWSTR Name) {
    self->SetName(Name);
}
LPCWSTR D3D12MA_Pool_GetName(const D3D12MA::Pool* self) {
    return self->GetName();
}

//
// ALLOCATOR
//
HRESULT D3D12MA_Allocator_CreateAllocator(
    const D3D12MA::ALLOCATOR_DESC* p_desc,
    D3D12MA::Allocator** pp_allocator
) {
    return D3D12MA::CreateAllocator(p_desc, pp_allocator);
}
void D3D12MA_Allocator_Release(D3D12MA::Allocator* self) {
    self->Release();
}
const D3D12_FEATURE_DATA_D3D12_OPTIONS* D3D12MA_Allocator_GetD3D12Options(
    const D3D12MA::Allocator* self
) {
    return &self->GetD3D12Options();
}
HRESULT D3D12MA_Allocator_CreateResource(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_DESC* p_resource_desc,
    D3D12_RESOURCE_STATES initial_resource_state,
    const D3D12_CLEAR_VALUE* p_optimized_clear_value,
    D3D12MA::Allocation** pp_allocation,
    REFIID riid_resource,
    void** ppv_resource
) {
    return self->CreateResource(
        p_alloc_desc,
        p_resource_desc,
        initial_resource_state,
        p_optimized_clear_value,
        pp_allocation,
        riid_resource,
        ppv_resource
    );
}
HRESULT D3D12MA_Allocator_CreateResource1(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_DESC* p_resource_desc,
    D3D12_RESOURCE_STATES initial_resource_state,
    const D3D12_CLEAR_VALUE* p_optimized_clear_value,
    ID3D12ProtectedResourceSession* p_protected_session,
    D3D12MA::Allocation** pp_allocation,
    REFIID riid_resource,
    void** ppv_resource
) {
    return self->CreateResource1(
        p_alloc_desc,
        p_resource_desc,
        initial_resource_state,
        p_optimized_clear_value,
        p_protected_session,
        pp_allocation,
        riid_resource,
        ppv_resource
    );
}
HRESULT D3D12MA_Allocator_CreateResource2(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_DESC1* p_resource_desc,
    D3D12_RESOURCE_STATES initial_resource_state,
    const D3D12_CLEAR_VALUE* p_optimized_clear_value,
    ID3D12ProtectedResourceSession* p_protected_session,
    D3D12MA::Allocation** pp_allocation,
    REFIID riid_resource,
    void** ppv_resource
) {
    return self->CreateResource2(
        p_alloc_desc,
        p_resource_desc,
        initial_resource_state,
        p_optimized_clear_value,
        p_protected_session,
        pp_allocation,
        riid_resource,
        ppv_resource
    );
}
HRESULT D3D12MA_Allocator_AllocateMemory(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_ALLOCATION_INFO* p_alloc_info,
    D3D12MA::Allocation** pp_allocation
) {
    return self->AllocateMemory(
        p_alloc_desc,
        p_alloc_info,
        pp_allocation
    );
}
HRESULT D3D12MA_Allocator_AllocateMemory1(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_ALLOCATION_INFO* p_alloc_info,
    ID3D12ProtectedResourceSession* p_protected_session,
    D3D12MA::Allocation** pp_allocation
) {
    return self->AllocateMemory1(
        p_alloc_desc,
        p_alloc_info,
        p_protected_session,
        pp_allocation
    );
}
HRESULT D3D12MA_Allocator_CreateAliasingResource(
    D3D12MA::Allocator* self,
    D3D12MA::Allocation* p_allocation,
    UINT64 allocation_local_offset,
    const D3D12_RESOURCE_DESC* p_resource_desc,
    D3D12_RESOURCE_STATES initial_resource_state,
    const D3D12_CLEAR_VALUE* p_optimized_clear_value,
    REFIID riid_resource,
    void** ppv_resource
) {
    return self->CreateAliasingResource(
        p_allocation,
        allocation_local_offset,
        p_resource_desc,
        initial_resource_state,
        p_optimized_clear_value,
        riid_resource,
        ppv_resource
    );
}
HRESULT D3D12MA_Allocator_CreatePool(
    D3D12MA::Allocator* self,
    const D3D12MA::POOL_DESC* p_pool_desc,
    D3D12MA::Pool** pp_pool
) {
    return self->CreatePool(p_pool_desc, pp_pool);
}
HRESULT D3D12MA_Allocator_SetDefaultHeapMinBytes(
    D3D12MA::Allocator* self,
    D3D12_HEAP_TYPE heap_type,
    D3D12_HEAP_FLAGS heap_flags,
    UINT64 min_bytes
);
void D3D12MA_Allocator_SetCurrentFrameIndex(D3D12MA::Allocator* self, UINT frame_index) {
    self->SetCurrentFrameIndex(frame_index);
}
void D3D12MA_Allocator_CalculateStats(D3D12MA::Allocator* self, D3D12MA::Stats* p_stats) {
    self->CalculateStats(p_stats);
}
void D3D12MA_Allocator_GetBudget(
    D3D12MA::Allocator* self,
    D3D12MA::Budget* p_gpu_budget,
    D3D12MA::Budget* p_cpu_budget
) {
    self->GetBudget(p_gpu_budget, p_cpu_budget);
}
void D3D12MA_Allocator_BuildStatsString(
    const D3D12MA::Allocator* self,
    WCHAR** pp_stats_string,
    BOOL detailed_map
) {
    self->BuildStatsString(pp_stats_string, detailed_map);
}
void D3D12MA_Allocator_FreeStatsString(const D3D12MA::Allocator* self, WCHAR* p_stats_string) {
    self->FreeStatsString(p_stats_string);
}

//
// POOL
//
void D3D12MA_VirtualBlock_Release(D3D12MA::VirtualBlock* self) {
    self->Release();
}
BOOL D3D12MA_VirtualBlock_IsEmpty(const D3D12MA::VirtualBlock* self) {
    return self->IsEmpty();
}
void D3D12MA_VirtualBlock_GetAllocationInfo(
    const D3D12MA::VirtualBlock* self,
    UINT64 offset,
    D3D12MA::VIRTUAL_ALLOCATION_INFO* pInfo
) {
    self->GetAllocationInfo(offset, pInfo);
}
HRESULT D3D12MA_VirtualBlock_Allocate(
    D3D12MA::VirtualBlock* self,
    const D3D12MA::VIRTUAL_ALLOCATION_DESC* pDesc,
    UINT64* pOffset
) {
    return self->Allocate(pDesc, pOffset);
}
void D3D12MA_VirtualBlock_FreeAllocation(D3D12MA::VirtualBlock* self, UINT64 offset) {
    self->FreeAllocation(offset);
}
void D3D12MA_VirtualBlock_Clear(D3D12MA::VirtualBlock* self) {
    self->Clear();
}
void D3D12MA_VirtualBlock_SetAllocationUserData(
    D3D12MA::VirtualBlock* self,
    UINT64 offset,
    void* pUserData
) {
    self->SetAllocationUserData(offset, pUserData);
}
void D3D12MA_VirtualBlock_CalculateStats(
    const D3D12MA::VirtualBlock* self,
    D3D12MA::StatInfo* pInfo
) {
    self->CalculateStats(pInfo);
}
void D3D12MA_VirtualBlock_BuildStatsString(
    const D3D12MA::VirtualBlock* self,
    WCHAR** ppStatsString
) {
    self->BuildStatsString(ppStatsString);
}
void D3D12MA_VirtualBlock_FreeStatsString(const D3D12MA::VirtualBlock* self, WCHAR* pStatsString) {
    self->FreeStatsString(pStatsString);
}

} // extern "C"
