const std = @import("std");

pub const Map = struct {
    width: usize,
    height: usize,
    map: []u8,

    fn get(self: Map, x: usize, y: usize) u8 {
        const corr_x = x % self.width;
        const corr_y = y % self.height;
        return self.map[corr_y * self.width + corr_x];
    }
};

pub fn readFile(gpa: std.mem.Allocator, path: []const u8) ![]const u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    const contents = try file.readToEndAlloc(gpa, std.math.maxInt(usize));
    return contents;
}

pub fn parseInput(gpa: std.mem.Allocator, input: []const u8) !Map {
    var line_it = std.mem.splitSequence(u8, input, "\n");
    const first_line = line_it.next().?;
    const width = first_line.len;

    var map = std.ArrayList(u8).init(gpa);
    defer map.deinit();

    try map.appendSlice(first_line);

    var height: usize = 0;
    while (line_it.next()) |line| : (height += 1) {
        try map.appendSlice(line);
    }

    return Map{ .width = width, .height = height, .map = try map.toOwnedSlice() };
}

pub fn countTreesForSlope(map: *const Map, slope: [2]i32) !usize {
    var x: usize = 0;
    var y: usize = 0;

    var count: usize = 0;

    while (y < map.height) {
        if (map.get(x, y) == '#') {
            count += 1;
        }
        x = (x + @as(usize, @intCast(slope[0]))) % map.width;
        y += @intCast(slope[1]);
    }

    return count;
}

pub fn part1(gpa: std.mem.Allocator, input: []const u8) !usize {
    const map = try parseInput(gpa, input);

    const slope = [_]i32{ 3, 1 };

    return countTreesForSlope(&map, slope);
}

pub fn part2(gpa: std.mem.Allocator, input: []const u8) !usize {
    const map = try parseInput(gpa, input);

    const slopes = [_][2]i32{
        .{ 1, 1 },
        .{ 3, 1 },
        .{ 5, 1 },
        .{ 7, 1 },
        .{ 1, 2 },
    };

    var product: usize = 1;

    for (slopes) |slope| {
        product *= try countTreesForSlope(&map, slope);
    }

    return product;
}
