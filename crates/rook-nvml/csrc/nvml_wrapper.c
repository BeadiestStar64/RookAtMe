#include <nvml.h>

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
