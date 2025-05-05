const std = @import("std");

pub fn part1(input: []u8) !i32 {
    var dsum: i32 = 0;
    const allocator = std.heap.page_allocator;
    var left = std.ArrayList(i32).init(allocator);
    defer left.deinit();

    var right = std.ArrayList(i32).init(allocator);
    defer right.deinit();

    var lines = std.mem.splitSequence(u8, input, "\n");

    while (lines.next()) |line| {
        var tokens = std.mem.splitSequence(u8, line, "   ");
        var token = tokens.next().?;
        if (token.len == 0) {
            break;
        }
        std.debug.print("Token: {s}\n", .{token});
        try left.append(try std.fmt.parseInt(i32, token, 10));
        token = tokens.next().?;
        std.debug.print("Token: {s}\n", .{token});
        try right.append(try std.fmt.parseInt(i32, token, 10));
    }

    std.mem.sort(i32, left.items[0..], {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right.items[0..], {}, comptime std.sort.asc(i32));

    for (left.items, 0..) |_, i| {
        const d: u32 = @abs(left.items[i] - right.items[i]);
        dsum += @intCast(d);
    }

    return dsum;
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const input = try std.fs.cwd().openFile("sample.txt", .{});
    defer input.close();

    const input_size = try input.getEndPos();
    const input_buffer = try allocator.alloc(u8, input_size);
    defer allocator.free(input_buffer);

    _ = try input.readAll(input_buffer);

    const result = try part1(input_buffer);
    std.debug.print("Result: {}\n", .{result});
}

