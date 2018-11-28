#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use cudart::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnContext {
    _unused: [u8; 0],
}
pub type cudnnHandle_t = *mut cudnnContext;
extern "C" {
    pub fn cudnnGetVersion() -> usize;
}
extern "C" {
    pub fn cudnnGetCudartVersion() -> usize;
}
pub const cudnnStatus_t_CUDNN_STATUS_SUCCESS: cudnnStatus_t = 0;
pub const cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED: cudnnStatus_t = 1;
pub const cudnnStatus_t_CUDNN_STATUS_ALLOC_FAILED: cudnnStatus_t = 2;
pub const cudnnStatus_t_CUDNN_STATUS_BAD_PARAM: cudnnStatus_t = 3;
pub const cudnnStatus_t_CUDNN_STATUS_INTERNAL_ERROR: cudnnStatus_t = 4;
pub const cudnnStatus_t_CUDNN_STATUS_INVALID_VALUE: cudnnStatus_t = 5;
pub const cudnnStatus_t_CUDNN_STATUS_ARCH_MISMATCH: cudnnStatus_t = 6;
pub const cudnnStatus_t_CUDNN_STATUS_MAPPING_ERROR: cudnnStatus_t = 7;
pub const cudnnStatus_t_CUDNN_STATUS_EXECUTION_FAILED: cudnnStatus_t = 8;
pub const cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED: cudnnStatus_t = 9;
pub const cudnnStatus_t_CUDNN_STATUS_LICENSE_ERROR: cudnnStatus_t = 10;
pub const cudnnStatus_t_CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING: cudnnStatus_t = 11;
pub const cudnnStatus_t_CUDNN_STATUS_RUNTIME_IN_PROGRESS: cudnnStatus_t = 12;
pub const cudnnStatus_t_CUDNN_STATUS_RUNTIME_FP_OVERFLOW: cudnnStatus_t = 13;
pub type cudnnStatus_t = u32;
extern "C" {
    pub fn cudnnGetErrorString(status: cudnnStatus_t) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRuntimeTag_t {
    _unused: [u8; 0],
}
pub const cudnnErrQueryMode_t_CUDNN_ERRQUERY_RAWCODE: cudnnErrQueryMode_t = 0;
pub const cudnnErrQueryMode_t_CUDNN_ERRQUERY_NONBLOCKING: cudnnErrQueryMode_t = 1;
pub const cudnnErrQueryMode_t_CUDNN_ERRQUERY_BLOCKING: cudnnErrQueryMode_t = 2;
pub type cudnnErrQueryMode_t = u32;
extern "C" {
    pub fn cudnnQueryRuntimeError(
        handle: cudnnHandle_t,
        rstatus: *mut cudnnStatus_t,
        mode: cudnnErrQueryMode_t,
        tag: *mut cudnnRuntimeTag_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetProperty(
        type_: libraryPropertyType,
        value: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCreate(handle: *mut cudnnHandle_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroy(handle: cudnnHandle_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetStream(handle: cudnnHandle_t, streamId: cudaStream_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetStream(handle: cudnnHandle_t, streamId: *mut cudaStream_t) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnTensorStruct {
    _unused: [u8; 0],
}
pub type cudnnTensorDescriptor_t = *mut cudnnTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionStruct {
    _unused: [u8; 0],
}
pub type cudnnConvolutionDescriptor_t = *mut cudnnConvolutionStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnPoolingStruct {
    _unused: [u8; 0],
}
pub type cudnnPoolingDescriptor_t = *mut cudnnPoolingStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnFilterStruct {
    _unused: [u8; 0],
}
pub type cudnnFilterDescriptor_t = *mut cudnnFilterStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnLRNStruct {
    _unused: [u8; 0],
}
pub type cudnnLRNDescriptor_t = *mut cudnnLRNStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnActivationStruct {
    _unused: [u8; 0],
}
pub type cudnnActivationDescriptor_t = *mut cudnnActivationStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnSpatialTransformerStruct {
    _unused: [u8; 0],
}
pub type cudnnSpatialTransformerDescriptor_t = *mut cudnnSpatialTransformerStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnOpTensorStruct {
    _unused: [u8; 0],
}
pub type cudnnOpTensorDescriptor_t = *mut cudnnOpTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnReduceTensorStruct {
    _unused: [u8; 0],
}
pub type cudnnReduceTensorDescriptor_t = *mut cudnnReduceTensorStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnCTCLossStruct {
    _unused: [u8; 0],
}
pub type cudnnCTCLossDescriptor_t = *mut cudnnCTCLossStruct;
pub const cudnnDataType_t_CUDNN_DATA_FLOAT: cudnnDataType_t = 0;
pub const cudnnDataType_t_CUDNN_DATA_DOUBLE: cudnnDataType_t = 1;
pub const cudnnDataType_t_CUDNN_DATA_HALF: cudnnDataType_t = 2;
pub const cudnnDataType_t_CUDNN_DATA_INT8: cudnnDataType_t = 3;
pub const cudnnDataType_t_CUDNN_DATA_INT32: cudnnDataType_t = 4;
pub const cudnnDataType_t_CUDNN_DATA_INT8x4: cudnnDataType_t = 5;
pub const cudnnDataType_t_CUDNN_DATA_UINT8: cudnnDataType_t = 6;
pub const cudnnDataType_t_CUDNN_DATA_UINT8x4: cudnnDataType_t = 7;
pub const cudnnDataType_t_CUDNN_DATA_INT8x32: cudnnDataType_t = 8;
pub type cudnnDataType_t = u32;
pub const cudnnMathType_t_CUDNN_DEFAULT_MATH: cudnnMathType_t = 0;
pub const cudnnMathType_t_CUDNN_TENSOR_OP_MATH: cudnnMathType_t = 1;
pub const cudnnMathType_t_CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION: cudnnMathType_t = 2;
pub type cudnnMathType_t = u32;
pub const cudnnNanPropagation_t_CUDNN_NOT_PROPAGATE_NAN: cudnnNanPropagation_t = 0;
pub const cudnnNanPropagation_t_CUDNN_PROPAGATE_NAN: cudnnNanPropagation_t = 1;
pub type cudnnNanPropagation_t = u32;
pub const cudnnDeterminism_t_CUDNN_NON_DETERMINISTIC: cudnnDeterminism_t = 0;
pub const cudnnDeterminism_t_CUDNN_DETERMINISTIC: cudnnDeterminism_t = 1;
pub type cudnnDeterminism_t = u32;
extern "C" {
    pub fn cudnnCreateTensorDescriptor(tensorDesc: *mut cudnnTensorDescriptor_t) -> cudnnStatus_t;
}
pub const cudnnTensorFormat_t_CUDNN_TENSOR_NCHW: cudnnTensorFormat_t = 0;
pub const cudnnTensorFormat_t_CUDNN_TENSOR_NHWC: cudnnTensorFormat_t = 1;
pub const cudnnTensorFormat_t_CUDNN_TENSOR_NCHW_VECT_C: cudnnTensorFormat_t = 2;
pub type cudnnTensorFormat_t = u32;
extern "C" {
    pub fn cudnnSetTensor4dDescriptor(
        tensorDesc: cudnnTensorDescriptor_t,
        format: cudnnTensorFormat_t,
        dataType: cudnnDataType_t,
        n: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetTensor4dDescriptorEx(
        tensorDesc: cudnnTensorDescriptor_t,
        dataType: cudnnDataType_t,
        n: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        nStride: ::std::os::raw::c_int,
        cStride: ::std::os::raw::c_int,
        hStride: ::std::os::raw::c_int,
        wStride: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetTensor4dDescriptor(
        tensorDesc: cudnnTensorDescriptor_t,
        dataType: *mut cudnnDataType_t,
        n: *mut ::std::os::raw::c_int,
        c: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
        nStride: *mut ::std::os::raw::c_int,
        cStride: *mut ::std::os::raw::c_int,
        hStride: *mut ::std::os::raw::c_int,
        wStride: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetTensorNdDescriptor(
        tensorDesc: cudnnTensorDescriptor_t,
        dataType: cudnnDataType_t,
        nbDims: ::std::os::raw::c_int,
        dimA: *const ::std::os::raw::c_int,
        strideA: *const ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetTensorNdDescriptorEx(
        tensorDesc: cudnnTensorDescriptor_t,
        format: cudnnTensorFormat_t,
        dataType: cudnnDataType_t,
        nbDims: ::std::os::raw::c_int,
        dimA: *const ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetTensorNdDescriptor(
        tensorDesc: cudnnTensorDescriptor_t,
        nbDimsRequested: ::std::os::raw::c_int,
        dataType: *mut cudnnDataType_t,
        nbDims: *mut ::std::os::raw::c_int,
        dimA: *mut ::std::os::raw::c_int,
        strideA: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetTensorSizeInBytes(
        tensorDesc: cudnnTensorDescriptor_t,
        size: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyTensorDescriptor(tensorDesc: cudnnTensorDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnTransformTensor(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnAddTensor(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        aDesc: cudnnTensorDescriptor_t,
        A: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        cDesc: cudnnTensorDescriptor_t,
        C: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_ADD: cudnnOpTensorOp_t = 0;
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_MUL: cudnnOpTensorOp_t = 1;
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_MIN: cudnnOpTensorOp_t = 2;
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_MAX: cudnnOpTensorOp_t = 3;
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_SQRT: cudnnOpTensorOp_t = 4;
pub const cudnnOpTensorOp_t_CUDNN_OP_TENSOR_NOT: cudnnOpTensorOp_t = 5;
pub type cudnnOpTensorOp_t = u32;
extern "C" {
    pub fn cudnnCreateOpTensorDescriptor(
        opTensorDesc: *mut cudnnOpTensorDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetOpTensorDescriptor(
        opTensorDesc: cudnnOpTensorDescriptor_t,
        opTensorOp: cudnnOpTensorOp_t,
        opTensorCompType: cudnnDataType_t,
        opTensorNanOpt: cudnnNanPropagation_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetOpTensorDescriptor(
        opTensorDesc: cudnnOpTensorDescriptor_t,
        opTensorOp: *mut cudnnOpTensorOp_t,
        opTensorCompType: *mut cudnnDataType_t,
        opTensorNanOpt: *mut cudnnNanPropagation_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyOpTensorDescriptor(opTensorDesc: cudnnOpTensorDescriptor_t)
        -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnOpTensor(
        handle: cudnnHandle_t,
        opTensorDesc: cudnnOpTensorDescriptor_t,
        alpha1: *const ::std::os::raw::c_void,
        aDesc: cudnnTensorDescriptor_t,
        A: *const ::std::os::raw::c_void,
        alpha2: *const ::std::os::raw::c_void,
        bDesc: cudnnTensorDescriptor_t,
        B: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        cDesc: cudnnTensorDescriptor_t,
        C: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_ADD: cudnnReduceTensorOp_t = 0;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_MUL: cudnnReduceTensorOp_t = 1;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_MIN: cudnnReduceTensorOp_t = 2;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_MAX: cudnnReduceTensorOp_t = 3;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_AMAX: cudnnReduceTensorOp_t = 4;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_AVG: cudnnReduceTensorOp_t = 5;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_NORM1: cudnnReduceTensorOp_t = 6;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_NORM2: cudnnReduceTensorOp_t = 7;
pub const cudnnReduceTensorOp_t_CUDNN_REDUCE_TENSOR_MUL_NO_ZEROS: cudnnReduceTensorOp_t = 8;
pub type cudnnReduceTensorOp_t = u32;
pub const cudnnReduceTensorIndices_t_CUDNN_REDUCE_TENSOR_NO_INDICES: cudnnReduceTensorIndices_t = 0;
pub const cudnnReduceTensorIndices_t_CUDNN_REDUCE_TENSOR_FLATTENED_INDICES:
    cudnnReduceTensorIndices_t = 1;
pub type cudnnReduceTensorIndices_t = u32;
pub const cudnnIndicesType_t_CUDNN_32BIT_INDICES: cudnnIndicesType_t = 0;
pub const cudnnIndicesType_t_CUDNN_64BIT_INDICES: cudnnIndicesType_t = 1;
pub const cudnnIndicesType_t_CUDNN_16BIT_INDICES: cudnnIndicesType_t = 2;
pub const cudnnIndicesType_t_CUDNN_8BIT_INDICES: cudnnIndicesType_t = 3;
pub type cudnnIndicesType_t = u32;
extern "C" {
    pub fn cudnnCreateReduceTensorDescriptor(
        reduceTensorDesc: *mut cudnnReduceTensorDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetReduceTensorDescriptor(
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        reduceTensorOp: cudnnReduceTensorOp_t,
        reduceTensorCompType: cudnnDataType_t,
        reduceTensorNanOpt: cudnnNanPropagation_t,
        reduceTensorIndices: cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: cudnnIndicesType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetReduceTensorDescriptor(
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        reduceTensorOp: *mut cudnnReduceTensorOp_t,
        reduceTensorCompType: *mut cudnnDataType_t,
        reduceTensorNanOpt: *mut cudnnNanPropagation_t,
        reduceTensorIndices: *mut cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: *mut cudnnIndicesType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyReduceTensorDescriptor(
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetReductionIndicesSize(
        handle: cudnnHandle_t,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        aDesc: cudnnTensorDescriptor_t,
        cDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetReductionWorkspaceSize(
        handle: cudnnHandle_t,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        aDesc: cudnnTensorDescriptor_t,
        cDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnReduceTensor(
        handle: cudnnHandle_t,
        reduceTensorDesc: cudnnReduceTensorDescriptor_t,
        indices: *mut ::std::os::raw::c_void,
        indicesSizeInBytes: usize,
        workspace: *mut ::std::os::raw::c_void,
        workspaceSizeInBytes: usize,
        alpha: *const ::std::os::raw::c_void,
        aDesc: cudnnTensorDescriptor_t,
        A: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        cDesc: cudnnTensorDescriptor_t,
        C: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetTensor(
        handle: cudnnHandle_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        valuePtr: *const ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnScaleTensor(
        handle: cudnnHandle_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        alpha: *const ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnConvolutionMode_t_CUDNN_CONVOLUTION: cudnnConvolutionMode_t = 0;
pub const cudnnConvolutionMode_t_CUDNN_CROSS_CORRELATION: cudnnConvolutionMode_t = 1;
pub type cudnnConvolutionMode_t = u32;
extern "C" {
    pub fn cudnnCreateFilterDescriptor(filterDesc: *mut cudnnFilterDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetFilter4dDescriptor(
        filterDesc: cudnnFilterDescriptor_t,
        dataType: cudnnDataType_t,
        format: cudnnTensorFormat_t,
        k: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetFilter4dDescriptor(
        filterDesc: cudnnFilterDescriptor_t,
        dataType: *mut cudnnDataType_t,
        format: *mut cudnnTensorFormat_t,
        k: *mut ::std::os::raw::c_int,
        c: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetFilterNdDescriptor(
        filterDesc: cudnnFilterDescriptor_t,
        dataType: cudnnDataType_t,
        format: cudnnTensorFormat_t,
        nbDims: ::std::os::raw::c_int,
        filterDimA: *const ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetFilterNdDescriptor(
        filterDesc: cudnnFilterDescriptor_t,
        nbDimsRequested: ::std::os::raw::c_int,
        dataType: *mut cudnnDataType_t,
        format: *mut cudnnTensorFormat_t,
        nbDims: *mut ::std::os::raw::c_int,
        filterDimA: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyFilterDescriptor(filterDesc: cudnnFilterDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCreateConvolutionDescriptor(
        convDesc: *mut cudnnConvolutionDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetConvolutionMathType(
        convDesc: cudnnConvolutionDescriptor_t,
        mathType: cudnnMathType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionMathType(
        convDesc: cudnnConvolutionDescriptor_t,
        mathType: *mut cudnnMathType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetConvolutionGroupCount(
        convDesc: cudnnConvolutionDescriptor_t,
        groupCount: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionGroupCount(
        convDesc: cudnnConvolutionDescriptor_t,
        groupCount: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetConvolution2dDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        pad_h: ::std::os::raw::c_int,
        pad_w: ::std::os::raw::c_int,
        u: ::std::os::raw::c_int,
        v: ::std::os::raw::c_int,
        dilation_h: ::std::os::raw::c_int,
        dilation_w: ::std::os::raw::c_int,
        mode: cudnnConvolutionMode_t,
        computeType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolution2dDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        pad_h: *mut ::std::os::raw::c_int,
        pad_w: *mut ::std::os::raw::c_int,
        u: *mut ::std::os::raw::c_int,
        v: *mut ::std::os::raw::c_int,
        dilation_h: *mut ::std::os::raw::c_int,
        dilation_w: *mut ::std::os::raw::c_int,
        mode: *mut cudnnConvolutionMode_t,
        computeType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolution2dForwardOutputDim(
        convDesc: cudnnConvolutionDescriptor_t,
        inputTensorDesc: cudnnTensorDescriptor_t,
        filterDesc: cudnnFilterDescriptor_t,
        n: *mut ::std::os::raw::c_int,
        c: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetConvolutionNdDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        arrayLength: ::std::os::raw::c_int,
        padA: *const ::std::os::raw::c_int,
        filterStrideA: *const ::std::os::raw::c_int,
        dilationA: *const ::std::os::raw::c_int,
        mode: cudnnConvolutionMode_t,
        computeType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionNdDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
        arrayLengthRequested: ::std::os::raw::c_int,
        arrayLength: *mut ::std::os::raw::c_int,
        padA: *mut ::std::os::raw::c_int,
        strideA: *mut ::std::os::raw::c_int,
        dilationA: *mut ::std::os::raw::c_int,
        mode: *mut cudnnConvolutionMode_t,
        computeType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionNdForwardOutputDim(
        convDesc: cudnnConvolutionDescriptor_t,
        inputTensorDesc: cudnnTensorDescriptor_t,
        filterDesc: cudnnFilterDescriptor_t,
        nbDims: ::std::os::raw::c_int,
        tensorOuputDimA: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyConvolutionDescriptor(
        convDesc: cudnnConvolutionDescriptor_t,
    ) -> cudnnStatus_t;
}
pub const cudnnConvolutionFwdPreference_t_CUDNN_CONVOLUTION_FWD_NO_WORKSPACE:
    cudnnConvolutionFwdPreference_t = 0;
pub const cudnnConvolutionFwdPreference_t_CUDNN_CONVOLUTION_FWD_PREFER_FASTEST:
    cudnnConvolutionFwdPreference_t = 1;
pub const cudnnConvolutionFwdPreference_t_CUDNN_CONVOLUTION_FWD_SPECIFY_WORKSPACE_LIMIT:
    cudnnConvolutionFwdPreference_t = 2;
pub type cudnnConvolutionFwdPreference_t = u32;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM:
    cudnnConvolutionFwdAlgo_t = 0;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM:
    cudnnConvolutionFwdAlgo_t = 1;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_GEMM: cudnnConvolutionFwdAlgo_t = 2;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_DIRECT: cudnnConvolutionFwdAlgo_t =
    3;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_FFT: cudnnConvolutionFwdAlgo_t = 4;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING:
    cudnnConvolutionFwdAlgo_t = 5;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD: cudnnConvolutionFwdAlgo_t =
    6;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED:
    cudnnConvolutionFwdAlgo_t = 7;
pub const cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_COUNT: cudnnConvolutionFwdAlgo_t = 8;
pub type cudnnConvolutionFwdAlgo_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionFwdAlgoPerf_t {
    pub algo: cudnnConvolutionFwdAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::std::os::raw::c_int; 3usize],
}
#[test]
fn bindgen_test_layout_cudnnConvolutionFwdAlgoPerf_t() {
    assert_eq!(
        ::std::mem::size_of::<cudnnConvolutionFwdAlgoPerf_t>(),
        48usize,
        concat!("Size of: ", stringify!(cudnnConvolutionFwdAlgoPerf_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnConvolutionFwdAlgoPerf_t>(),
        8usize,
        concat!("Alignment of ", stringify!(cudnnConvolutionFwdAlgoPerf_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).algo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(algo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).status as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).time as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).memory as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(memory)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).determinism as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(determinism)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).mathType as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(mathType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionFwdAlgoPerf_t>())).reserved as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionFwdAlgoPerf_t),
            "::",
            stringify!(reserved)
        )
    );
}
extern "C" {
    pub fn cudnnGetConvolutionForwardAlgorithmMaxCount(
        handle: cudnnHandle_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionForwardAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionForwardAlgorithmEx(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionForwardAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        preference: cudnnConvolutionFwdPreference_t,
        memoryLimitInBytes: usize,
        algo: *mut cudnnConvolutionFwdAlgo_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionForwardAlgorithm_v7(
        handle: cudnnHandle_t,
        srcDesc: cudnnTensorDescriptor_t,
        filterDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        destDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionForwardWorkspaceSize(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnConvolutionForward(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnConvolutionBiasActivationForward(
        handle: cudnnHandle_t,
        alpha1: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        alpha2: *const ::std::os::raw::c_void,
        zDesc: cudnnTensorDescriptor_t,
        z: *const ::std::os::raw::c_void,
        biasDesc: cudnnTensorDescriptor_t,
        bias: *const ::std::os::raw::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnConvolutionBackwardBias(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dbDesc: cudnnTensorDescriptor_t,
        db: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnConvolutionBwdFilterPreference_t_CUDNN_CONVOLUTION_BWD_FILTER_NO_WORKSPACE:
    cudnnConvolutionBwdFilterPreference_t = 0;
pub const cudnnConvolutionBwdFilterPreference_t_CUDNN_CONVOLUTION_BWD_FILTER_PREFER_FASTEST:
    cudnnConvolutionBwdFilterPreference_t = 1;
pub const cudnnConvolutionBwdFilterPreference_t_CUDNN_CONVOLUTION_BWD_FILTER_SPECIFY_WORKSPACE_LIMIT : cudnnConvolutionBwdFilterPreference_t = 2 ;
pub type cudnnConvolutionBwdFilterPreference_t = u32;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0:
    cudnnConvolutionBwdFilterAlgo_t = 0;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1:
    cudnnConvolutionBwdFilterAlgo_t = 1;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT:
    cudnnConvolutionBwdFilterAlgo_t = 2;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3:
    cudnnConvolutionBwdFilterAlgo_t = 3;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD:
    cudnnConvolutionBwdFilterAlgo_t = 4;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD_NONFUSED:
    cudnnConvolutionBwdFilterAlgo_t = 5;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT_TILING:
    cudnnConvolutionBwdFilterAlgo_t = 6;
pub const cudnnConvolutionBwdFilterAlgo_t_CUDNN_CONVOLUTION_BWD_FILTER_ALGO_COUNT:
    cudnnConvolutionBwdFilterAlgo_t = 7;
pub type cudnnConvolutionBwdFilterAlgo_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionBwdFilterAlgoPerf_t {
    pub algo: cudnnConvolutionBwdFilterAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::std::os::raw::c_int; 3usize],
}
#[test]
fn bindgen_test_layout_cudnnConvolutionBwdFilterAlgoPerf_t() {
    assert_eq!(
        ::std::mem::size_of::<cudnnConvolutionBwdFilterAlgoPerf_t>(),
        48usize,
        concat!("Size of: ", stringify!(cudnnConvolutionBwdFilterAlgoPerf_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnConvolutionBwdFilterAlgoPerf_t>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).algo as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(algo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).status as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).time as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).memory as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(memory)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).determinism as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(determinism)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).mathType as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(mathType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdFilterAlgoPerf_t>())).reserved as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdFilterAlgoPerf_t),
            "::",
            stringify!(reserved)
        )
    );
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
        handle: cudnnHandle_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionBackwardFilterAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::std::os::raw::c_void,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardFilterAlgorithm(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dwDesc: cudnnFilterDescriptor_t,
        preference: cudnnConvolutionBwdFilterPreference_t,
        memoryLimitInBytes: usize,
        algo: *mut cudnnConvolutionBwdFilterAlgo_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        handle: cudnnHandle_t,
        srcDesc: cudnnTensorDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnFilterDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnConvolutionBackwardFilter(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::std::os::raw::c_void,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnConvolutionBwdDataPreference_t_CUDNN_CONVOLUTION_BWD_DATA_NO_WORKSPACE:
    cudnnConvolutionBwdDataPreference_t = 0;
pub const cudnnConvolutionBwdDataPreference_t_CUDNN_CONVOLUTION_BWD_DATA_PREFER_FASTEST:
    cudnnConvolutionBwdDataPreference_t = 1;
pub const cudnnConvolutionBwdDataPreference_t_CUDNN_CONVOLUTION_BWD_DATA_SPECIFY_WORKSPACE_LIMIT:
    cudnnConvolutionBwdDataPreference_t = 2;
pub type cudnnConvolutionBwdDataPreference_t = u32;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_0:
    cudnnConvolutionBwdDataAlgo_t = 0;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_1:
    cudnnConvolutionBwdDataAlgo_t = 1;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT:
    cudnnConvolutionBwdDataAlgo_t = 2;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING:
    cudnnConvolutionBwdDataAlgo_t = 3;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD:
    cudnnConvolutionBwdDataAlgo_t = 4;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED:
    cudnnConvolutionBwdDataAlgo_t = 5;
pub const cudnnConvolutionBwdDataAlgo_t_CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT:
    cudnnConvolutionBwdDataAlgo_t = 6;
pub type cudnnConvolutionBwdDataAlgo_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnConvolutionBwdDataAlgoPerf_t {
    pub algo: cudnnConvolutionBwdDataAlgo_t,
    pub status: cudnnStatus_t,
    pub time: f32,
    pub memory: usize,
    pub determinism: cudnnDeterminism_t,
    pub mathType: cudnnMathType_t,
    pub reserved: [::std::os::raw::c_int; 3usize],
}
#[test]
fn bindgen_test_layout_cudnnConvolutionBwdDataAlgoPerf_t() {
    assert_eq!(
        ::std::mem::size_of::<cudnnConvolutionBwdDataAlgoPerf_t>(),
        48usize,
        concat!("Size of: ", stringify!(cudnnConvolutionBwdDataAlgoPerf_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnConvolutionBwdDataAlgoPerf_t>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).algo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(algo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).status as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).time as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).memory as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(memory)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).determinism as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(determinism)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).mathType as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(mathType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnConvolutionBwdDataAlgoPerf_t>())).reserved as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnConvolutionBwdDataAlgoPerf_t),
            "::",
            stringify!(reserved)
        )
    );
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
        handle: cudnnHandle_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionBackwardDataAlgorithm(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardDataAlgorithm(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        preference: cudnnConvolutionBwdDataPreference_t,
        memoryLimitInBytes: usize,
        algo: *mut cudnnConvolutionBwdDataAlgo_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        handle: cudnnHandle_t,
        filterDesc: cudnnFilterDescriptor_t,
        diffDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        gradDesc: cudnnTensorDescriptor_t,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetConvolutionBackwardDataWorkspaceSize(
        handle: cudnnHandle_t,
        wDesc: cudnnFilterDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnConvolutionBackwardData(
        handle: cudnnHandle_t,
        alpha: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        convDesc: cudnnConvolutionDescriptor_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnIm2Col(
        handle: cudnnHandle_t,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        convDesc: cudnnConvolutionDescriptor_t,
        colBuffer: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_FAST: cudnnSoftmaxAlgorithm_t = 0;
pub const cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_ACCURATE: cudnnSoftmaxAlgorithm_t = 1;
pub const cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_LOG: cudnnSoftmaxAlgorithm_t = 2;
pub type cudnnSoftmaxAlgorithm_t = u32;
pub const cudnnSoftmaxMode_t_CUDNN_SOFTMAX_MODE_INSTANCE: cudnnSoftmaxMode_t = 0;
pub const cudnnSoftmaxMode_t_CUDNN_SOFTMAX_MODE_CHANNEL: cudnnSoftmaxMode_t = 1;
pub type cudnnSoftmaxMode_t = u32;
extern "C" {
    pub fn cudnnSoftmaxForward(
        handle: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSoftmaxBackward(
        handle: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnPoolingMode_t_CUDNN_POOLING_MAX: cudnnPoolingMode_t = 0;
pub const cudnnPoolingMode_t_CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING: cudnnPoolingMode_t = 1;
pub const cudnnPoolingMode_t_CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING: cudnnPoolingMode_t = 2;
pub const cudnnPoolingMode_t_CUDNN_POOLING_MAX_DETERMINISTIC: cudnnPoolingMode_t = 3;
pub type cudnnPoolingMode_t = u32;
extern "C" {
    pub fn cudnnCreatePoolingDescriptor(
        poolingDesc: *mut cudnnPoolingDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetPooling2dDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: cudnnPoolingMode_t,
        maxpoolingNanOpt: cudnnNanPropagation_t,
        windowHeight: ::std::os::raw::c_int,
        windowWidth: ::std::os::raw::c_int,
        verticalPadding: ::std::os::raw::c_int,
        horizontalPadding: ::std::os::raw::c_int,
        verticalStride: ::std::os::raw::c_int,
        horizontalStride: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetPooling2dDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: *mut cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cudnnNanPropagation_t,
        windowHeight: *mut ::std::os::raw::c_int,
        windowWidth: *mut ::std::os::raw::c_int,
        verticalPadding: *mut ::std::os::raw::c_int,
        horizontalPadding: *mut ::std::os::raw::c_int,
        verticalStride: *mut ::std::os::raw::c_int,
        horizontalStride: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetPoolingNdDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        mode: cudnnPoolingMode_t,
        maxpoolingNanOpt: cudnnNanPropagation_t,
        nbDims: ::std::os::raw::c_int,
        windowDimA: *const ::std::os::raw::c_int,
        paddingA: *const ::std::os::raw::c_int,
        strideA: *const ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetPoolingNdDescriptor(
        poolingDesc: cudnnPoolingDescriptor_t,
        nbDimsRequested: ::std::os::raw::c_int,
        mode: *mut cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cudnnNanPropagation_t,
        nbDims: *mut ::std::os::raw::c_int,
        windowDimA: *mut ::std::os::raw::c_int,
        paddingA: *mut ::std::os::raw::c_int,
        strideA: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetPoolingNdForwardOutputDim(
        poolingDesc: cudnnPoolingDescriptor_t,
        inputTensorDesc: cudnnTensorDescriptor_t,
        nbDims: ::std::os::raw::c_int,
        outputTensorDimA: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetPooling2dForwardOutputDim(
        poolingDesc: cudnnPoolingDescriptor_t,
        inputTensorDesc: cudnnTensorDescriptor_t,
        n: *mut ::std::os::raw::c_int,
        c: *mut ::std::os::raw::c_int,
        h: *mut ::std::os::raw::c_int,
        w: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyPoolingDescriptor(poolingDesc: cudnnPoolingDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnPoolingForward(
        handle: cudnnHandle_t,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnPoolingBackward(
        handle: cudnnHandle_t,
        poolingDesc: cudnnPoolingDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_SIGMOID: cudnnActivationMode_t = 0;
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_RELU: cudnnActivationMode_t = 1;
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_TANH: cudnnActivationMode_t = 2;
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_CLIPPED_RELU: cudnnActivationMode_t = 3;
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_ELU: cudnnActivationMode_t = 4;
pub const cudnnActivationMode_t_CUDNN_ACTIVATION_IDENTITY: cudnnActivationMode_t = 5;
pub type cudnnActivationMode_t = u32;
extern "C" {
    pub fn cudnnCreateActivationDescriptor(
        activationDesc: *mut cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetActivationDescriptor(
        activationDesc: cudnnActivationDescriptor_t,
        mode: cudnnActivationMode_t,
        reluNanOpt: cudnnNanPropagation_t,
        coef: f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetActivationDescriptor(
        activationDesc: cudnnActivationDescriptor_t,
        mode: *mut cudnnActivationMode_t,
        reluNanOpt: *mut cudnnNanPropagation_t,
        coef: *mut f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyActivationDescriptor(
        activationDesc: cudnnActivationDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnActivationForward(
        handle: cudnnHandle_t,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnActivationBackward(
        handle: cudnnHandle_t,
        activationDesc: cudnnActivationDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCreateLRNDescriptor(normDesc: *mut cudnnLRNDescriptor_t) -> cudnnStatus_t;
}
pub const cudnnLRNMode_t_CUDNN_LRN_CROSS_CHANNEL_DIM1: cudnnLRNMode_t = 0;
pub type cudnnLRNMode_t = u32;
extern "C" {
    pub fn cudnnSetLRNDescriptor(
        normDesc: cudnnLRNDescriptor_t,
        lrnN: ::std::os::raw::c_uint,
        lrnAlpha: f64,
        lrnBeta: f64,
        lrnK: f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetLRNDescriptor(
        normDesc: cudnnLRNDescriptor_t,
        lrnN: *mut ::std::os::raw::c_uint,
        lrnAlpha: *mut f64,
        lrnBeta: *mut f64,
        lrnK: *mut f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyLRNDescriptor(lrnDesc: cudnnLRNDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnLRNCrossChannelForward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnLRNCrossChannelBackward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        lrnMode: cudnnLRNMode_t,
        alpha: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnDivNormMode_t_CUDNN_DIVNORM_PRECOMPUTED_MEANS: cudnnDivNormMode_t = 0;
pub type cudnnDivNormMode_t = u32;
extern "C" {
    pub fn cudnnDivisiveNormalizationForward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        means: *const ::std::os::raw::c_void,
        temp: *mut ::std::os::raw::c_void,
        temp2: *mut ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDivisiveNormalizationBackward(
        handle: cudnnHandle_t,
        normDesc: cudnnLRNDescriptor_t,
        mode: cudnnDivNormMode_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        means: *const ::std::os::raw::c_void,
        dy: *const ::std::os::raw::c_void,
        temp: *mut ::std::os::raw::c_void,
        temp2: *mut ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dXdMeansDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        dMeans: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
pub const cudnnBatchNormMode_t_CUDNN_BATCHNORM_PER_ACTIVATION: cudnnBatchNormMode_t = 0;
pub const cudnnBatchNormMode_t_CUDNN_BATCHNORM_SPATIAL: cudnnBatchNormMode_t = 1;
pub const cudnnBatchNormMode_t_CUDNN_BATCHNORM_SPATIAL_PERSISTENT: cudnnBatchNormMode_t = 2;
pub type cudnnBatchNormMode_t = u32;
extern "C" {
    pub fn cudnnDeriveBNTensorDescriptor(
        derivedBnDesc: cudnnTensorDescriptor_t,
        xDesc: cudnnTensorDescriptor_t,
        mode: cudnnBatchNormMode_t,
    ) -> cudnnStatus_t;
}
pub const cudnnBatchNormOps_t_CUDNN_BATCHNORM_OPS_BN: cudnnBatchNormOps_t = 0;
pub const cudnnBatchNormOps_t_CUDNN_BATCHNORM_OPS_BN_ACTIVATION: cudnnBatchNormOps_t = 1;
pub const cudnnBatchNormOps_t_CUDNN_BATCHNORM_OPS_BN_ADD_ACTIVATION: cudnnBatchNormOps_t = 2;
pub type cudnnBatchNormOps_t = u32;
extern "C" {
    pub fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        zDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        xDesc: cudnnTensorDescriptor_t,
        yDesc: cudnnTensorDescriptor_t,
        dyDesc: cudnnTensorDescriptor_t,
        dzDesc: cudnnTensorDescriptor_t,
        dxDesc: cudnnTensorDescriptor_t,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        activationDesc: cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        activationDesc: cudnnActivationDescriptor_t,
        xDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnBatchNormalizationForwardTraining(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::std::os::raw::c_void,
        bnBias: *const ::std::os::raw::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::std::os::raw::c_void,
        resultRunningVariance: *mut ::std::os::raw::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::std::os::raw::c_void,
        resultSaveInvVariance: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnBatchNormalizationForwardTrainingEx(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alpha: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::std::os::raw::c_void,
        zDesc: cudnnTensorDescriptor_t,
        zData: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *mut ::std::os::raw::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::std::os::raw::c_void,
        bnBias: *const ::std::os::raw::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::std::os::raw::c_void,
        resultRunningVariance: *mut ::std::os::raw::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::std::os::raw::c_void,
        resultSaveInvVariance: *mut ::std::os::raw::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnBatchNormalizationForwardInference(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::std::os::raw::c_void,
        bnBias: *const ::std::os::raw::c_void,
        estimatedMean: *const ::std::os::raw::c_void,
        estimatedVariance: *const ::std::os::raw::c_void,
        epsilon: f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnBatchNormalizationBackward(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alphaDataDiff: *const ::std::os::raw::c_void,
        betaDataDiff: *const ::std::os::raw::c_void,
        alphaParamDiff: *const ::std::os::raw::c_void,
        betaParamDiff: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScale: *const ::std::os::raw::c_void,
        dBnScaleResult: *mut ::std::os::raw::c_void,
        dBnBiasResult: *mut ::std::os::raw::c_void,
        epsilon: f64,
        savedMean: *const ::std::os::raw::c_void,
        savedInvVariance: *const ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnBatchNormalizationBackwardEx(
        handle: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        bnOps: cudnnBatchNormOps_t,
        alphaDataDiff: *const ::std::os::raw::c_void,
        betaDataDiff: *const ::std::os::raw::c_void,
        alphaParamDiff: *const ::std::os::raw::c_void,
        betaParamDiff: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        xData: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        yData: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dyData: *const ::std::os::raw::c_void,
        dzDesc: cudnnTensorDescriptor_t,
        dzData: *mut ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dxData: *mut ::std::os::raw::c_void,
        dBnScaleBiasDesc: cudnnTensorDescriptor_t,
        bnScaleData: *const ::std::os::raw::c_void,
        bnBiasData: *const ::std::os::raw::c_void,
        dBnScaleData: *mut ::std::os::raw::c_void,
        dBnBiasData: *mut ::std::os::raw::c_void,
        epsilon: f64,
        savedMean: *const ::std::os::raw::c_void,
        savedInvVariance: *const ::std::os::raw::c_void,
        activationDesc: cudnnActivationDescriptor_t,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
pub const cudnnSamplerType_t_CUDNN_SAMPLER_BILINEAR: cudnnSamplerType_t = 0;
pub type cudnnSamplerType_t = u32;
extern "C" {
    pub fn cudnnCreateSpatialTransformerDescriptor(
        stDesc: *mut cudnnSpatialTransformerDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetSpatialTransformerNdDescriptor(
        stDesc: cudnnSpatialTransformerDescriptor_t,
        samplerType: cudnnSamplerType_t,
        dataType: cudnnDataType_t,
        nbDims: ::std::os::raw::c_int,
        dimA: *const ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroySpatialTransformerDescriptor(
        stDesc: cudnnSpatialTransformerDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSpatialTfGridGeneratorForward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        theta: *const ::std::os::raw::c_void,
        grid: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSpatialTfGridGeneratorBackward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        dgrid: *const ::std::os::raw::c_void,
        dtheta: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSpatialTfSamplerForward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        grid: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        yDesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSpatialTfSamplerBackward(
        handle: cudnnHandle_t,
        stDesc: cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::std::os::raw::c_void,
        xDesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        beta: *const ::std::os::raw::c_void,
        dxDesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        alphaDgrid: *const ::std::os::raw::c_void,
        dyDesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        grid: *const ::std::os::raw::c_void,
        betaDgrid: *const ::std::os::raw::c_void,
        dgrid: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnDropoutStruct {
    _unused: [u8; 0],
}
pub type cudnnDropoutDescriptor_t = *mut cudnnDropoutStruct;
extern "C" {
    pub fn cudnnCreateDropoutDescriptor(
        dropoutDesc: *mut cudnnDropoutDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyDropoutDescriptor(dropoutDesc: cudnnDropoutDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDropoutGetStatesSize(
        handle: cudnnHandle_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDropoutGetReserveSpaceSize(
        xdesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetDropoutDescriptor(
        dropoutDesc: cudnnDropoutDescriptor_t,
        handle: cudnnHandle_t,
        dropout: f32,
        states: *mut ::std::os::raw::c_void,
        stateSizeInBytes: usize,
        seed: ::std::os::raw::c_ulonglong,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRestoreDropoutDescriptor(
        dropoutDesc: cudnnDropoutDescriptor_t,
        handle: cudnnHandle_t,
        dropout: f32,
        states: *mut ::std::os::raw::c_void,
        stateSizeInBytes: usize,
        seed: ::std::os::raw::c_ulonglong,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetDropoutDescriptor(
        dropoutDesc: cudnnDropoutDescriptor_t,
        handle: cudnnHandle_t,
        dropout: *mut f32,
        states: *mut *mut ::std::os::raw::c_void,
        seed: *mut ::std::os::raw::c_ulonglong,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDropoutForward(
        handle: cudnnHandle_t,
        dropoutDesc: cudnnDropoutDescriptor_t,
        xdesc: cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        ydesc: cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDropoutBackward(
        handle: cudnnHandle_t,
        dropoutDesc: cudnnDropoutDescriptor_t,
        dydesc: cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        dxdesc: cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
pub const cudnnRNNMode_t_CUDNN_RNN_RELU: cudnnRNNMode_t = 0;
pub const cudnnRNNMode_t_CUDNN_RNN_TANH: cudnnRNNMode_t = 1;
pub const cudnnRNNMode_t_CUDNN_LSTM: cudnnRNNMode_t = 2;
pub const cudnnRNNMode_t_CUDNN_GRU: cudnnRNNMode_t = 3;
pub type cudnnRNNMode_t = u32;
pub const cudnnDirectionMode_t_CUDNN_UNIDIRECTIONAL: cudnnDirectionMode_t = 0;
pub const cudnnDirectionMode_t_CUDNN_BIDIRECTIONAL: cudnnDirectionMode_t = 1;
pub type cudnnDirectionMode_t = u32;
pub const cudnnRNNInputMode_t_CUDNN_LINEAR_INPUT: cudnnRNNInputMode_t = 0;
pub const cudnnRNNInputMode_t_CUDNN_SKIP_INPUT: cudnnRNNInputMode_t = 1;
pub type cudnnRNNInputMode_t = u32;
pub const cudnnRNNAlgo_t_CUDNN_RNN_ALGO_STANDARD: cudnnRNNAlgo_t = 0;
pub const cudnnRNNAlgo_t_CUDNN_RNN_ALGO_PERSIST_STATIC: cudnnRNNAlgo_t = 1;
pub const cudnnRNNAlgo_t_CUDNN_RNN_ALGO_PERSIST_DYNAMIC: cudnnRNNAlgo_t = 2;
pub const cudnnRNNAlgo_t_CUDNN_RNN_ALGO_COUNT: cudnnRNNAlgo_t = 3;
pub type cudnnRNNAlgo_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAlgorithmStruct {
    _unused: [u8; 0],
}
pub type cudnnAlgorithmDescriptor_t = *mut cudnnAlgorithmStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnAlgorithmPerformanceStruct {
    _unused: [u8; 0],
}
pub type cudnnAlgorithmPerformance_t = *mut cudnnAlgorithmPerformanceStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNStruct {
    _unused: [u8; 0],
}
pub type cudnnRNNDescriptor_t = *mut cudnnRNNStruct;
extern "C" {
    pub fn cudnnCreateRNNDescriptor(rnnDesc: *mut cudnnRNNDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyRNNDescriptor(rnnDesc: cudnnRNNDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNForwardInferenceAlgorithmMaxCount(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindRNNForwardInferenceAlgorithmEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnAlgorithmPerformance_t,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNForwardTrainingAlgorithmMaxCount(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindRNNForwardTrainingAlgorithmEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnAlgorithmPerformance_t,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNBackwardDataAlgorithmMaxCount(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindRNNBackwardDataAlgorithmEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: *const cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        dhyDesc: cudnnTensorDescriptor_t,
        dhy: *const ::std::os::raw::c_void,
        dcyDesc: cudnnTensorDescriptor_t,
        dcy: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        dxDesc: *const cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        dhxDesc: cudnnTensorDescriptor_t,
        dhx: *mut ::std::os::raw::c_void,
        dcxDesc: cudnnTensorDescriptor_t,
        dcx: *mut ::std::os::raw::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnAlgorithmPerformance_t,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNBackwardWeightsAlgorithmMaxCount(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        count: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnFindRNNBackwardWeightsAlgorithmEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::std::os::raw::c_int,
        returnedAlgoCount: *mut ::std::os::raw::c_int,
        perfResults: *mut cudnnAlgorithmPerformance_t,
        workspace: *const ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::std::os::raw::c_void,
        reserveSpace: *const ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnPersistentRNNPlan {
    _unused: [u8; 0],
}
pub type cudnnPersistentRNNPlan_t = *mut cudnnPersistentRNNPlan;
extern "C" {
    pub fn cudnnCreatePersistentRNNPlan(
        rnnDesc: cudnnRNNDescriptor_t,
        minibatch: ::std::os::raw::c_int,
        dataType: cudnnDataType_t,
        plan: *mut cudnnPersistentRNNPlan_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetPersistentRNNPlan(
        rnnDesc: cudnnRNNDescriptor_t,
        plan: cudnnPersistentRNNPlan_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyPersistentRNNPlan(plan: cudnnPersistentRNNPlan_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNDescriptor(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        hiddenSize: ::std::os::raw::c_int,
        numLayers: ::std::os::raw::c_int,
        dropoutDesc: cudnnDropoutDescriptor_t,
        inputMode: cudnnRNNInputMode_t,
        direction: cudnnDirectionMode_t,
        mode: cudnnRNNMode_t,
        algo: cudnnRNNAlgo_t,
        dataType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNProjectionLayers(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        recProjSize: ::std::os::raw::c_int,
        outProjSize: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNProjectionLayers(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        recProjSize: *mut ::std::os::raw::c_int,
        outProjSize: *mut ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNAlgorithmDescriptor(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        algoDesc: cudnnAlgorithmDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNDescriptor(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        hiddenSize: *mut ::std::os::raw::c_int,
        numLayers: *mut ::std::os::raw::c_int,
        dropoutDesc: *mut cudnnDropoutDescriptor_t,
        inputMode: *mut cudnnRNNInputMode_t,
        direction: *mut cudnnDirectionMode_t,
        mode: *mut cudnnRNNMode_t,
        algo: *mut cudnnRNNAlgo_t,
        dataType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNMatrixMathType(
        rnnDesc: cudnnRNNDescriptor_t,
        mType: cudnnMathType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNMatrixMathType(
        rnnDesc: cudnnRNNDescriptor_t,
        mType: *mut cudnnMathType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNWorkspaceSize(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNTrainingReserveSize(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNParamsSize(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        xDesc: cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        dataType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNLinLayerMatrixParams(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        pseudoLayer: ::std::os::raw::c_int,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        linLayerID: ::std::os::raw::c_int,
        linLayerMatDesc: cudnnFilterDescriptor_t,
        linLayerMat: *mut *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNLinLayerBiasParams(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        pseudoLayer: ::std::os::raw::c_int,
        xDesc: cudnnTensorDescriptor_t,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        linLayerID: ::std::os::raw::c_int,
        linLayerBiasDesc: cudnnFilterDescriptor_t,
        linLayerBias: *mut *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNForwardInference(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNForwardTraining(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNBackwardData(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: *const cudnnTensorDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        dhyDesc: cudnnTensorDescriptor_t,
        dhy: *const ::std::os::raw::c_void,
        dcyDesc: cudnnTensorDescriptor_t,
        dcy: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        dxDesc: *const cudnnTensorDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        dhxDesc: cudnnTensorDescriptor_t,
        dhx: *mut ::std::os::raw::c_void,
        dcxDesc: cudnnTensorDescriptor_t,
        dcx: *mut ::std::os::raw::c_void,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNBackwardWeights(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        seqLength: ::std::os::raw::c_int,
        xDesc: *const cudnnTensorDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        yDesc: *const cudnnTensorDescriptor_t,
        y: *const ::std::os::raw::c_void,
        workspace: *const ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::std::os::raw::c_void,
        reserveSpace: *const ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
pub const cudnnCTCLossAlgo_t_CUDNN_CTC_LOSS_ALGO_DETERMINISTIC: cudnnCTCLossAlgo_t = 0;
pub const cudnnCTCLossAlgo_t_CUDNN_CTC_LOSS_ALGO_NON_DETERMINISTIC: cudnnCTCLossAlgo_t = 1;
pub type cudnnCTCLossAlgo_t = u32;
extern "C" {
    pub fn cudnnCreateCTCLossDescriptor(
        ctcLossDesc: *mut cudnnCTCLossDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetCTCLossDescriptor(
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        compType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetCTCLossDescriptor(
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        compType: *mut cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyCTCLossDescriptor(ctcLossDesc: cudnnCTCLossDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCTCLoss(
        handle: cudnnHandle_t,
        probsDesc: cudnnTensorDescriptor_t,
        probs: *const ::std::os::raw::c_void,
        labels: *const ::std::os::raw::c_int,
        labelLengths: *const ::std::os::raw::c_int,
        inputLengths: *const ::std::os::raw::c_int,
        costs: *mut ::std::os::raw::c_void,
        gradientsDesc: cudnnTensorDescriptor_t,
        gradients: *const ::std::os::raw::c_void,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        workspace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetCTCLossWorkspaceSize(
        handle: cudnnHandle_t,
        probsDesc: cudnnTensorDescriptor_t,
        gradientsDesc: cudnnTensorDescriptor_t,
        labels: *const ::std::os::raw::c_int,
        labelLengths: *const ::std::os::raw::c_int,
        inputLengths: *const ::std::os::raw::c_int,
        algo: cudnnCTCLossAlgo_t,
        ctcLossDesc: cudnnCTCLossDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudnnAlgorithm_t {
    pub algo: cudnnAlgorithm_t_Algorithm,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudnnAlgorithm_t_Algorithm {
    pub convFwdAlgo: cudnnConvolutionFwdAlgo_t,
    pub convBwdFilterAlgo: cudnnConvolutionBwdFilterAlgo_t,
    pub convBwdDataAlgo: cudnnConvolutionBwdDataAlgo_t,
    pub RNNAlgo: cudnnRNNAlgo_t,
    pub CTCLossAlgo: cudnnCTCLossAlgo_t,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_cudnnAlgorithm_t_Algorithm() {
    assert_eq!(
        ::std::mem::size_of::<cudnnAlgorithm_t_Algorithm>(),
        4usize,
        concat!("Size of: ", stringify!(cudnnAlgorithm_t_Algorithm))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnAlgorithm_t_Algorithm>(),
        4usize,
        concat!("Alignment of ", stringify!(cudnnAlgorithm_t_Algorithm))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnAlgorithm_t_Algorithm>())).convFwdAlgo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t_Algorithm),
            "::",
            stringify!(convFwdAlgo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnAlgorithm_t_Algorithm>())).convBwdFilterAlgo as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t_Algorithm),
            "::",
            stringify!(convBwdFilterAlgo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnAlgorithm_t_Algorithm>())).convBwdDataAlgo as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t_Algorithm),
            "::",
            stringify!(convBwdDataAlgo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnAlgorithm_t_Algorithm>())).RNNAlgo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t_Algorithm),
            "::",
            stringify!(RNNAlgo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<cudnnAlgorithm_t_Algorithm>())).CTCLossAlgo as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t_Algorithm),
            "::",
            stringify!(CTCLossAlgo)
        )
    );
}
#[test]
fn bindgen_test_layout_cudnnAlgorithm_t() {
    assert_eq!(
        ::std::mem::size_of::<cudnnAlgorithm_t>(),
        4usize,
        concat!("Size of: ", stringify!(cudnnAlgorithm_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnAlgorithm_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cudnnAlgorithm_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnAlgorithm_t>())).algo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnAlgorithm_t),
            "::",
            stringify!(algo)
        )
    );
}
extern "C" {
    pub fn cudnnCreateAlgorithmDescriptor(
        algoDesc: *mut cudnnAlgorithmDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetAlgorithmDescriptor(
        algoDesc: cudnnAlgorithmDescriptor_t,
        algorithm: cudnnAlgorithm_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetAlgorithmDescriptor(
        algoDesc: cudnnAlgorithmDescriptor_t,
        algorithm: *mut cudnnAlgorithm_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCopyAlgorithmDescriptor(
        src: cudnnAlgorithmDescriptor_t,
        dest: cudnnAlgorithmDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyAlgorithmDescriptor(algoDesc: cudnnAlgorithmDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCreateAlgorithmPerformance(
        algoPerf: *mut cudnnAlgorithmPerformance_t,
        numberToCreate: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetAlgorithmPerformance(
        algoPerf: cudnnAlgorithmPerformance_t,
        algoDesc: cudnnAlgorithmDescriptor_t,
        status: cudnnStatus_t,
        time: f32,
        memory: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetAlgorithmPerformance(
        algoPerf: cudnnAlgorithmPerformance_t,
        algoDesc: *mut cudnnAlgorithmDescriptor_t,
        status: *mut cudnnStatus_t,
        time: *mut f32,
        memory: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyAlgorithmPerformance(
        algoPerf: *mut cudnnAlgorithmPerformance_t,
        numberToDestroy: ::std::os::raw::c_int,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetAlgorithmSpaceSize(
        handle: cudnnHandle_t,
        algoDesc: cudnnAlgorithmDescriptor_t,
        algoSpaceSizeInBytes: *mut usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSaveAlgorithm(
        handle: cudnnHandle_t,
        algoDesc: cudnnAlgorithmDescriptor_t,
        algoSpace: *mut ::std::os::raw::c_void,
        algoSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRestoreAlgorithm(
        handle: cudnnHandle_t,
        algoSpace: *mut ::std::os::raw::c_void,
        algoSpaceSizeInBytes: usize,
        algoDesc: cudnnAlgorithmDescriptor_t,
    ) -> cudnnStatus_t;
}
pub const cudnnRNNClipMode_t_CUDNN_RNN_CLIP_NONE: cudnnRNNClipMode_t = 0;
pub const cudnnRNNClipMode_t_CUDNN_RNN_CLIP_MINMAX: cudnnRNNClipMode_t = 1;
pub type cudnnRNNClipMode_t = u32;
extern "C" {
    pub fn cudnnRNNSetClip(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        clipMode: cudnnRNNClipMode_t,
        clipNanOpt: cudnnNanPropagation_t,
        lclip: f64,
        rclip: f64,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNGetClip(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        clipMode: *mut cudnnRNNClipMode_t,
        clipNanOpt: *mut cudnnNanPropagation_t,
        lclip: *mut f64,
        rclip: *mut f64,
    ) -> cudnnStatus_t;
}
pub const cudnnSeverity_t_CUDNN_SEV_FATAL: cudnnSeverity_t = 0;
pub const cudnnSeverity_t_CUDNN_SEV_ERROR: cudnnSeverity_t = 1;
pub const cudnnSeverity_t_CUDNN_SEV_WARNING: cudnnSeverity_t = 2;
pub const cudnnSeverity_t_CUDNN_SEV_INFO: cudnnSeverity_t = 3;
pub type cudnnSeverity_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnDebug_t {
    pub cudnn_version: ::std::os::raw::c_uint,
    pub cudnnStatus: cudnnStatus_t,
    pub time_sec: ::std::os::raw::c_uint,
    pub time_usec: ::std::os::raw::c_uint,
    pub time_delta: ::std::os::raw::c_uint,
    pub handle: cudnnHandle_t,
    pub stream: cudaStream_t,
    pub pid: ::std::os::raw::c_ulonglong,
    pub tid: ::std::os::raw::c_ulonglong,
    pub cudaDeviceId: ::std::os::raw::c_int,
    pub reserved: [::std::os::raw::c_int; 15usize],
}
#[test]
fn bindgen_test_layout_cudnnDebug_t() {
    assert_eq!(
        ::std::mem::size_of::<cudnnDebug_t>(),
        120usize,
        concat!("Size of: ", stringify!(cudnnDebug_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cudnnDebug_t>(),
        8usize,
        concat!("Alignment of ", stringify!(cudnnDebug_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).cudnn_version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(cudnn_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).cudnnStatus as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(cudnnStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).time_sec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(time_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).time_usec as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(time_usec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).time_delta as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(time_delta)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).handle as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(handle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).stream as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(stream)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).pid as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).tid as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(tid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).cudaDeviceId as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(cudaDeviceId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cudnnDebug_t>())).reserved as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(cudnnDebug_t),
            "::",
            stringify!(reserved)
        )
    );
}
pub type cudnnCallback_t = ::std::option::Option<
    unsafe extern "C" fn(
        sev: cudnnSeverity_t,
        udata: *mut ::std::os::raw::c_void,
        dbg: *const cudnnDebug_t,
        msg: *const ::std::os::raw::c_char,
    ),
>;
extern "C" {
    pub fn cudnnSetCallback(
        mask: ::std::os::raw::c_uint,
        udata: *mut ::std::os::raw::c_void,
        fptr: cudnnCallback_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetCallback(
        mask: *mut ::std::os::raw::c_uint,
        udata: *mut *mut ::std::os::raw::c_void,
        fptr: *mut cudnnCallback_t,
    ) -> cudnnStatus_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudnnRNNDataStruct {
    _unused: [u8; 0],
}
pub type cudnnRNNDataDescriptor_t = *mut cudnnRNNDataStruct;
pub const cudnnRNNDataLayout_t_CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_UNPACKED: cudnnRNNDataLayout_t = 0;
pub const cudnnRNNDataLayout_t_CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_PACKED: cudnnRNNDataLayout_t = 1;
pub const cudnnRNNDataLayout_t_CUDNN_RNN_DATA_LAYOUT_BATCH_MAJOR_UNPACKED: cudnnRNNDataLayout_t = 2;
pub type cudnnRNNDataLayout_t = u32;
pub const cudnnRNNPaddingMode_t_CUDNN_RNN_PADDED_IO_DISABLED: cudnnRNNPaddingMode_t = 0;
pub const cudnnRNNPaddingMode_t_CUDNN_RNN_PADDED_IO_ENABLED: cudnnRNNPaddingMode_t = 1;
pub type cudnnRNNPaddingMode_t = u32;
extern "C" {
    pub fn cudnnSetRNNPaddingMode(
        rnnDesc: cudnnRNNDescriptor_t,
        paddingMode: cudnnRNNPaddingMode_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNPaddingMode(
        rnnDesc: cudnnRNNDescriptor_t,
        paddingMode: *mut cudnnRNNPaddingMode_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnCreateRNNDataDescriptor(
        RNNDataDesc: *mut cudnnRNNDataDescriptor_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnDestroyRNNDataDescriptor(RNNDataDesc: cudnnRNNDataDescriptor_t) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNDataDescriptor(
        RNNDataDesc: cudnnRNNDataDescriptor_t,
        dataType: cudnnDataType_t,
        layout: cudnnRNNDataLayout_t,
        maxSeqLength: ::std::os::raw::c_int,
        batchSize: ::std::os::raw::c_int,
        vectorSize: ::std::os::raw::c_int,
        seqLengthArray: *const ::std::os::raw::c_int,
        paddingFill: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnGetRNNDataDescriptor(
        RNNDataDesc: cudnnRNNDataDescriptor_t,
        dataType: *mut cudnnDataType_t,
        layout: *mut cudnnRNNDataLayout_t,
        maxSeqLength: *mut ::std::os::raw::c_int,
        batchSize: *mut ::std::os::raw::c_int,
        vectorSize: *mut ::std::os::raw::c_int,
        arrayLengthRequested: ::std::os::raw::c_int,
        seqLengthArray: *mut ::std::os::raw::c_int,
        paddingFill: *mut ::std::os::raw::c_void,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNForwardTrainingEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        xDesc: cudnnRNNDataDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        kDesc: cudnnRNNDataDescriptor_t,
        keys: *const ::std::os::raw::c_void,
        cDesc: cudnnRNNDataDescriptor_t,
        cAttn: *mut ::std::os::raw::c_void,
        iDesc: cudnnRNNDataDescriptor_t,
        iAttn: *mut ::std::os::raw::c_void,
        qDesc: cudnnRNNDataDescriptor_t,
        queries: *mut ::std::os::raw::c_void,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNForwardInferenceEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        xDesc: cudnnRNNDataDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *mut ::std::os::raw::c_void,
        hyDesc: cudnnTensorDescriptor_t,
        hy: *mut ::std::os::raw::c_void,
        cyDesc: cudnnTensorDescriptor_t,
        cy: *mut ::std::os::raw::c_void,
        kDesc: cudnnRNNDataDescriptor_t,
        keys: *const ::std::os::raw::c_void,
        cDesc: cudnnRNNDataDescriptor_t,
        cAttn: *mut ::std::os::raw::c_void,
        iDesc: cudnnRNNDataDescriptor_t,
        iAttn: *mut ::std::os::raw::c_void,
        qDesc: cudnnRNNDataDescriptor_t,
        queries: *mut ::std::os::raw::c_void,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNBackwardDataEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *const ::std::os::raw::c_void,
        dyDesc: cudnnRNNDataDescriptor_t,
        dy: *const ::std::os::raw::c_void,
        dcDesc: cudnnRNNDataDescriptor_t,
        dcAttn: *const ::std::os::raw::c_void,
        dhyDesc: cudnnTensorDescriptor_t,
        dhy: *const ::std::os::raw::c_void,
        dcyDesc: cudnnTensorDescriptor_t,
        dcy: *const ::std::os::raw::c_void,
        wDesc: cudnnFilterDescriptor_t,
        w: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        cxDesc: cudnnTensorDescriptor_t,
        cx: *const ::std::os::raw::c_void,
        dxDesc: cudnnRNNDataDescriptor_t,
        dx: *mut ::std::os::raw::c_void,
        dhxDesc: cudnnTensorDescriptor_t,
        dhx: *mut ::std::os::raw::c_void,
        dcxDesc: cudnnTensorDescriptor_t,
        dcx: *mut ::std::os::raw::c_void,
        dkDesc: cudnnRNNDataDescriptor_t,
        dkeys: *mut ::std::os::raw::c_void,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnRNNBackwardWeightsEx(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        xDesc: cudnnRNNDataDescriptor_t,
        x: *const ::std::os::raw::c_void,
        hxDesc: cudnnTensorDescriptor_t,
        hx: *const ::std::os::raw::c_void,
        yDesc: cudnnRNNDataDescriptor_t,
        y: *const ::std::os::raw::c_void,
        workSpace: *mut ::std::os::raw::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cudnnFilterDescriptor_t,
        dw: *mut ::std::os::raw::c_void,
        reserveSpace: *mut ::std::os::raw::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNDescriptor_v6(
        handle: cudnnHandle_t,
        rnnDesc: cudnnRNNDescriptor_t,
        hiddenSize: ::std::os::raw::c_int,
        numLayers: ::std::os::raw::c_int,
        dropoutDesc: cudnnDropoutDescriptor_t,
        inputMode: cudnnRNNInputMode_t,
        direction: cudnnDirectionMode_t,
        mode: cudnnRNNMode_t,
        algo: cudnnRNNAlgo_t,
        dataType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
extern "C" {
    pub fn cudnnSetRNNDescriptor_v5(
        rnnDesc: cudnnRNNDescriptor_t,
        hiddenSize: ::std::os::raw::c_int,
        numLayers: ::std::os::raw::c_int,
        dropoutDesc: cudnnDropoutDescriptor_t,
        inputMode: cudnnRNNInputMode_t,
        direction: cudnnDirectionMode_t,
        mode: cudnnRNNMode_t,
        dataType: cudnnDataType_t,
    ) -> cudnnStatus_t;
}
