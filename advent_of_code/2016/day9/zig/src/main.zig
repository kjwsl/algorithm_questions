const std = @import("std");

fn decompress(allocator: std.mem.Allocator, input:[]const u8, recursive: bool) ![]const u8 {
    var new_str = std.ArrayList(u8).init(allocator);

    var i: usize = 0;
    while (i < input.len) {
        if (input[i] == '(') {
            const start_idx: usize = i + 1;
            var end_idx: usize = start_idx;
            while (end_idx < input.len and input[end_idx] != ')') {
                end_idx += 1;
            }

            const marker = input[start_idx..end_idx];

            var x_idx: usize = 0;
            while (x_idx < marker.len and marker[x_idx] != 'x') {
                x_idx += 1;
            }

            const subsequent = std.fmt.parseInt(usize, marker[0..x_idx], 10) catch unreachable;
            const repeat = std.fmt.parseInt(usize, marker[x_idx + 1 .. marker.len], 10) catch unreachable;

            const sub_str = input[end_idx + 1 .. end_idx + 1 + subsequent];

            if (recursive) {
                const expanded = try decompress(allocator, sub_str, recursive);
                defer allocator.free(expanded);

                for (0..repeat) |_| {
                    try new_str.appendSlice(expanded);
                }
            } else {
                for (0..repeat) |_| {
                    try new_str.appendSlice(sub_str);
                }
            }


            i = end_idx + subsequent + 1;
        } else {
            try new_str.append(input[i]);
            i += 1;
        }
    }

    return try new_str.toOwnedSlice();
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !usize {
    const new_str = try decompress(allocator, input, false);
    defer allocator.free(new_str);

    return new_str.len;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !usize {
    const new_str = try decompress(allocator, input, true);
    defer allocator.free(new_str);

    return new_str.len;
}


pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try std.fs.cwd().readFileAlloc(allocator, "../input.txt", 1024 * 1024);
    defer allocator.free(input);

    std.debug.print("Part 1: {}\n", .{try part1(allocator, input)});
    std.debug.print("Part 2: {}\n", .{try part2(allocator, input)});
}
