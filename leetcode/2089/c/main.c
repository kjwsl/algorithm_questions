/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

#include <stdio.h>
#include <stdlib.h>

void swap(int* a, int* b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int _partition(int* arr, int arr_size, int lo, int hi) {
    int pivot = arr[hi - 1];

    if (arr_size <= 1 || lo + 1 >= hi) {
        return pivot;
    }
    
    int j = lo;

    for (int i = lo; i < hi - 1; i++) {
        if (arr[i] < pivot) {
            swap(&arr[i], &arr[j]);
            j++;
        }
    }
    swap(&arr[j], &arr[hi - 1]);

    return j;
}

void _qsort(int* arr, int arr_size, int lo, int hi) {
    if (lo + 1 >= hi) {
        return;
    }

    int pivot_idx = _partition(arr, arr_size, lo, hi);
    _qsort(arr, arr_size, lo, pivot_idx);
    _qsort(arr, arr_size, pivot_idx + 1, hi);
}

void my_qsort(int* arr, int arr_size) {
    if (arr_size <= 1) {
        return;
    }
    _qsort(arr, arr_size, 0, arr_size);
}

int* find_all(int* arr, int arr_size, int target, int* count) {
    int start = -1;
    *count = 0;
    for (int i = 0; i < arr_size; i++) {
        if (start == -1 && arr[i] == target) {
            start = i;
        }
        if (arr[i] == target) {
            (*count)++;
        }
    }

    int* idxs = calloc(*count, sizeof(int));

    for (int i = 0; i < *count; i++) {
        idxs[i] = start + i;
    }

    return idxs;
}

int* targetIndices(int* nums, int numsSize, int target, int* returnSize) {
    my_qsort(nums, numsSize);
    return find_all(nums, numsSize, target, returnSize);
}


void print_arr(int* arr, int arr_size) {
    printf("[");
    for (int i = 0; i < arr_size; i++) {
        printf("%d", arr[i]);
        if (i < arr_size - 1) {
            printf(", ");
        }
    }
    printf("]");
}

int main() {
    int arr[] = { 1, 2, 5, 2, 3 };
    int arr_size = sizeof(arr) / sizeof(int);
    my_qsort(arr, arr_size);
    print_arr(arr, arr_size);
}
