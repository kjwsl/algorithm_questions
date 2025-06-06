const std = @import("std");


fn castToU64(n: u64) u64 {
    return @intCast(n);
}

fn isqrt(n: u64) u64 {
    var x = n;
    var y = (x + 1) / 2;
    while (y < x) {
        x = y;
        y = (n / x + x) / 2;
    }
    return x;
}
pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    var largest: u64 = 0;
    const target = 600851475143;
    var num: u64 = @intCast(target);

    for (2..isqrt(castToU64(target))) |n| {
        while (@mod(num, @as(u64, @intCast(n))) == 0) {
            if (n > largest) {
                largest = @intCast(n);
            }
            num = @divExact(num, @as(u64, @intCast(n)));
        }
    }

    try stdout.print("{}", .{largest});
}
