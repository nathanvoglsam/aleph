//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#![allow(non_snake_case)]
#![allow(unused)]
#![warn(unused_imports)]

use std::mem::MaybeUninit;
use windows::core::{Error, HRESULT};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

#[derive(Debug)]
pub struct FeatureSupport {
    device: ID3D12Device,

    options: D3D12_FEATURE_DATA_D3D12_OPTIONS,
    max_feature_level: D3D_FEATURE_LEVEL,
    gpu_va_support: D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT,
    shader_model: D3D12_FEATURE_DATA_SHADER_MODEL,
    options_1: D3D12_FEATURE_DATA_D3D12_OPTIONS1,
    // protected_resource_session_support: Vec<D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT>,
    root_signature: D3D_ROOT_SIGNATURE_VERSION,
    architecture_1: Vec<D3D12_FEATURE_DATA_ARCHITECTURE1>,
    options_2: D3D12_FEATURE_DATA_D3D12_OPTIONS2,
    shader_cache: D3D12_FEATURE_DATA_SHADER_CACHE,
    // command_queue_priority: D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY,
    options_3: D3D12_FEATURE_DATA_D3D12_OPTIONS3,
    existing_heaps: D3D12_FEATURE_DATA_EXISTING_HEAPS,
    options_4: D3D12_FEATURE_DATA_D3D12_OPTIONS4,
    serialization: Vec<D3D12_FEATURE_DATA_SERIALIZATION>, // Cat2 NodeIndex
    cross_node: D3D12_FEATURE_DATA_CROSS_NODE,
    options_5: D3D12_FEATURE_DATA_D3D12_OPTIONS5,
    displayable: D3D12_FEATURE_DATA_DISPLAYABLE,
    options_6: D3D12_FEATURE_DATA_D3D12_OPTIONS6,
    options_7: D3D12_FEATURE_DATA_D3D12_OPTIONS7,
    // protected_resource_session_type_count: Vec<D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT>, // Cat2 NodeIndex
    // protected_resource_session_types: Vec<ProtectedResourceSessionTypesLocal>, // Cat3
    options_8: D3D12_FEATURE_DATA_D3D12_OPTIONS8,
    options_9: D3D12_FEATURE_DATA_D3D12_OPTIONS9,
    options_10: D3D12_FEATURE_DATA_D3D12_OPTIONS10,
    options_11: D3D12_FEATURE_DATA_D3D12_OPTIONS11,
    options_12: D3D12_FEATURE_DATA_D3D12_OPTIONS12,
}

impl FeatureSupport {
    pub fn new(device: impl Into<ID3D12Device>) -> windows::core::Result<Self> {
        let device = device.into();
        unsafe {
            let options = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS)
                .unwrap_or_else(|_| D3D12_FEATURE_DATA_D3D12_OPTIONS {
                    DoublePrecisionFloatShaderOps: BOOL(0),
                    OutputMergerLogicOp: BOOL(0),
                    MinPrecisionSupport: D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE,
                    TiledResourcesTier: D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED,
                    ResourceBindingTier: Default::default(),
                    PSSpecifiedStencilRefSupported: BOOL(0),
                    TypedUAVLoadAdditionalFormats: BOOL(0),
                    ROVsSupported: BOOL(0),
                    ConservativeRasterizationTier:
                        D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED,
                    MaxGPUVirtualAddressBitsPerResource: 0,
                    StandardSwizzle64KBSupported: BOOL(0),
                    CrossNodeSharingTier: D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED,
                    CrossAdapterRowMajorTextureSupported: BOOL(0),
                    VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation:
                        BOOL(0),
                    ResourceHeapTier: Default::default(),
                });

            let gpu_va_support =
                load_options_or_default(&device, D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT)
                    .unwrap_or(D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
                        MaxGPUVirtualAddressBitsPerProcess: 0,
                        MaxGPUVirtualAddressBitsPerResource: 0,
                    });

