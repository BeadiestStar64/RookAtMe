#pragma once

#include <stdint.h>

// NVMLの初期化を行う
int32_t rook_nvml_init(void);

// NVMLのシャットダウンを行う
int32_t rook_nvml_shutdown(void);

// NVML経由でGPUの総数を取得する
int32_t rook_nvml_device_count(uint32_t* p_count);
