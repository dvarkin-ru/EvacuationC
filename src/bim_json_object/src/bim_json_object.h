#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct bim_json_address_t_rust {
  const char *city;
  const char *street_address;
  const char *add_info;
} bim_json_address_t_rust;

typedef struct bim_json_element_t_rust {
  const char *name;
} bim_json_element_t_rust;

typedef struct bim_json_level_t_rust {
  const char *name;
  const struct bim_json_element_t_rust *elements;
  double z_level;
  unsigned long long numofelements;
} bim_json_level_t_rust;

typedef struct bim_json_object_t_rust {
  struct bim_json_address_t_rust *address;
  const char *name;
  struct bim_json_level_t_rust *levels;
  unsigned long long numoflevels;
} bim_json_object_t_rust;

const struct bim_json_object_t_rust *bim_json_new_rust(const char *path_to_file);