            let options_1 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS1)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
                    WaveOps: BOOL(0),
                    WaveLaneCountMin: 0,
                    WaveLaneCountMax: 0,
                    TotalLaneCount: 0,
                    ExpandedComputeResourceStates: BOOL(0),
                    Int64ShaderOps: BOOL(0),
                });

            let options_2 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS2)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
                    DepthBoundsTestSupported: BOOL(0),
                    ProgrammableSamplePositionsTier:
                        D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED,
                });

            let shader_cache = load_options_or_default(&device, D3D12_FEATURE_SHADER_CACHE)
                .unwrap_or(D3D12_FEATURE_DATA_SHADER_CACHE {
                    SupportFlags: D3D12_SHADER_CACHE_SUPPORT_NONE,
                });

            let options_3 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS3)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
                    CopyQueueTimestampQueriesSupported: BOOL(0),
                    CastingFullyTypedFormatSupported: BOOL(0),
                    WriteBufferImmediateSupportFlags: D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE,
                    ViewInstancingTier: D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED,
                    BarycentricsSupported: BOOL(0),
                });

            let existing_heaps = load_options_or_default(&device, D3D12_FEATURE_EXISTING_HEAPS)
                .unwrap_or(D3D12_FEATURE_DATA_EXISTING_HEAPS { Supported: BOOL(0) });

            let options_4 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS4)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
                    MSAA64KBAlignedTextureSupported: BOOL(0),
                    SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0,
                    Native16BitShaderOpsSupported: BOOL(0),
                });

            let cross_node = load_options_or_default(&device, D3D12_FEATURE_CROSS_NODE).unwrap_or(
                D3D12_FEATURE_DATA_CROSS_NODE {
                    SharingTier: D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED,
                    AtomicShaderInstructions: BOOL(0),
                },
            );

            let options_5 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS5)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
                    SRVOnlyTiledResourceTier3: BOOL(0),
                    RenderPassesTier: D3D12_RENDER_PASS_TIER_0,
                    RaytracingTier: D3D12_RAYTRACING_TIER_NOT_SUPPORTED,
                });

            let displayable = load_options_or_default(&device, D3D12_FEATURE_DISPLAYABLE)
                .unwrap_or(D3D12_FEATURE_DATA_DISPLAYABLE {
                    DisplayableTexture: BOOL(0),
                    SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0,
                });

            let options_6 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS6)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
                    AdditionalShadingRatesSupported: BOOL(0),
                    PerPrimitiveShadingRateSupportedWithViewportIndexing: BOOL(0),
                    VariableShadingRateTier: D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED,
                    ShadingRateImageTileSize: 0,
                    BackgroundProcessingSupported: BOOL(0),
                });

            let options_7 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS7)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
                    MeshShaderTier: D3D12_MESH_SHADER_TIER_NOT_SUPPORTED,
                    SamplerFeedbackTier: D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED,
                });

            let options_8 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS8)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
                    UnalignedBlockTexturesSupported: BOOL(0),
                });

            let options_9 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS9)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
                    MeshShaderPipelineStatsSupported: BOOL(0),
                    MeshShaderSupportsFullRangeRenderTargetArrayIndex: BOOL(0),
                    AtomicInt64OnTypedResourceSupported: BOOL(0),
                    AtomicInt64OnGroupSharedSupported: BOOL(0),
                    DerivativesInMeshAndAmplificationShadersSupported: BOOL(0),
                    WaveMMATier: D3D12_WAVE_MMA_TIER_NOT_SUPPORTED,
                });

            let options_10 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS10)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
                    VariableRateShadingSumCombinerSupported: BOOL(0),
                    MeshShaderPerPrimitiveShadingRateSupported: BOOL(0),
                });

            let options_11 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS11)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
                    AtomicInt64OnDescriptorHeapResourceSupported: BOOL(0),
                });

            let options_12 = load_options_or_default(&device, D3D12_FEATURE_D3D12_OPTIONS12)
                .unwrap_or(D3D12_FEATURE_DATA_D3D12_OPTIONS12 {
                    MSPrimitivesPipelineStatisticIncludesCulledPrimitives: D3D12_TRI_STATE::UNKNOWN,
                    EnhancedBarriersSupported: BOOL(0),
                    RelaxedFormatCastingSupported: BOOL(0),
                });

            let node_count = device.GetNodeCount();
            let mut serialization = Vec::with_capacity(node_count as _);
            let mut architecture_1 = Vec::with_capacity(node_count as _);
            for i in 0..node_count {
                let mut node_architecture_1 = D3D12_FEATURE_DATA_ARCHITECTURE1 {
                    NodeIndex: i,
                    ..Default::default()
                };
                let result: Result<(), _> = device.CheckFeatureSupport(
                    D3D12_FEATURE_ARCHITECTURE1,
                    &mut node_architecture_1 as *mut D3D12_FEATURE_DATA_ARCHITECTURE1 as *mut _,
                    std::mem::size_of_val(&node_architecture_1) as u32,
                );

                if result.is_err() {
                    let mut node_architecture = D3D12_FEATURE_DATA_ARCHITECTURE {
                        NodeIndex: i,
                        ..Default::default()
                    };
                    let result: Result<(), _> = device.CheckFeatureSupport(
                        D3D12_FEATURE_ARCHITECTURE,
                        &mut node_architecture as *mut D3D12_FEATURE_DATA_ARCHITECTURE as *mut _,
                        std::mem::size_of_val(&node_architecture) as u32,
                    );

                    if result.is_err() {
                        node_architecture.TileBasedRenderer = Default::default();
                        node_architecture.UMA = Default::default();
                        node_architecture.CacheCoherentUMA = Default::default();
                    }

                    node_architecture_1.TileBasedRenderer = node_architecture.TileBasedRenderer;
                    node_architecture_1.UMA = node_architecture.UMA;
                    node_architecture_1.CacheCoherentUMA = node_architecture.CacheCoherentUMA;
                    node_architecture_1.IsolatedMMU = Default::default();
                }
                architecture_1.push(node_architecture_1);

                let mut node_serialization = D3D12_FEATURE_DATA_SERIALIZATION {
                    NodeIndex: i,
                    ..Default::default()
                };
                let result: Result<(), _> = device.CheckFeatureSupport(
                    D3D12_FEATURE_SERIALIZATION,
                    &mut node_serialization as *mut D3D12_FEATURE_DATA_SERIALIZATION as *mut _,
                    std::mem::size_of_val(&node_serialization) as u32,
                );

                if result.is_err() {
                    node_serialization.HeapSerializationTier = D3D12_HEAP_SERIALIZATION_TIER_0;
                }
                serialization.push(node_serialization);
            }

            let shader_model = Self::query_highest_shader_module(&device)?;
            let root_signature = Self::query_highest_root_signature(&device)?;
            let max_feature_level = Self::query_highest_feature_level(&device)?;

            Ok(Self {
                device,
                options,
                max_feature_level,
                gpu_va_support,
                shader_model,
                options_1,
                root_signature,
                architecture_1,
                options_2,
                shader_cache,
                options_3,
                existing_heaps,
                options_4,
                serialization,
                cross_node,
                options_5,
                displayable,
                options_6,
                options_7,
                options_8,
                options_9,
                options_10,
                options_11,
                options_12,
            })
        }
    }

    unsafe fn query_highest_shader_module(
        device: &ID3D12Device,
    ) -> windows::core::Result<D3D12_FEATURE_DATA_SHADER_MODEL> {
        let models = [
            D3D_SHADER_MODEL_6_7,
            D3D_SHADER_MODEL_6_6,
            D3D_SHADER_MODEL_6_5,
            D3D_SHADER_MODEL_6_4,
            D3D_SHADER_MODEL_6_3,
            D3D_SHADER_MODEL_6_2,
            D3D_SHADER_MODEL_6_1,
            D3D_SHADER_MODEL_6_0,
            D3D_SHADER_MODEL_5_1,
        ];

        for model in models {
            let mut data = D3D12_FEATURE_DATA_SHADER_MODEL {
                HighestShaderModel: model,
            };
            let result: Result<(), _> = device.CheckFeatureSupport(
                D3D12_FEATURE_SHADER_MODEL,
                &mut data as *mut D3D12_FEATURE_DATA_SHADER_MODEL as *mut _,
                std::mem::size_of_val(&data) as u32,
            );

            match result.map_err(HRESULT::from) {
                Ok(_) => {
                    // A successful return code means we support this shader model
                    return Ok(data);
                }
                Err(e) => {
                    // Invalid arg is an acceptable return code, meaning that the requested shader
                    // model is not supported. Any other error code is an error that should be
                    // returned
                    if e != E_INVALIDARG {
                        return Err(Error::from(e));
                    }
                }
            }
        }

        Ok(Default::default())
    }

    unsafe fn query_highest_root_signature(
        device: &ID3D12Device,
    ) -> windows::core::Result<D3D_ROOT_SIGNATURE_VERSION> {
        let signature_version = [
            D3D_ROOT_SIGNATURE_VERSION_1_1,
            D3D_ROOT_SIGNATURE_VERSION_1_0,
            D3D_ROOT_SIGNATURE_VERSION_1,
        ];

        for version in signature_version {
            let mut data = D3D12_FEATURE_DATA_ROOT_SIGNATURE {
                HighestVersion: version,
            };

            let result: Result<(), _> = device.CheckFeatureSupport(
                D3D12_FEATURE_ROOT_SIGNATURE,
                &mut data as *mut D3D12_FEATURE_DATA_ROOT_SIGNATURE as *mut _,
                std::mem::size_of_val(&data) as u32,
            );

            match result.map_err(HRESULT::from) {
                Ok(_) => {
                    // A successful return code means we support this shader model
                    return Ok(data.HighestVersion);
                }
                Err(e) => {
                    // Invalid arg is an acceptable return code, meaning that the requested shader
                    // model is not supported. Any other error code is an error that should be
                    // returned
                    if e != E_INVALIDARG {
                        return Err(Error::from(e));
                    }
                }
            }
        }

        Ok(Default::default())
    }

    unsafe fn query_highest_feature_level(
        device: &ID3D12Device,
    ) -> windows::core::Result<D3D_FEATURE_LEVEL> {
        let feature_levels = [
            D3D_FEATURE_LEVEL_12_2,
            D3D_FEATURE_LEVEL_12_1,
            D3D_FEATURE_LEVEL_12_0,
            D3D_FEATURE_LEVEL_11_1,
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1,
            D3D_FEATURE_LEVEL_1_0_CORE,
        ];

        let mut levels = D3D12_FEATURE_DATA_FEATURE_LEVELS {
            NumFeatureLevels: feature_levels.len() as _,
            pFeatureLevelsRequested: feature_levels.as_ptr(),
            MaxSupportedFeatureLevel: Default::default(),
        };

        let result: Result<(), _> = device.CheckFeatureSupport(
            D3D12_FEATURE_FEATURE_LEVELS,
            &mut levels as *mut D3D12_FEATURE_DATA_FEATURE_LEVELS as *mut _,
            std::mem::size_of_val(&levels) as u32,
        );

        match result.map_err(HRESULT::from) {
            Ok(_) => Ok(levels.MaxSupportedFeatureLevel),
            Err(e) => {
                if e == DXGI_ERROR_UNSUPPORTED {
                    Ok(D3D_FEATURE_LEVEL::default())
                } else {
                    Err(Error::from(e))
                }
            }
        }
    }
}

