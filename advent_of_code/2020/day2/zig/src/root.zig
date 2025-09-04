//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub fn print(buffer_size: comptime_int, comptime fmt: []const u8, args: anytype) !void {
    var stdout_buffer: [buffer_size]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print(fmt, args);

    try stdout.flush();
}

pub const PasswordSection = struct {
    min: usize,
    max: usize,
    character: u8,
    password: []const u8,
};

fn parseInput(gpa: std.mem.Allocator, input: []const u8) ![]PasswordSection {
    var list = try std.ArrayList(PasswordSection).initCapacity(gpa, 10000);
    defer list.deinit(gpa);

    var tok_iter = std.mem.tokenizeSequence(u8, input, "\n");

    while (tok_iter.next()) |line| {
        var part_iter = std.mem.tokenizeSequence(u8, line, " ");

        const num_constraint = part_iter.next().?;
        const character = part_iter.next().?[0];
        const password = part_iter.next().?;

        var min_max = std.mem.tokenizeSequence(u8, num_constraint, "-");
        const min = try std.fmt.parseInt(usize, min_max.next().?, 10);
        const max = try std.fmt.parseInt(usize, min_max.next().?, 10);

        try list.append(gpa, .{
            .min = min,
            .max = max,
            .character = character,
            .password = password,
        });
    }

    return list.toOwnedSlice(gpa);
}

pub fn part1(gpa: std.mem.Allocator, input: []const u8) !usize {
    var count: usize = 0;
    const sections = try parseInput(gpa, input);
    defer gpa.free(sections);

    for (sections) |section| {
        var char_count: usize = 0;

        for (section.password) |char| {
            if (char == section.character) {
                char_count += 1;
            }
        }

        if (char_count >= section.min and char_count <= section.max) {
            count += 1;
        }
    }

    return count;
}

pub fn part2(gpa: std.mem.Allocator, input: []const u8) !usize {
    var count: usize = 0;
    const sections = try parseInput(gpa, input);
    defer gpa.free(sections);

    for (sections) |section| {
        var char_count: usize = 0;
        if (section.password[section.min - 1] == section.character) {
            char_count += 1;
        }
        if (section.password[section.max - 1] == section.character) {
            char_count += 1;
        }

        if (char_count == 1) {
            count += 1;
        }
    }

    return count;
}
