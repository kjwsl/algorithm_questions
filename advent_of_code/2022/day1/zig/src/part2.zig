const std = @import("std");
pub fn solve(input: []u8) !i32 {
    var lines = std.mem.split(u8, input, "\n");

    var list = std.ArrayList(i32).init(std.heap.page_allocator);

    var max: i32 = 0;
    var current: i32 = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            max = @max(max, current);
            try list.append(current);
            current = 0;
            continue;
        }

        current += try std.fmt.parseInt(i32, line, 10);
    }

    std.mem.sort(i32, list.items, {}, comptime std.sort.desc(i32));

    const top_list = list.items[0..3];

    return top_list[0] + top_list[1] + top_list[2];
}
