#pragma once

#include <stddef.h>
#define DEFINE_ARRAYLIST(T, name)                                                \
    typedef struct {                                                             \
        T* data;                                                                 \
        size_t size;                                                             \
        size_t capacity;                                                         \
    } array_##name##_t;                                                          \
                                                                                 \
    array_##name##_t array_##name##_new(size_t capacity) {                       \
        array_##name##_t array;                                                  \
        array.data     = (T*)malloc(capacity * sizeof(T));                       \
        array.capacity = capacity;                                               \
        array.size     = 0;                                                      \
        return array;                                                            \
    }                                                                            \
                                                                                 \
    void array_##name##_free(array_##name##_t* array) {                          \
        free(array->data);                                                       \
    }                                                                            \
                                                                                 \
    void array_##name##_push(array_##name##_t* array, T value) {                 \
        if (array->size == array->capacity) {                                    \
            array->capacity *= 2;                                                \
            array->data = (T*)realloc(array->data, array->capacity * sizeof(T)); \
        }                                                                        \
        array->data[array->size] = value;                                        \
        array->size++;                                                           \
    }                                                                            \
                                                                                 \
    T array_##name##_pop(array_##name##_t* array) {                              \
        array->size--;                                                           \
        return array->data[array->size];                                         \
    }                                                                            \
    T array_##name##_peek(array_##name##_t* array) {                             \
        return array->data[array->size - 1];                                     \
    }                                                                            \
    size_t array_##name##_size(array_##name##_t* array) {                        \
        return array->size;                                                      \
    }                                                                            \
    void array_##name##_clear(array_##name##_t* array) {                         \
        array->size = 0;                                                         \
    }                                                                            \
    T* array_##name##_data(array_##name##_t* array) {                            \
        return array->data;                                                      \
    }                                                                            \
    T array_##name##_at(array_##name##_t* array, size_t index) {                 \
        return array->data[index];                                               \
    }                                                                            \
    T array_##name##_last(array_##name##_t* array) {                             \
        return array->data[array->size - 1];                                     \
    }
