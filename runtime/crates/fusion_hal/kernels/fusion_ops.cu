/**
 * @file fusion_ops.cu
 * @brief CUDA kernels for Fusion Runtime operations
 * 
 * This file contains optimized CUDA kernels for matrix operations
 * and other GPU-accelerated computations used by the Fusion Runtime.
 */

#include <cuda_runtime.h>

/**
 * @brief Single-precision general matrix multiply (SGEMM) kernel
 * 
 * Computes C = A × B where A, B, C are N×N matrices.
 * Uses tile-based multiplication with 16×16 thread blocks.
 * 
 * @param A Pointer to matrix A (row-major, N×N)
 * @param B Pointer to matrix B (row-major, N×N)
 * @param C Pointer to output matrix C (row-major, N×N)
 * @param N Matrix dimension
 */
__global__ void sgemm_kernel(const float* A, const float* B, float* C, int N) {
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    
    if (row < N && col < N) {
        float sum = 0.0f;
        for (int k = 0; k < N; ++k) {
            sum += A[row * N + k] * B[k * N + col];
        }
        C[row * N + col] = sum;
    }
}

/**
 * @brief Element-wise vector addition kernel
 * 
 * Computes C = A + B where A, B, C are vectors of length N.
 * 
 * @param A Pointer to vector A
 * @param B Pointer to vector B
 * @param C Pointer to output vector C
 * @param N Vector length
 */
__global__ void vector_add_kernel(const float* A, const float* B, float* C, int N) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < N) {
        C[idx] = A[idx] + B[idx];
    }
}

/**
 * @brief ReLU activation kernel
 * 
 * Applies ReLU activation: output[i] = max(0, input[i])
 * 
 * @param input Pointer to input tensor
 * @param output Pointer to output tensor
 * @param N Tensor size
 */
__global__ void relu_kernel(const float* input, float* output, int N) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < N) {
        output[idx] = fmaxf(0.0f, input[idx]);
    }
}

/**
 * @brief Softmax kernel (numerically stable)
 * 
 * Computes softmax over the last dimension.
 * Uses log-sum-exp trick for numerical stability.
 * 
 * @param input Pointer to input tensor
 * @param output Pointer to output tensor
 * @param batch_size Number of batches
 * @param dim Dimension size
 */
__global__ void softmax_kernel(const float* input, float* output, int batch_size, int dim) {
    int batch = blockIdx.x;
    
    if (batch < batch_size) {
        // Find max for numerical stability
        float max_val = -INFINITY;
        for (int i = 0; i < dim; ++i) {
            max_val = fmaxf(max_val, input[batch * dim + i]);
        }
        
        // Compute exp sum
        float exp_sum = 0.0f;
        for (int i = 0; i < dim; ++i) {
            exp_sum += expf(input[batch * dim + i] - max_val);
        }
        
        // Normalize
        for (int i = 0; i < dim; ++i) {
            output[batch * dim + i] = expf(input[batch * dim + i] - max_val) / exp_sum;
        }
    }
}

// C interface for Rust FFI
extern "C" {
    /**
     * @brief Launch SGEMM kernel
     * 
     * @param A Device pointer to matrix A
     * @param B Device pointer to matrix B
     * @param C Device pointer to output matrix C
     * @param N Matrix dimension
     * @param stream CUDA stream for async execution
     * @return cudaError_t return code (0 = success)
     */
    int launch_sgemm(const float* A, const float* B, float* C, int N, void* stream) {
        dim3 threads(16, 16);
        dim3 blocks((N + 15) / 16, (N + 15) / 16);
        sgemm_kernel<<<blocks, threads, 0, (cudaStream_t)stream>>>(A, B, C, N);
        return cudaGetLastError();
    }
    
    /**
     * @brief Launch vector addition kernel
     */
    int launch_vector_add(const float* A, const float* B, float* C, int N, void* stream) {
        int threads = 256;
        int blocks = (N + threads - 1) / threads;
        vector_add_kernel<<<blocks, threads, 0, (cudaStream_t)stream>>>(A, B, C, N);
        return cudaGetLastError();
    }
    
    /**
     * @brief Launch ReLU activation kernel
     */
    int launch_relu(const float* input, float* output, int N, void* stream) {
        int threads = 256;
        int blocks = (N + threads - 1) / threads;
        relu_kernel<<<blocks, threads, 0, (cudaStream_t)stream>>>(input, output, N);
        return cudaGetLastError();
    }
    
    /**
     * @brief Launch softmax kernel
     */
    int launch_softmax(const float* input, float* output, int batch_size, int dim, void* stream) {
        softmax_kernel<<<batch_size, 1, 0, (cudaStream_t)stream>>>(input, output, batch_size, dim);
        return cudaGetLastError();
    }
}