macro_rules! feature_support_get {
    ($return_type:ty, $source:ident, $name:ident) => {
        #[inline]
        pub fn $name(&self) -> $return_type {
            self.$source.$name.into()
        }
    };
}

macro_rules! feature_support_get_name {
    ($return_type:ty, $source:ident, $name:ident, $fn_name:ident) => {
        #[inline]
        pub fn $fn_name(&self) -> $return_type {
            self.$source.$name.into()
        }
    };
}

macro_rules! feature_support_get_node_indexed {
    ($return_type:ty, $source:ident, $name:ident) => {
        #[inline]
        pub fn $name(&self, node: usize) -> $return_type {
            self.$source[node].$name.into()
        }
    };
}

macro_rules! feature_support_get_node_indexed_name {
    ($return_type:ty, $source:ident, $name:ident, $fn_name:ident) => {
        #[inline]
        pub fn $fn_name(&self, node: usize) -> $return_type {
            self.$source[node].$name.into()
        }
    };
}

impl FeatureSupport {
    // 0: D3D12_OPTIONS
    feature_support_get!(bool, options, DoublePrecisionFloatShaderOps);
    feature_support_get!(bool, options, OutputMergerLogicOp);
    feature_support_get!(
        D3D12_SHADER_MIN_PRECISION_SUPPORT,
        options,
        MinPrecisionSupport
    );
    feature_support_get!(D3D12_TILED_RESOURCES_TIER, options, TiledResourcesTier);
    feature_support_get!(D3D12_RESOURCE_BINDING_TIER, options, ResourceBindingTier);
    feature_support_get!(bool, options, PSSpecifiedStencilRefSupported);
    feature_support_get!(bool, options, TypedUAVLoadAdditionalFormats);
    feature_support_get!(bool, options, ROVsSupported);
    feature_support_get!(
        D3D12_CONSERVATIVE_RASTERIZATION_TIER,
        options,
        ConservativeRasterizationTier
    );
    feature_support_get!(bool, options, StandardSwizzle64KBSupported);
    feature_support_get!(bool, options, CrossAdapterRowMajorTextureSupported);
    feature_support_get!(
        bool,
        options,
        VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation
    );
    feature_support_get!(D3D12_RESOURCE_HEAP_TIER, options, ResourceHeapTier);

