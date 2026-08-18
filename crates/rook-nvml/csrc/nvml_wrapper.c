#include <nvml.h>
#include <stddef.h>

#include "nvml_wrapper.h"

// nvmlを初期化する
int32_t rook_nvml_init(void) {
  return (int32_t)nvmlInit_v2();
}

// nvmlをシャットダウンする
int32_t rook_nvml_shutdown(void) {
  return (int32_t)nvmlShutdown();
}

// GPUの総数を取得する
int32_t rook_nvml_device_count(uint32_t* p_count) {
  return (int32_t)nvmlDeviceGetCount_v2(p_count);
}

// VRAM情報を取得する
int32_t rook_nvml_device_memory(uint32_t index, rook_nvml_memory_t* p_memory) {
  // ヌルポインターを許容しない
  if (p_memory == NULL) {
    return (int32_t)NVML_ERROR_INVALID_ARGUMENT;
  }

  // NVMLデバイスハンドルの格納先
  nvmlDevice_t device;

  // indexのGPUを参照するためのハンドルを取得
  nvmlReturn_t result = nvmlDeviceGetHandleByIndex_v2(index, &device);

  if (result != NVML_SUCCESS) {
    return (int32_t)result;
  }

  // nvmlMemory_v2_tの各メンバをゼロ初期化
  nvmlMemory_v2_t nvml_memory = {0};

  // nvmMemory_v2_tの構造体バージョンを指定
  nvml_memory.version = nvmlMemory_v2;

  // VRAM情報を取得する
  result = nvmlDeviceGetMemoryInfo_v2(device, &nvml_memory);

  if (result != NVML_SUCCESS) {
    return (int32_t)result;
  }

  // rook_nvml_memory構造体にデータを書き込む
  p_memory->free = nvml_memory.free;
  p_memory->reserved = nvml_memory.reserved;
  p_memory->total = nvml_memory.total;
  p_memory->used = nvml_memory.used;

  return (int32_t)NVML_SUCCESS;
}
