#include "arraylist.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char direction;
    int distance;
} instruction_t;

DEFINE_ARRAYLIST(instruction_t, instruction)

array_instruction_t parse_input(char input[], int len) {
    int i                            = 0;

    array_instruction_t instructions = array_instruction_new(100);
    while (i < len) {
        char direction = input[i];

        int comma_idx  = ++i;

        while (comma_idx < len && input[comma_idx] != ',') {
            comma_idx++;
        }

        char buffer[10];
        strncpy(buffer, input + i, comma_idx - i);
        buffer[comma_idx - i] = '\0';

        int distance          = atoi(buffer);
        if (distance == 0) {
            fprintf(stderr, "Invalid distance: %s\n", buffer);
            exit(1);
        }

        instruction_t instruction = {
            .direction = direction,
            .distance  = distance,
        };

        array_instruction_push(&instructions, instruction);

        i = comma_idx + 2;
    }

    return instructions;
}

int part1(char input[], int len) {
    array_instruction_t instructions = parse_input(input, len);

    int x                            = 0;
    int y                            = 0;
    int current_direction            = 0;

    for (int i = 0; i < instructions.size; i++) {
        instruction_t instruction = array_instruction_at(&instructions, i);

        switch (instruction.direction) {
        case 'L':
            current_direction = (current_direction + 3) % 4;
            break;
        case 'R':
            current_direction = (current_direction + 1) % 4;
            break;
        default:
            fprintf(stderr, "Unknown direction: %c\n", instruction.direction);
            exit(1);
        }

        switch (current_direction) {
        case 0: // North
            y += instruction.distance;
            break;
        case 1: // East
            x += instruction.distance;
            break;
        case 2: // South
            y -= instruction.distance;
            break;
        case 3: // West
            x -= instruction.distance;
            break;
        default:
            fprintf(stderr, "Unknown direction: %c\n", instruction.direction);
            exit(1);
        }
    }

    array_instruction_free(&instructions);

    return abs(x) + abs(y);
}

int main() {
    FILE* f = fopen("../sample.txt", "r");
    if (f == NULL) {
        fprintf(stderr, "Failed to open file\n");
        goto error;
    }

    char buffer[1024];
    size_t len        = 0;
    size_t bytes_read = fread(buffer, sizeof(char), sizeof(buffer), f);
    if (bytes_read == 0) {
        fprintf(stderr, "Failed to read file\n");
        goto error_fopen;
    }
    buffer[bytes_read] = '\0';

    int ans1           = part1(buffer, bytes_read);

    printf("Part 1: %d\n", ans1);

    fclose(f);
    return 0;

error_fopen:
    fclose(f);
error:
    return 1;
}
