const std = @import("std");

const Point = struct {
    x: usize,
    y: usize,
};

const KEYPAD1 = [_][3]u8{
    [_]u8{ '1', '2', '3' },
    [_]u8{ '4', '5', '6' },
    [_]u8{ '7', '8', '9' },
};

const KEYPAD2 = [_][5]u8{
    [_]u8{ ' ', ' ', '1', ' ', ' ' },
    [_]u8{ ' ', '2', '3', '4', ' ' },
    [_]u8{ '5', '6', '7', '8', '9' },
    [_]u8{ ' ', 'A', 'B', 'C', ' ' },
    [_]u8{ ' ', ' ', 'D', ' ', ' ' },
};

fn part1(input: []const u8) ![]const u8 {
    const allocator = std.heap.page_allocator;
    var ret = std.ArrayList(u8).init(allocator);
    defer ret.deinit();

    var lines = std.mem.splitSequence(u8, input, "\n");

    var pos = Point{ .x = 1, .y = 1 };

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        for (line) |c| {
            switch (c) {
                'U' => pos.y -|= 1,
                'D' => pos.y = @min(pos.y + 1, KEYPAD1.len - 1),
                'L' => pos.x -|= 1,
                'R' => pos.x = @min(pos.x + 1, KEYPAD1[0].len - 1),
                else => unreachable,
            }
        }

        ret.append(KEYPAD1[pos.y][pos.x]) catch unreachable;
    }

    return ret.toOwnedSlice();
}

fn part2(input: []const u8) ![]const u8 {
    const allocator = std.heap.page_allocator;
    var ret = std.ArrayList(u8).init(allocator);
    defer ret.deinit();

    var lines = std.mem.splitSequence(u8, input, "\n");
    var pos = Point{ .x = 0, .y = 2 };

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        for (line) |c| {
            const prev_pos = pos;
            switch (c) {
                'U' => pos.y -|= 1,
                'D' => pos.y = @min(pos.y + 1, KEYPAD2.len - 1),
                'L' => pos.x -|= 1,
                'R' => pos.x = @min(pos.x + 1, KEYPAD2[0].len - 1),
                else => unreachable,
            }

            if (KEYPAD2[pos.y][pos.x] == ' ') {
                pos = prev_pos;
            }
        }

        ret.append(KEYPAD2[pos.y][pos.x]) catch unreachable;
    }

    return ret.toOwnedSlice();
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("../sample.txt", .{});
    defer file.close();

    var reader = file.reader();

    var buf: [10000]u8 = undefined;
    const bytes_read = try reader.read(&buf);

    const ans1 = try part1(buf[0..bytes_read]);
    std.debug.print("Part 1: {s}\n", .{ans1});

    const ans2 = try part2(buf[0..bytes_read]);
    std.debug.print("Part 2: {s}\n", .{ans2});
}
