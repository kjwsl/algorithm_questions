const std = @import("std");

const Triangle = struct {
    sides: [3]u32,

    pub fn init(a: u32, b: u32, c: u32) Triangle {
        return Triangle{ .sides = [3]u32{ a, b, c } };
    }

    pub fn isValid(self: Triangle) bool {
        return self.sides[0] + self.sides[1] > self.sides[2] and
            self.sides[1] + self.sides[2] > self.sides[0] and
            self.sides[2] + self.sides[0] > self.sides[1];
    }
};

fn swap(comptime T: type, a: *T, b: *T) void {
    const tmp = a.*;
    a.* = b.*;
    b.* = tmp;
}
fn parse_input(input: []const u8) ![]Triangle {
    var triangles = std.ArrayList(Triangle).init(std.heap.page_allocator);
    defer triangles.deinit();

    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var sides_iter = std.mem.splitSequence(u8, line, " ");
        var sides: [3]u32 = undefined;

        var i: usize = 0;

        while (sides_iter.next()) |side| {
            if (i == 3) {
                break;
            }

            if (side.len == 0) {
                continue;
            }

            sides[i] = try std.fmt.parseInt(u32, side, 10);
            i += 1;
        }

        try triangles.append(Triangle.init(sides[0], sides[1], sides[2]));
    }

    return triangles.toOwnedSlice();
}

fn parse_input2(input: []const u8) ![]Triangle {
    var triangles = try parse_input(input);

    var i: usize = 0;

    while (i < triangles.len) {
        const triangle_group = &triangles[i..i + 3];
        swap(u32, &triangle_group.*[0].sides[1], &triangle_group.*[1].sides[0]);
        swap(u32, &triangle_group.*[0].sides[2], &triangle_group.*[2].sides[0]);
        swap(u32, &triangle_group.*[1].sides[2], &triangle_group.*[2].sides[1]);

        i += 3;
    }

    return triangles;

}

fn part1(input: []const u8) !u32 {
    const triangles = try parse_input(input);

    var count: u32 = 0;

    for (triangles) |triangle| {
        if (triangle.isValid()) {
            count += 1;
        }
    }

    return count;
}

fn part2(input: []const u8) !u32 {
    const triangles = try parse_input2(input);

    var count: u32 = 0;

    for (triangles) |triangle| {
        if (triangle.isValid()) {
            count += 1;
        }
    }

    return count;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file = try std.fs.cwd().openFile("../input.txt", .{});
    defer file.close();

    const input = try file.reader().readAllAlloc(allocator, 1024 * 1024);
    defer allocator.free(input);

    const ans1 = try part1(input);
    std.debug.print("Part 1: {d}\n", .{ans1});

    const ans2 = try part2(input);
    std.debug.print("Part 2: {d}\n", .{ans2});


}
