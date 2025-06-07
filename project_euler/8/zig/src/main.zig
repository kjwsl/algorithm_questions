const std = @import("std");

fn Queue(comptime T: type, comptime size: usize) type {
    return struct {
        const Self = @This();
        data: [size]T = .{0} ** size,
        head: usize = 0,
        tail: usize = 0,
        current_product: T = 1,
        zero_count: usize = 0,

        pub fn push(self: *Self, value: T) void {
            self.data[self.tail] = value;
            if (value == 0) {
                self.zero_count += 1;
            }  else {
                self.current_product *= value;
            }
            self.tail = (self.tail + 1) % size;
        }

        pub fn pop(self: *Self) T {
            const value = self.data[self.head];
            if (value == 0) {
                self.zero_count -= 1;
            } else {
                self.current_product /= value;
            }
            self.head = (self.head + 1) % size;
            return value;
        }

        pub fn is_empty(self: *Self) bool {
            return self.head == self.tail;
        }

        pub fn product(self: *Self) T {
            return if (self.zero_count > 0) 0 else self.current_product;
        }
    };
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const input = "731671765313306249192251196744265747423553491949396983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    var queue = Queue(u64, 13){};

    for (0..13) |i| {
        queue.push(input[i] - '0');
    }

    var max_product: u64 = queue.product();

    for (13..input.len) |i| {
        _ = queue.pop();
        queue.push(input[i] - '0');
        const current_product = queue.product();
        if (current_product > max_product) {
            max_product = current_product;
        }
    }

    try stdout.print("{}\n", .{max_product});
}
