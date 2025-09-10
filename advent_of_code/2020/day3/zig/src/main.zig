const std = @import("std");
const lib = @import("root.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const input = try lib.readFile(allocator, "../input.txt");

    std.debug.print("Part 1: {}\n", .{try lib.part1(allocator, input)});
    std.debug.print("Part 2: {}\n", .{try lib.part2(allocator, input)});
}
