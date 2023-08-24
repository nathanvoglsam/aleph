//
// Created by Nathan on 8/02/2021.
//

#include "D3D12MemAlloc.h"

extern "C" {

// =====================================================================================================================
// ALLOCATION
// =====================================================================================================================

// Allocation::Release
void D3D12MA_Allocation_Release(D3D12MA::Allocation* self) {
    self->Release();
}

// Allocation::GetOffset
UINT64 D3D12MA_Allocation_GetOffset(const D3D12MA::Allocation* self) {
    return self->GetOffset();
}

// Allocation::GetAlignment
UINT64 D3D12MA_Allocation_GetAlignment(const D3D12MA::Allocation* self) {
    return self->GetAlignment();
}

// Allocation::GetSize
UINT64 D3D12MA_Allocation_GetSize(const D3D12MA::Allocation* self) {
    return self->GetSize();
}

// Allocation::GetResource
ID3D12Resource* D3D12MA_Allocation_GetResource(const D3D12MA::Allocation* self) {
    auto resource = self->GetResource();
    resource->AddRef();
    return resource;
}

// Allocation::SetResource
void D3D12MA_Allocation_SetResource(D3D12MA::Allocation* self, ID3D12Resource *pResource) {
    self->SetResource(pResource);
}

// Allocation::GetHeap
ID3D12Heap* D3D12MA_Allocation_GetHeap(const D3D12MA::Allocation* self) {
    auto heap = self->GetHeap();
    heap->AddRef();
    return heap;
}

// Allocation::SetPrivateData
void D3D12MA_Allocation_SetPrivateData(D3D12MA::Allocation* self, void* pPrivateData) {
    self->SetPrivateData(pPrivateData);
}

// Allocation::GetPrivateData
void* D3D12MA_Allocation_GetPrivateData(const D3D12MA::Allocation* self) {
    return self->GetPrivateData();
}

// Allocation::SetName
void D3D12MA_Allocation_SetName(D3D12MA::Allocation* self, LPCWSTR Name) {
    self->SetName(Name);
}

// Allocation::GetName
LPCWSTR D3D12MA_Allocation_GetName(const D3D12MA::Allocation* self) {
    return self->GetName();
}

// Allocation::WasZeroInitialized
BOOL D3D12MA_Allocation_WasZeroInitialized(const D3D12MA::Allocation* self) {
    return self->WasZeroInitialized();
}

// =====================================================================================================================
// POOL
// =====================================================================================================================

// Pool::Release
void D3D12MA_Pool_Release(D3D12MA::Pool* self) {
    self->Release();
}

// Pool::GetDesc
D3D12MA::POOL_DESC D3D12MA_Pool_GetDesc(const D3D12MA::Pool* self) {
    return self->GetDesc();
}

// Pool::GetStatistics
void D3D12MA_Pool_GetStatistics(D3D12MA::Pool* self, D3D12MA::Statistics* pStats) {
    return self->GetStatistics(pStats);
}

// Pool::CalculateStatistics
void D3D12MA_Pool_CalculateStatistics(D3D12MA::Pool* self, D3D12MA::DetailedStatistics* pStats) {
    self->CalculateStatistics(pStats);
}

// Pool::SetName
void D3D12MA_Pool_SetName(D3D12MA::Pool* self, LPCWSTR Name) {
    self->SetName(Name);
}

// Pool::GetName
LPCWSTR D3D12MA_Pool_GetName(const D3D12MA::Pool* self) {
    return self->GetName();
}

// Pool::BeginDefragmentation
void D3D12MA_Pool_BeginDefragmentation(
        D3D12MA::Pool* self,
        const D3D12MA::DEFRAGMENTATION_DESC *pDesc,
        D3D12MA::DefragmentationContext **ppContext
) {
    self->BeginDefragmentation(pDesc, ppContext);
}

// =====================================================================================================================
// ALLOCATOR
// =====================================================================================================================

// CreateAllocator
HRESULT D3D12MA_Allocator_CreateAllocator(
    const D3D12MA::ALLOCATOR_DESC* p_desc,
    D3D12MA::Allocator** pp_allocator
) {
    return D3D12MA::CreateAllocator(p_desc, pp_allocator);
}

// Allocator::Release
void D3D12MA_Allocator_Release(D3D12MA::Allocator* self) {
    self->Release();
}

// Allocator::GetD3D12Options
const D3D12_FEATURE_DATA_D3D12_OPTIONS* D3D12MA_Allocator_GetD3D12Options(
    const D3D12MA::Allocator* self
) {
    return &self->GetD3D12Options();
}

// Allocator::IsUMA
BOOL D3D12MA_Allocator_IsUMA(
    const D3D12MA::Allocator* self
) {
    return self->IsUMA();
}

// Allocator::IsCacheCoherentUMA
BOOL D3D12MA_Allocator_IsCacheCoherentUMA(
    const D3D12MA::Allocator* self
) {
    return self->IsCacheCoherentUMA();
}

// Allocator::GetMemoryCapacity
UINT64 D3D12MA_Allocator_GetMemoryCapacity(
    const D3D12MA::Allocator* self,
    UINT memorySegmentGroup
) {
    return self->GetMemoryCapacity(memorySegmentGroup);
}

// Allocator::CreateResource
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

#ifdef __ID3D12Device8_INTERFACE_DEFINED__
// Allocator::CreateResource2
HRESULT D3D12MA_Allocator_CreateResource2(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* p_alloc_desc,
    const D3D12_RESOURCE_DESC1* p_resource_desc,
    D3D12_RESOURCE_STATES initial_resource_state,
    const D3D12_CLEAR_VALUE* p_optimized_clear_value,
    D3D12MA::Allocation** pp_allocation,
    REFIID riid_resource,
    void** ppv_resource
) {
    return self->CreateResource2(
        p_alloc_desc,
        p_resource_desc,
        initial_resource_state,
        p_optimized_clear_value,
        pp_allocation,
        riid_resource,
        ppv_resource
    );
}
#endif // #ifdef __ID3D12Device8_INTERFACE_DEFINED__

// Allocator::AllocateMemory
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

// Allocator::CreateAliasingResource
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

// Allocator::CreatePool
HRESULT D3D12MA_Allocator_CreatePool(
    D3D12MA::Allocator* self,
    const D3D12MA::POOL_DESC* p_pool_desc,
    D3D12MA::Pool** pp_pool
) {
    return self->CreatePool(p_pool_desc, pp_pool);
}

// Allocator::SetCurrentFrameIndex
void D3D12MA_Allocator_SetCurrentFrameIndex(D3D12MA::Allocator* self, UINT frame_index) {
    self->SetCurrentFrameIndex(frame_index);
}

// Allocator::GetBudget
void D3D12MA_Allocator_GetBudget(
    D3D12MA::Allocator* self,
    D3D12MA::Budget* p_gpu_budget,
    D3D12MA::Budget* p_cpu_budget
) {
    self->GetBudget(p_gpu_budget, p_cpu_budget);
}

// Allocator::CalculateStatistics
void D3D12MA_Allocator_CalculateStatistics(D3D12MA::Allocator* self, D3D12MA::TotalStatistics* pStats) {
    self->CalculateStatistics(pStats);
}

// Allocator::BuildStatsString
void D3D12MA_Allocator_BuildStatsString(
    const D3D12MA::Allocator* self,
    WCHAR** pp_stats_string,
    BOOL detailed_map
) {
    self->BuildStatsString(pp_stats_string, detailed_map);
}

// Allocator::FreeStatsString
void D3D12MA_Allocator_FreeStatsString(const D3D12MA::Allocator* self, WCHAR* p_stats_string) {
    self->FreeStatsString(p_stats_string);
}

// Allocator::BeginDefragmentation
void D3D12MA_Allocator_BeginDefragmentation(
    D3D12MA::Allocator* self,
    const D3D12MA::DEFRAGMENTATION_DESC *pDesc,
    D3D12MA::DefragmentationContext **ppContext
) {
    self->BeginDefragmentation(pDesc, ppContext);
}

// =====================================================================================================================
// VirtualBlock
// =====================================================================================================================

// CreateVirtualBlock
HRESULT D3D12MA_VirtualBlock_CreateVirtualBlock(
    const D3D12MA::VIRTUAL_BLOCK_DESC* pDesc,
    D3D12MA::VirtualBlock** ppVirtualBlock
) {
    return D3D12MA::CreateVirtualBlock(pDesc, ppVirtualBlock);
}

// VirtualBlock::Release
void D3D12MA_VirtualBlock_Release(D3D12MA::VirtualBlock* self) {
    self->Release();
}

// VirtualBlock::IsEmpty
BOOL D3D12MA_VirtualBlock_IsEmpty(const D3D12MA::VirtualBlock* self) {
    return self->IsEmpty();
}

// VirtualBlock::GetAllocationInfo
void D3D12MA_VirtualBlock_GetAllocationInfo(
    const D3D12MA::VirtualBlock* self,
    D3D12MA::VirtualAllocation allocation,
    D3D12MA::VIRTUAL_ALLOCATION_INFO* pInfo
) {
    self->GetAllocationInfo(allocation, pInfo);
}

// VirtualBlock::Allocate
HRESULT D3D12MA_VirtualBlock_Allocate(
    D3D12MA::VirtualBlock* self,
    const D3D12MA::VIRTUAL_ALLOCATION_DESC* pDesc,
    D3D12MA::VirtualAllocation* pAllocation,
    UINT64* pOffset
) {
    return self->Allocate(pDesc, pAllocation, pOffset);
}

// VirtualBlock::FreeAllocation
void D3D12MA_VirtualBlock_FreeAllocation(D3D12MA::VirtualBlock* self, D3D12MA::VirtualAllocation allocation) {
    self->FreeAllocation(allocation);
}

// VirtualBlock::Clear
void D3D12MA_VirtualBlock_Clear(D3D12MA::VirtualBlock* self) {
    self->Clear();
}

// VirtualBlock::SetAllocationPrivateData
void D3D12MA_VirtualBlock_SetAllocationPrivateData(
    D3D12MA::VirtualBlock* self,
    D3D12MA::VirtualAllocation allocation,
    void* pUserData
) {
    self->SetAllocationPrivateData(allocation, pUserData);
}

// VirtualBlock::GetStatistics
void D3D12MA_VirtualBlock_GetStatistics(
        const D3D12MA::VirtualBlock* self,
        D3D12MA::Statistics* pStats
) {
    self->GetStatistics(pStats);
}

// VirtualBlock::CalculateStatistics
void D3D12MA_VirtualBlock_CalculateStatistics(
    const D3D12MA::VirtualBlock* self,
    D3D12MA::DetailedStatistics* pStats
) {
    self->CalculateStatistics(pStats);
}

// VirtualBlock::BuildStatsString
void D3D12MA_VirtualBlock_BuildStatsString(
    const D3D12MA::VirtualBlock* self,
    WCHAR** ppStatsString
) {
    self->BuildStatsString(ppStatsString);
}

// VirtualBlock::FreeStatsString
void D3D12MA_VirtualBlock_FreeStatsString(const D3D12MA::VirtualBlock* self, WCHAR* pStatsString) {
    self->FreeStatsString(pStatsString);
}

} // extern "C"