    #[inline]
    pub fn CrossNodeSharingTier(&self) -> D3D12_CROSS_NODE_SHARING_TIER {
        if self.cross_node.SharingTier.0 > 0 {
            self.cross_node.SharingTier
        } else {
            self.options.CrossNodeSharingTier
        }
    }

    #[inline]
    pub fn MaxGPUVirtualAddressBitsPerResource(&self) -> u32 {
        if self.options.MaxGPUVirtualAddressBitsPerResource > 0 {
            self.options.MaxGPUVirtualAddressBitsPerResource
        } else {
            self.gpu_va_support.MaxGPUVirtualAddressBitsPerResource
        }
    }

    // 2: Feature Levels
    #[inline]
    pub fn MaxSupportedFeatureLevel(&self) -> D3D_FEATURE_LEVEL {
        self.max_feature_level
    }

    // 3: Feature Format Support
    #[inline]
    pub fn FormatSupport(
        &self,
        format: DXGI_FORMAT,
    ) -> Option<(D3D12_FORMAT_SUPPORT1, D3D12_FORMAT_SUPPORT2)> {
        let mut support = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: format,
            Support1: Default::default(),
            Support2: Default::default(),
        };

        unsafe {
            self.device
                .CheckFeatureSupport(
                    D3D12_FEATURE_FORMAT_SUPPORT,
                    &mut support as *mut D3D12_FEATURE_DATA_FORMAT_SUPPORT as *mut _,
                    std::mem::size_of_val(&support) as u32,
                )
                .ok()
                .map(|_| (support.Support1, support.Support2))
        }
    }

    // 4: Multisample Quality Levels
    #[inline]
    pub fn MultisampleQualityLevels(
        &self,
        format: DXGI_FORMAT,
        sample_count: u32,
        flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    ) -> Option<u32> {
        let mut levels = D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
            Format: format,
            SampleCount: sample_count,
            Flags: flags,
            NumQualityLevels: 0,
        };

        unsafe {
            self.device
                .CheckFeatureSupport(
                    D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS,
                    &mut levels as *mut D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS as *mut _,
                    std::mem::size_of_val(&levels) as u32,
                )
                .ok()
                .map(|_| (levels.NumQualityLevels))
        }
    }

    // 5: Format Info
    #[inline]
    pub fn FormatInfo(&self, format: DXGI_FORMAT) -> Option<u8> {
        let mut format_info = D3D12_FEATURE_DATA_FORMAT_INFO {
            Format: format,
            PlaneCount: 0,
        };

        unsafe {
            self.device
                .CheckFeatureSupport(
                    D3D12_FEATURE_FORMAT_INFO,
                    &mut format_info as *mut D3D12_FEATURE_DATA_FORMAT_INFO as *mut _,
                    std::mem::size_of_val(&format_info) as u32,
                )
                .ok()
                .map(|_| (format_info.PlaneCount))
        }
    }

    // 6: GPU Virtual Address Support
    // MaxGPUVirtualAddressBitsPerResource handled in D3D12Options
    feature_support_get!(u32, gpu_va_support, MaxGPUVirtualAddressBitsPerProcess);

    // 7: Shader Model
    #[inline]
    pub fn HighestShaderModel(&self) -> D3D_SHADER_MODEL {
        self.shader_model.HighestShaderModel
    }

    // 8: D3D12 Options1
    feature_support_get!(bool, options_1, WaveOps);
    feature_support_get!(u32, options_1, WaveLaneCountMin);
    feature_support_get!(u32, options_1, WaveLaneCountMax);
    feature_support_get!(u32, options_1, TotalLaneCount);
    feature_support_get!(bool, options_1, ExpandedComputeResourceStates);
    feature_support_get!(bool, options_1, Int64ShaderOps);

    // 12: Root Signature
    #[inline]
    pub fn HighestRootSignatureVersion(&self) -> D3D_ROOT_SIGNATURE_VERSION {
        self.root_signature
    }

    // 16: Architecture1
    // Same data fields can be queried from m_dArchitecture
    feature_support_get_node_indexed!(bool, architecture_1, TileBasedRenderer);
    feature_support_get_node_indexed!(bool, architecture_1, UMA);
    feature_support_get_node_indexed!(bool, architecture_1, CacheCoherentUMA);
    feature_support_get_node_indexed!(bool, architecture_1, IsolatedMMU);

    // 18: D3D12 Options2
    feature_support_get!(bool, options_2, DepthBoundsTestSupported);
    feature_support_get!(
        D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER,
        options_2,
        ProgrammableSamplePositionsTier
    );

    // 19: Shader Cache
    feature_support_get_name!(
        D3D12_SHADER_CACHE_SUPPORT_FLAGS,
        shader_cache,
        SupportFlags,
        ShaderCacheSupportFlags
    );

    // 20: Command Queue Priority
    #[inline]
    pub fn CommandQueuePrioritySupported(
        &self,
        command_list_type: D3D12_COMMAND_LIST_TYPE,
        priority: u32,
    ) -> bool {
        let mut priority = D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
            CommandListType: command_list_type,
            Priority: priority,
            PriorityForTypeIsSupported: Default::default(),
        };

        unsafe {
            let success: bool = self
                .device
                .CheckFeatureSupport(
                    D3D12_FEATURE_COMMAND_QUEUE_PRIORITY,
                    &mut priority as *mut D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY as *mut _,
                    std::mem::size_of_val(&priority) as u32,
                )
                .is_ok();
            if !success {
                false
            } else {
                priority.PriorityForTypeIsSupported.as_bool()
            }
        }
    }

    // 21: D3D12 Options3
    feature_support_get!(bool, options_3, CopyQueueTimestampQueriesSupported);
    feature_support_get!(bool, options_3, CastingFullyTypedFormatSupported);
    feature_support_get!(
        D3D12_COMMAND_LIST_SUPPORT_FLAGS,
        options_3,
        WriteBufferImmediateSupportFlags
    );
    feature_support_get!(D3D12_VIEW_INSTANCING_TIER, options_3, ViewInstancingTier);
    feature_support_get!(bool, options_3, BarycentricsSupported);

    // 22: Existing Heaps
    feature_support_get_name!(bool, existing_heaps, Supported, ExistingHeapsSupported);

    // 23: D3D12 Options4
    feature_support_get!(bool, options_4, MSAA64KBAlignedTextureSupported);
    feature_support_get!(
        D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
        options_4,
        SharedResourceCompatibilityTier
    );
    feature_support_get!(bool, options_4, Native16BitShaderOpsSupported);

    // 24: Serialization
    feature_support_get_node_indexed!(
        D3D12_HEAP_SERIALIZATION_TIER,
        serialization,
        HeapSerializationTier
    );

    // 25: Cross Node
    // CrossNodeSharingTier handled in D3D12Options
    feature_support_get_name!(
        bool,
        cross_node,
        AtomicShaderInstructions,
        CrossNodeAtomicShaderInstructions
    );

    // 27: D3D12 Options5
    feature_support_get!(bool, options_5, SRVOnlyTiledResourceTier3);
    feature_support_get!(D3D12_RENDER_PASS_TIER, options_5, RenderPassesTier);
    feature_support_get!(D3D12_RAYTRACING_TIER, options_5, RaytracingTier);

    // 28: Displayable
    feature_support_get!(bool, displayable, DisplayableTexture);
    // SharedResourceCompatibilityTier handled in D3D12Options4

    // 30: D3D12 Options6
    feature_support_get!(bool, options_6, AdditionalShadingRatesSupported);
    feature_support_get!(
        bool,
        options_6,
        PerPrimitiveShadingRateSupportedWithViewportIndexing
    );
    feature_support_get!(
        D3D12_VARIABLE_SHADING_RATE_TIER,
        options_6,
        VariableShadingRateTier
    );
    feature_support_get!(u32, options_6, ShadingRateImageTileSize);
    feature_support_get!(bool, options_6, BackgroundProcessingSupported);

    // 32: D3D12 Options7
    feature_support_get!(D3D12_MESH_SHADER_TIER, options_7, MeshShaderTier);
    feature_support_get!(D3D12_SAMPLER_FEEDBACK_TIER, options_7, SamplerFeedbackTier);

    // 36: Options8
    feature_support_get!(bool, options_8, UnalignedBlockTexturesSupported);

    // 37: Options9
    feature_support_get!(bool, options_9, MeshShaderPipelineStatsSupported);
    feature_support_get!(
        bool,
        options_9,
        MeshShaderSupportsFullRangeRenderTargetArrayIndex
    );
    feature_support_get!(bool, options_9, AtomicInt64OnTypedResourceSupported);
    feature_support_get!(bool, options_9, AtomicInt64OnGroupSharedSupported);
    feature_support_get!(
        bool,
        options_9,
        DerivativesInMeshAndAmplificationShadersSupported
    );
    feature_support_get!(D3D12_WAVE_MMA_TIER, options_9, WaveMMATier);

    // 39: Options10
    feature_support_get!(bool, options_10, VariableRateShadingSumCombinerSupported);
    feature_support_get!(bool, options_10, MeshShaderPerPrimitiveShadingRateSupported);

    // 40: Options11
    feature_support_get!(
        bool,
        options_11,
        AtomicInt64OnDescriptorHeapResourceSupported
    );

    // 41: Options12
    feature_support_get!(
        D3D12_TRI_STATE,
        options_12,
        MSPrimitivesPipelineStatisticIncludesCulledPrimitives
    );
    feature_support_get!(bool, options_12, EnhancedBarriersSupported);
    feature_support_get!(bool, options_12, RelaxedFormatCastingSupported);

    // // 42: Options13
    // feature_support_get!(bool, options_13, UnrestrictedBufferTextureCopyPitchSupported);
    // feature_support_get!(bool, options_13, UnrestrictedVertexElementAlignmentSupported);
    // feature_support_get!(bool, options_13, InvertedViewportHeightFlipsYSupported);
    // feature_support_get!(bool, options_13, InvertedViewportDepthFlipsZSupported);
    // feature_support_get!(bool, options_13, TextureCopyBetweenDimensionsSupported);
    // feature_support_get!(bool, options_13, AlphaBlendFactorSupported);
    //
    // // 43: Options14
    // feature_support_get!(bool, options_14, AdvancedTextureOpsSupported);
    // feature_support_get!(bool, options_14, WriteableMSAATexturesSupported);
    // feature_support_get!(bool, options_14, IndependentFrontAndBackStencilRefMaskSupported);
    //
    // // 44: Options15
    // feature_support_get!(bool, options_15, TriangleFanSupported);
    // feature_support_get!(bool, options_15, DynamicIndexBufferStripCutSupported);
}

unsafe fn load_options_or_default<T>(
    device: &ID3D12Device,
    feature: D3D12_FEATURE,
) -> windows::core::Result<T> {
    let mut data = MaybeUninit::<T>::uninit();
    let result: Result<(), _> = device.CheckFeatureSupport(
        feature,
        data.as_mut_ptr() as *mut _,
        std::mem::size_of_val(&data) as u32,
    );
    result.map(|_| data.assume_init())
}
