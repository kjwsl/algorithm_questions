const std = @import("std");

fn is_solvable(target: i64, numbers: []const i64) !bool {
    // Initialize a stack as an ArrayList
    const allocator = std.heap.page_allocator;
    var stack = std.ArrayList(i64).init(allocator);
    defer stack.deinit(); // Free memory at the end

    // Push all numbers to the stack
    for (numbers) |number| {
        try stack.append(number);
    }

    // Simplify the stack by adding the top two numbers repeatedly
    while (stack.items.len > 1) {
        const a = stack.pop();
        const b = stack.pop();
        const result = a + b;
        try stack.append(result);
    }

    // Check if the final result matches the target
    return stack.items[0] == target;
}

pub fn solve(input: []const u8) !i64 {
    const allocator = std.heap.page_allocator;
    var sum: i64 = 0;

    // Split input string by comma
    var it = std.mem.splitScalar(u8, input, ',');
    const target_str = it.next() orelse return error.InvalidInput;
    const numbers_str = it.next() orelse return error.InvalidInput;

    // Parse the target number
    const target = try std.fmt.parseInt(i64, target_str, 10);

    // Split numbers_str by spaces and parse them into an ArrayList
    var numbers = std.ArrayList(i64).init(allocator);
    defer numbers.deinit(); // Free memory at the end

    var numbers_it = std.mem.splitScalar(u8, numbers_str, ' ');
    while (numbers_it.next()) |number_str| {
        if (number_str.len > 0) { // Ignore empty parts
            const number = try std.fmt.parseInt(i64, number_str, 10);
            try numbers.append(number);
        }
    }

    // Check if the target can be achieved
    if (try is_solvable(target, numbers.items)) {
        sum += target;
    }

    return sum;
}
