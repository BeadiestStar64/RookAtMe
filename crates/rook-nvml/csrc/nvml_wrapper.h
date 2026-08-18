#pragma once

#include <stdint.h>

// NVMLの初期化を行う
int32_t rook_nvml_init(void);

// NVMLのシャットダウンを行う
int32_t rook_nvml_shutdown(void);

// NVML経由でGPUの総数を取得する
int32_t rook_nvml_device_count(uint32_t* p_count);

// NVML経由のメモリ情報を扱う構造体
typedef struct rook_nvml_device_memory_st {
  uint64_t free;
  uint64_t reserved;
  uint64_t total;
  uint64_t used;
} rook_nvml_memory_t;

// 指定されたindexのGPUからVRAM情報を取得する
int32_t rook_nvml_device_memory(uint32_t index, rook_nvml_memory_t* p_memory);
