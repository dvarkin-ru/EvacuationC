#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum distribution_type_rust {
  distribution_from_bim_rust,
  distribution_uniform_rust,
} distribution_type_rust;

typedef enum transits_width_type_rust {
  transits_width_from_bim_rust,
  transits_width_users_rust,
} transits_width_type_rust;

typedef struct uuid_t_rust
{
    const char x[UUID_SIZE];
} uuid_t_rust;

typedef struct bim_cfg_file_name_t_rust {
  char x[256];
} bim_cfg_file_name_t_rust;

typedef struct special_t_rust {
  struct uuid_t_rust *uuid;
  unsigned char num_of_uuids;
  float value;
} special_t_rust;

typedef struct bim_cfg_distribution_t_rust {
  enum distribution_type_rust type;
  float density;
  struct special_t_rust *special;
  unsigned char num_of_special_blocks;
} bim_cfg_distribution_t_rust;

typedef struct bim_cfg_transitions_width_t_rust {
  enum transits_width_type_rust type;
  float doorwayin;
  float doorwayout;
  struct special_t_rust *special;
  unsigned char num_of_special_blocks;
} bim_cfg_transitions_width_t_rust;

typedef struct bim_cfg_scenario_t_rust {
  struct bim_cfg_file_name_t_rust *bim_jsons;
  struct bim_cfg_file_name_t_rust logger_configure;
  unsigned char num_of_bim_jsons;
  struct bim_cfg_distribution_t_rust distribution;
  struct bim_cfg_transitions_width_t_rust transits;
} bim_cfg_scenario_t_rust;

const struct bim_cfg_scenario_t_rust *bim_cfg_load_rust(const char *path_to_file);
