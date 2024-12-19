const std = @import("std");
const part1 = @import("part1.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    const file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();
    const input_str = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(input_str);

    const ans = part1.solve(input_str);
    std.debug.print("Part 1: {}\n", .{ans});
}
