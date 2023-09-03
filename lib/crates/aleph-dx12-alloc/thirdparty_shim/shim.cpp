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
    const D3D12MA::ALLOCATOR_DESC* pDesc,
    D3D12MA::Allocator** ppAllocator
) {
    return D3D12MA::CreateAllocator(pDesc, ppAllocator);
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
    const D3D12MA::ALLOCATION_DESC* pAllocDesc,
    const D3D12_RESOURCE_DESC* pResourceDesc,
    D3D12_RESOURCE_STATES InitialResourceState,
    const D3D12_CLEAR_VALUE* pOptimizedClearValue,
    D3D12MA::Allocation** ppAllocation,
    REFIID riidResource,
    void** ppvResource
) {
    return self->CreateResource(
        pAllocDesc,
        pResourceDesc,
        InitialResourceState,
        pOptimizedClearValue,
        ppAllocation,
        riidResource,
        ppvResource
    );
}

#ifdef __ID3D12Device8_INTERFACE_DEFINED__
// Allocator::CreateResource2
HRESULT D3D12MA_Allocator_CreateResource2(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* pAllocDesc,
    const D3D12_RESOURCE_DESC1* pResourceDesc,
    D3D12_RESOURCE_STATES InitialResourceState,
    const D3D12_CLEAR_VALUE* pOptimizedClearValue,
    D3D12MA::Allocation** ppAllocation,
    REFIID riidResource,
    void** ppvResource
) {
    return self->CreateResource2(
        pAllocDesc,
        pResourceDesc,
        InitialResourceState,
        pOptimizedClearValue,
        ppAllocation,
        riidResource,
        ppvResource
    );
}
#endif // #ifdef __ID3D12Device8_INTERFACE_DEFINED__

// Allocator::AllocateMemory
HRESULT D3D12MA_Allocator_AllocateMemory(
    D3D12MA::Allocator* self,
    const D3D12MA::ALLOCATION_DESC* pAllocDesc,
    const D3D12_RESOURCE_ALLOCATION_INFO* pAllocInfo,
    D3D12MA::Allocation** ppAllocation
) {
    return self->AllocateMemory(
        pAllocDesc,
        pAllocInfo,
        ppAllocation
    );
}

// Allocator::CreateAliasingResource
HRESULT D3D12MA_Allocator_CreateAliasingResource(
    D3D12MA::Allocator* self,
    D3D12MA::Allocation* pAllocation,
    UINT64 AllocationLocalOffset,
    const D3D12_RESOURCE_DESC* pResourceDesc,
    D3D12_RESOURCE_STATES InitialResourceState,
    const D3D12_CLEAR_VALUE* pOptimizedClearValue,
    REFIID riidResource,
    void** ppvResource
) {
    return self->CreateAliasingResource(
        pAllocation,
        AllocationLocalOffset,
        pResourceDesc,
        InitialResourceState,
        pOptimizedClearValue,
        riidResource,
        ppvResourcee
    );
}

// Allocator::CreatePool
HRESULT D3D12MA_Allocator_CreatePool(
    D3D12MA::Allocator* self,
    const D3D12MA::POOL_DESC* pPoolDesc,
    D3D12MA::Pool** ppPool
) {
    return self->CreatePool(pPoolDesc, ppPool);
}

// Allocator::SetCurrentFrameIndex
void D3D12MA_Allocator_SetCurrentFrameIndex(D3D12MA::Allocator* self, UINT frameIndex) {
    self->SetCurrentFrameIndex(frameIndex);
}

// Allocator::GetBudget
void D3D12MA_Allocator_GetBudget(
    D3D12MA::Allocator* self,
    D3D12MA::Budget* pLocalBudget,
    D3D12MA::Budget* pNonLocalBudget
) {
    self->GetBudget(pLocalBudget, pNonLocalBudget);
}

// Allocator::CalculateStatistics
void D3D12MA_Allocator_CalculateStatistics(D3D12MA::Allocator* self, D3D12MA::TotalStatistics* pStats) {
    self->CalculateStatistics(pStats);
}

// Allocator::BuildStatsString
void D3D12MA_Allocator_BuildStatsString(
    const D3D12MA::Allocator* self,
    WCHAR** ppStatsString,
    BOOL DetailedMap
) {
    self->BuildStatsString(ppStatsString, DetailedMap);
}

// Allocator::FreeStatsString
void D3D12MA_Allocator_FreeStatsString(const D3D12MA::Allocator* self, WCHAR* pStatsString) {
    self->FreeStatsString(pStatsString);
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

// =====================================================================================================================
// DefragmentationContext
// =====================================================================================================================

void D3D12MA_DefragmentationContext_Release(D3D12MA::DefragmentationContext* self) {
    self->Release();
}

HRESULT D3D12MA_DefragmentationContext_BeginPass(
    D3D12MA::DefragmentationContext* self,
    D3D12MA::DEFRAGMENTATION_PASS_MOVE_INFO* pPassInfo
) {
    return self->BeginPass(pPassInfo);
}

HRESULT D3D12MA_DefragmentationContext_EndPass(
    D3D12MA::DefragmentationContext* self,
    D3D12MA::DEFRAGMENTATION_PASS_MOVE_INFO* pPassInfo
) {
    return self->EndPass(pPassInfo);
}

void D3D12MA_DefragmentationContext_GetStats(
    D3D12MA::DefragmentationContext* self,
    D3D12MA::DEFRAGMENTATION_STATS* pStats
) {
    self->GetStats(pStats);
}

} // extern "C"
