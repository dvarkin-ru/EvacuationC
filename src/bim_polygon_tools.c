/* Copyright © 2021 bvchirkov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "bim_polygon_tools.h"
#include "triangle.h"

double geom_tools_length_side(const point_t *const p1, const point_t *const p2)
{
    return sqrt(pow(p1->x - p2->x, 2) + pow(p1->y - p2->y, 2));
}

double geom_tools_area_polygon(const polygon_t *const polygon)
{
    // https://ru.wikipedia.org/wiki/Формула_площади_Гаусса
    size_t n = polygon->numofpoints;
    double sum = polygon->points[n].x * polygon->points[0].y - polygon->points[0].x * polygon->points[n].y;
    for (size_t i = 0; i < n; i++) {
      sum += polygon->points[i].x * polygon->points[i+1].y - polygon->points[i+1].x * polygon->points[i].y;
    }
    return 0.5*fabs(sum);
}

uint8_t _on_segment(const point_t *const a, const point_t *const b, const point_t *const c) {
    if (geom_tools_length_side(a, b) + geom_tools_length_side(b, c) == geom_tools_length_side(a, c)) return 1;
    return 0;
}

uint8_t geom_tools_is_point_in_polygon(const point_t *const point, const polygon_t *const polygon)
{
    // https://web.archive.org/web/20161108113341/https://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html
    uint8_t c = 0;
    for (size_t i = 0, j = polygon->numofpoints-1; i < polygon->numofpoints; j = i++) {
      if (_on_segment(&polygon->points[i], point, &polygon->points[i+1])) return 1;
      if ( ((polygon->points[i].y > point->y) != (polygon->points[j].y>point->y)) &&
	   (point->x < (polygon->points[j].x-polygon->points[i].x) * (point->y-polygon->points[i].y) / (polygon->points[j].y-polygon->points[i].y) + polygon->points[i].x) )
         c = !c;
    }
    return c;
}

// signed area of a triangle
static double _area(const point_t *p1, const point_t *p2, const point_t *p3)
{
    return (p2->x - p1->x) * (p3->y - p1->y) - (p2->y - p1->y) * (p3->x - p1->x);
}

static void _fswap(double *v1, double *v2)
{
    double tmp_v1 = *v1;
    *v1 = *v2;
    *v2 = tmp_v1;
}

// https://e-maxx.ru/algo/segments_intersection_checking
static uint8_t _intersect_1(double a, double b, double c, double d)
{
    if (a > b) _fswap(&a, &b);
    if (c > d) _fswap(&c, &d);
    return fmax(a, c) <= fmin(b, d);
}

// check if two segments intersect
uint8_t geom_tools_is_intersect_line(const line_t *const l1, const line_t *const l2)
{
    const point_t *p1 = l1->p1;
    const point_t *p2 = l1->p2;
    const point_t *p3 = l2->p1;
    const point_t *p4 = l2->p2;
    return _intersect_1(p1->x, p2->x, p3->x, p4->x)
        && _intersect_1(p1->y, p2->y, p3->y, p4->y)
        && _area(p1, p2, p3) * _area(p1, p2, p4) <= 0
        && _area(p3, p4, p1) * _area(p3, p4, p2) <= 0;
}

// Определение точки на линии, расстояние до которой от заданной точки является минимальным из существующих
point_t *geom_tools_nearest_point(const point_t *const point_start, const line_t *const line)
{
    point_t a = {line->p1->x, line->p1->y};
    point_t b = {line->p2->x, line->p2->y};

    if (geom_tools_length_side(&a, &b) < 1e-9)
    {
        return line->p1;
    }

    double A = point_start->x - a.x;
    double B = point_start->y - a.y;
    double C = b.x - a.x;
    double D = b.y - a.y;

    double dot = A * C + B * D;
    double len_sq = C * C + D * D;
    double param = -1;

    if (len_sq != 0)
    {
        param = dot / len_sq;
    }

    double xx, yy;

    if (param < 0)
    {
        xx = a.x;
        yy = a.y;
    } else if (param > 1)
    {
        xx = b.x;
        yy = b.y;
    } else
    {
        xx = a.x + param * C;
        yy = a.y + param * D;
    }

    point_t *point_end = NULL;
    point_end = (point_t *) malloc(sizeof (point_t));
    point_end->x = xx;
    point_end->y = yy;
    return point_end;
}

