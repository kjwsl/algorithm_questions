const std = @import("std");

const Instruction = struct { direction: u8, distance: i32 };

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

    var x: i32 = 0;
    var y: i32 = 0;

    const directions = [_]u8{ 'N', 'E', 'S', 'W' };
    var current_direction: usize = 0;

    for (instructions) |instruction| {
        current_direction = switch (instruction.direction) {
            'R' => (current_direction + 1) % directions.len,
            'L' => (current_direction + directions.len - 1) % directions.len,
            else => current_direction,
        };

        switch (directions[current_direction]) {
            'N' => y += instruction.distance,
            'E' => x += instruction.distance,
            'S' => y -= instruction.distance,
            'W' => x -= instruction.distance,
            else => unreachable,
        }
    }

    return @intCast(@abs(x) + @abs(y));
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("../sample.txt", .{});
    defer file.close();

    var buf: [1024]u8 = undefined;
    var reader = file.reader();

    const len = try reader.read(&buf);

    const answer = try part1(buf[0 .. len - 1]);
    std.debug.print("{d}\n", .{answer});
}
