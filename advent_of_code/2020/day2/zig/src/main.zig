const std = @import("std");
const zig = @import("zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = try std.fs.cwd().readFileAlloc(allocator, "../input.txt", std.math.maxInt(usize));
    defer allocator.free(input);

    try zig.print(1024, "Part 1: {}\n", .{try zig.part1(allocator, input)});
    try zig.print(1024, "Part 2: {}\n", .{try zig.part2(allocator, input)});
}
