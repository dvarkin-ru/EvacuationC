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

typedef struct bim_json_object_t_rust {
  struct bim_json_address_t_rust *address;
} bim_json_object_t_rust;

const struct bim_json_object_t_rust *bim_json_new_rust(const char *path_to_file);
