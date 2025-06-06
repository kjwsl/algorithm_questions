const std = @import("std");

pub fn main() !void {
    var prevs = [_]i32{1, 2};
    var sum: i32 = 2;

    while (prevs[1] < 4_000_000) {
        const lsum = prevs[0] + prevs[1];
        if (@mod(lsum, 2) == 0) {
            sum += lsum;
        }

        prevs[0] = prevs[1];
        prevs[1] = lsum;

    }

    std.debug.print("{}", .{sum});
}
