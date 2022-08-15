#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct bim_cfg_file_name_t_rust {
  char x[256];
} bim_cfg_file_name_t_rust;

typedef struct bim_cfg_scenario_t_rust {
  struct bim_cfg_file_name_t_rust *bim_jsons;
  struct bim_cfg_file_name_t_rust logger_configure;
} bim_cfg_scenario_t_rust;

const struct bim_cfg_scenario_t_rust *bim_cfg_load_rust(const char *path_to_file);
