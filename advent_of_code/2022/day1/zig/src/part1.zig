const std = @import("std");
pub fn solve(input: []u8) !i32 {
    var lines = std.mem.split(u8, input, "\n");

    var max: i32 = 0;
    var current: i32 = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            max = @max(max, current);
            current = 0;
            continue;
        }

        current += try std.fmt.parseInt(i32, line, 10);
    }

    return max;
}
