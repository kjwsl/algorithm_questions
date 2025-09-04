const std = @import("std");

fn print(buffer_size: comptime_int, comptime fmt: []const u8, args: anytype) !void {
    var stdout_buffer: [buffer_size]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print(fmt, args);

    try stdout.flush();
}

const Pair = struct {
    a: u32,
    b: u32,
};

fn parseInput(gpa: std.mem.Allocator, input: []const u8) ![]u32 {
    var token_iter = std.mem.tokenizeSequence(u8, input, "\n");
    var nums = try std.ArrayList(u32).initCapacity(gpa, 10000);
    while (token_iter.next()) |token| {
        const num = try std.fmt.parseInt(u32, token, 10);
        try nums.append(gpa, num);
    }
    return nums.toOwnedSlice(gpa);
}

fn findPairForSum(gpa: std.mem.Allocator, target_sum: u32, nums: *const []u32) ?Pair {
    var map = std.AutoHashMap(u32, u32).init(gpa);
    defer map.deinit();

    for (nums.*) |num| {
        if (num > target_sum) {
            continue;
        }

        if (map.contains(num)) {
            return Pair{ .a = num, .b = map.get(num).? };
        }

        map.put(target_sum - num, num) catch return null;
    }

    return null;
}

fn part1(gpa: std.mem.Allocator, input: []const u8) !u32 {
    const TARGET_SUM = 2020;

    const nums = try parseInput(gpa, input);
    defer gpa.free(nums);
    const pair = findPairForSum(gpa, TARGET_SUM, &nums);
    if (pair) |p| {
        return p.a * p.b;
    }
    unreachable;
}

fn part2(gpa: std.mem.Allocator, input: []const u8) !u32 {
    const TARGET_SUM = 2020;

    const nums = try parseInput(gpa, input);
    defer gpa.free(nums);
    for (nums) |num| {
        const pair = findPairForSum(gpa, TARGET_SUM - num, &nums);
        if (pair) |p| {
            return num * p.a * p.b;
        }
    }
    unreachable;

}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const input = try std.fs.cwd().readFileAlloc(alloc, "../input.txt", 1024 * 1024);
    const result = try part1(alloc, input);

    try print(100, "Part 1: {}\n", .{result});
    try print(100, "Part 2: {}\n", .{try part2(alloc, input)});
}
