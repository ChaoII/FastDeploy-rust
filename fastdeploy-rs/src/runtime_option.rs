use std::ffi::CString;
use std::mem;

use fastdeploy_bind::*;

use crate::enum_variables::{LitePowerMode, ModelFormat, RKNpu2CoreMask, RKNpu2CpuName};

pub struct RuntimeOption {
    pub(crate) ptr: *mut FD_C_RuntimeOptionWrapper,
}

impl RuntimeOption {
    pub fn new() -> Self {
        unsafe {
            return RuntimeOption { ptr: FD_C_CreateRuntimeOptionWrapper() };
        }
    }
    pub fn set_model_path(&mut self, model_path: &str, params_path: &str, format: &ModelFormat) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetModelPath(
                self.ptr,
                CString::new(model_path).unwrap().into_raw(),
                CString::new(params_path).unwrap().into_raw(),
                format.to_raw(),
            );
        }
    }


    pub fn set_model_buffer(&mut self, model_buffer: &[u8], params_buffer: &[u8], format: &ModelFormat) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetModelBuffer(
                self.ptr,
                CString::new(model_buffer).unwrap().into_raw(),
                CString::new(params_buffer).unwrap().into_raw(),
                format.to_raw(),
            );
        }
    }
    pub fn use_cpu(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseCpu(self.ptr);
        }
    }

    pub fn use_gpu(&mut self, device_id: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseGpu(self.ptr, device_id);
        }
    }

    pub fn use_rk_npu2(&mut self, rk_npu_cpu_name: &RKNpu2CpuName, rk_npu_core_mask: &RKNpu2CoreMask) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseRKNPU2(
                self.ptr,
                rk_npu_cpu_name.to_raw(),
                rk_npu_core_mask.to_raw(),
            );
        }
    }
    pub fn use_timvx(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseTimVX(self.ptr);
        }
    }

    pub fn use_ascend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseAscend(self.ptr);
        }
    }
    pub fn use_kunlunxin(&mut self, kunlunxin_id: i32, l3_workspace_size: i32,
                         locked: bool, autotune: bool, autotune_file: &str, precision: &str,
                         adaptive_seqlen: bool, enable_multi_stream: bool, gm_default_size: i64) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseKunlunXin(self.ptr,
                                                  kunlunxin_id,
                                                  l3_workspace_size,
                                                  locked as FD_C_Bool,
                                                  autotune as FD_C_Bool,
                                                  CString::new(autotune_file).unwrap().into_raw(),
                                                  CString::new(precision).unwrap().into_raw(),
                                                  adaptive_seqlen as FD_C_Bool,
                                                  enable_multi_stream as FD_C_Bool,
                                                  gm_default_size);
        }
    }
    pub fn use_sophgo(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseSophgo(self.ptr);
        }
    }

    pub fn set_external_stream(&mut self, external_stream: &mut [u8]) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetExternalStream(self.ptr, mem::transmute(external_stream.as_mut_ptr()));
        }
    }
    pub fn set_cpu_thread_num(&mut self, thread_num: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetCpuThreadNum(self.ptr, thread_num);
        }
    }

    pub fn set_ort_graph_opt_level(&mut self, level: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetOrtGraphOptLevel(self.ptr, level);
        }
    }

    pub fn use_paddle_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUsePaddleBackend(self.ptr);
        }
    }

    pub fn use_paddle_inference_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUsePaddleInferBackend(self.ptr);
        }
    }

    pub fn use_ort_backend(&self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseOrtBackend(self.ptr);
        }
    }

    pub fn use_sophgo_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseSophgoBackend(self.ptr);
        }
    }

    pub fn use_trt_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseTrtBackend(self.ptr);
        }
    }
    pub fn use_poros_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUsePorosBackend(self.ptr);
        }
    }

    pub fn use_openvino_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseOpenVINOBackend(self.ptr);
        }
    }

    pub fn use_lite_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseLiteBackend(self.ptr);
        }
    }

    pub fn use_paddle_lite_backend(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperUsePaddleLiteBackend(self.ptr);
        }
    }

    pub fn set_paddle_mkldnn(&mut self, pd_mkldnn: bool) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetPaddleMKLDNN(self.ptr, pd_mkldnn as FD_C_Bool);
        }
    }

    pub fn enable_paddle_2_trt(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnablePaddleToTrt(self.ptr);
        }
    }

    pub fn delete_paddle_backend_pass(&mut self, delete_pass_name: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperDeletePaddleBackendPass(self.ptr, CString::new(delete_pass_name).unwrap().into_raw());
        }
    }

    pub fn enable_paddle_log_info(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnablePaddleLogInfo(self.ptr);
        }
    }

    pub fn disable_paddle_log_info(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisablePaddleLogInfo(self.ptr);
        }
    }

    pub fn set_paddle_mkldnn_cache_size(&mut self, size: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetPaddleMKLDNNCacheSize(self.ptr, size);
        }
    }
    pub fn set_openvino_device(&mut self, device_name: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetOpenVINODevice(self.ptr, CString::new(device_name).unwrap().into_raw());
        }
    }

    pub fn set_lite_opt_model_dir(&mut self, optimized_model_dir: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteOptimizedModelDir(self.ptr,
                                                              CString::new(optimized_model_dir).unwrap().into_raw());
        }
    }

    pub fn set_lite_subgraph_partition_path(&mut self, nnadapter_subgraph_partition_config_path: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteSubgraphPartitionPath(self.ptr,
                                                                  CString::new(nnadapter_subgraph_partition_config_path).unwrap().into_raw());
        }
    }

    pub fn set_lite_subgraph_partition_config_buffer(&mut self, nnadapter_subgraph_partition_config_buffer: &[u8]) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteSubgraphPartitionConfigBuffer(self.ptr,
                                                                          CString::new(nnadapter_subgraph_partition_config_buffer).unwrap().into_raw());
        }
    }

    pub fn set_lite_context_properties(&mut self, nnadapter_context_properties: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteContextProperties(self.ptr,
                                                              CString::new(nnadapter_context_properties).unwrap().into_raw());
        }
    }

    pub fn set_lite_model_cache_dir(&mut self, nnadapter_model_cache_dir: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteModelCacheDir(self.ptr,
                                                          CString::new(nnadapter_model_cache_dir).unwrap().into_raw());
        }
    }
    pub fn set_lite_mixed_precision_quantization_config_path(&mut self, nnadapter_mixed_precision_quantization_config_path: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLiteMixedPrecisionQuantizationConfigPath(self.ptr,
                                                                                 CString::new(nnadapter_mixed_precision_quantization_config_path).unwrap().into_raw());
        }
    }

    pub fn enable_lite_fp16(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnableLiteFP16(self.ptr);
        }
    }
    pub fn disable_lite_fp16(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisableLiteFP16(self.ptr);
        }
    }

    pub fn enable_lite_int8(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnableLiteInt8(self.ptr);
        }
    }
    pub fn disable_lite_int8(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisableLiteInt8(self.ptr);
        }
    }

    pub fn set_lite_power_mode(&mut self, mode: &LitePowerMode) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetLitePowerMode(self.ptr, mode.to_raw());
        }
    }

    pub fn enable_trt_fp16(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnableTrtFP16(self.ptr);
        }
    }
    pub fn disable_trt_fp16(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisableTrtFP16(self.ptr);
        }
    }

    pub fn set_trt_cache_file(&mut self, cache_file_path: &str) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetTrtCacheFile(self.ptr,
                                                     CString::new(cache_file_path).unwrap().into_raw());
        }
    }

    pub fn enable_pinned_memory(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnablePinnedMemory(self.ptr);
        }
    }

    pub fn disable_pinned_memory(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisablePinnedMemory(self.ptr);
        }
    }

    pub fn enable_paddle_trt_collect_shape(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperEnablePaddleTrtCollectShape(self.ptr);
        }
    }

    pub fn disable_paddle_trt_collect_shape(&mut self) {
        unsafe {
            FD_C_RuntimeOptionWrapperDisablePaddleTrtCollectShape(self.ptr);
        }
    }

    pub fn set_openvino_streams(&mut self, num_streams: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperSetOpenVINOStreams(self.ptr,
                                                        num_streams);
        }
    }

    pub fn use_ipu(&mut self, device_num: i32, micro_batch_size: i32, enable_pipelining: bool, batches_per_step: i32) {
        unsafe {
            FD_C_RuntimeOptionWrapperUseIpu(self.ptr,
                                            device_num,
                                            micro_batch_size,
                                            enable_pipelining as FD_C_Bool,
                                            batches_per_step);
        }
    }
}

impl Drop for RuntimeOption {
    fn drop(&mut self) {
        unsafe {
            FD_C_DestroyRuntimeOptionWrapper(self.ptr);
        }
    }
}



