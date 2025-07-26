const std = @import("std");

const Instruction = struct { direction: u8, distance: i32 };

const Point = struct { x: i32, y: i32 };

const Increment = struct {
    ptr: *i32,
    parity: i8,

    fn init(ptr: *i32, parity: i8) Increment {
        return Increment{ .ptr = ptr, .parity = parity };
    }
};

fn parse_input(allocator: std.mem.Allocator, input: []const u8) ![]const Instruction {
    var i: usize = 0;
    var result = std.ArrayList(Instruction).init(allocator);

    while (i < input.len) {
        const direction = input[i];
        i += 1;
        var comma_idx = i;

        while (comma_idx < input.len and input[comma_idx] != ',') {
            comma_idx += 1;
        }

        const distance = std.fmt.parseInt(i32, input[i..comma_idx], 10) catch |err| {
            if (err == error.InvalidCharacter) {
                std.debug.print("Invalid Character: {s}\n", .{input[i..comma_idx]});
                return err;
            }
            std.debug.print("Invalid Number: {s}\n", .{input[i..comma_idx]});
            return err;
        };
        i = comma_idx + 2;
        try result.append(Instruction{ .direction = direction, .distance = distance });
    }

    return result.toOwnedSlice();
}

fn part1(sample: []const u8) !i32 {
    const allocator = std.heap.page_allocator;
    const instructions = try parse_input(allocator, sample);
    defer allocator.free(instructions);

    var pos = Point{ .x = 0, .y = 0 };

    var current_direction: usize = 0;

    for (instructions) |instruction| {
        current_direction = switch (instruction.direction) {
            'R' => (current_direction + 1) % 4,
            'L' => (current_direction + 3) % 4,
            else => current_direction,
        };

        switch (current_direction) {
            0 => pos.y += instruction.distance,
            1 => pos.x += instruction.distance,
            2 => pos.y -= instruction.distance,
            3 => pos.x -= instruction.distance,
            else => unreachable,
        }
    }

    return @intCast(@abs(pos.x) + @abs(pos.y));
}

fn part2(sample: []const u8) !i32 {
    const allocator = std.heap.page_allocator;
    const instructions = try parse_input(allocator, sample);
    defer allocator.free(instructions);

    var pos = Point{ .x = 0, .y = 0 };
    var map = std.AutoHashMap(Point, bool).init(allocator);
    defer map.deinit();

    var current_direction: usize = 0;

    for (instructions) |inst| {
        current_direction = switch (inst.direction) {
            'L' => (current_direction + 3) % 4,
            'R' => (current_direction + 1) % 4,
            else => unreachable,
        };

        const distance: usize = @intCast(inst.distance);

        const incr: Increment = switch (current_direction) {
            0 => Increment.init(&pos.y, 1),
            1 => Increment.init(&pos.x, 1),
            2 => Increment.init(&pos.y, -1),
            3 => Increment.init(&pos.x, -1),
            else => unreachable,
        };

        for (0..distance) |_| {
            const key = Point{ .x = pos.x, .y = pos.y };
            if (map.contains(key)) {
                return @intCast(@abs(pos.x) + @abs(pos.y));
            }
            try map.put(key, true);
            incr.ptr.* += incr.parity;
        }
    }

    unreachable;
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("../sample.txt", .{});
    defer file.close();

    var buf: [1024]u8 = undefined;
    var reader = file.reader();

    const len = try reader.read(&buf);

    const answer1 = try part1(buf[0 .. len - 1]);
    std.debug.print("Part 1: {d}\n", .{answer1});

    const answer2 = try part2(buf[0 .. len - 1]);
    std.debug.print("Part 2: {d}\n", .{answer2});
}
