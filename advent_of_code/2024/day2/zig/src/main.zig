const std = @import("std");

fn readFile(alloc: std.mem.Allocator, path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    const size = (try file.metadata()).size();
    const file_content = try std.fs.cwd().readFileAlloc(alloc, path, size + 1);

    return file_content;
}

fn is_safe(report: []const u8) bool {
    if (report.len == 0) {
        return false; // empty report is not safe
    }
    var levels = std.mem.splitScalar(u8, report, ' ');
    const first = levels.next().?;
    const second = levels.peek().?;
    var prev = std.fmt.parseInt(i32, first, 10) catch unreachable;
    const curr = std.fmt.parseInt(i32, second, 10) catch unreachable;
    const is_asc = prev < curr;
    while (levels.next()) |level| {
        const level_int = std.fmt.parseInt(i32, level, 10) catch unreachable;
        if (@abs(prev - level_int) < 1 or @abs(prev - level_int) > 3) {
            return false; // not safe
        }

        if (is_asc and level_int < prev) {
            return false; // not ascending
        } else if (!is_asc and level_int > prev) {
            return false; // not descending
        }
        prev = level_int;
    }
    return true; // all checks passed
}

fn solve(input: []const u8) i32 {
    var safe_count: i32 = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue; // skip empty lines
        }
        if (is_safe(line)) {
            safe_count += 1;
        }
    }

    return safe_count;
}

pub fn main() !void {
    const stdout = std.io.getStdOut();
    defer stdout.close();
    const writer = stdout.writer();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();
    const input = try readFile(alloc, "input.txt");
    defer alloc.free(input);

    const result = solve(input);
    try writer.print("Safe reports: {any}\n", .{result});
}
