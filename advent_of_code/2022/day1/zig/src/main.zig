const std = @import("std");
const part1 = @import("part1.zig");
const part2 = @import("part2.zig");

pub fn main() !void {
    const input_file = try std.fs.cwd().openFile("src/input.txt", .{});
    defer input_file.close();

    var result: i32 = 0;
    const input = try input_file.readToEndAlloc(std.heap.page_allocator, 1024 * 1024);
    result = try part1.solve(input);

    std.debug.print("Part1: {!}\n", .{result});

    result = try part2.solve(input);

    std.debug.print("Part2: {!}\n", .{result});
}
