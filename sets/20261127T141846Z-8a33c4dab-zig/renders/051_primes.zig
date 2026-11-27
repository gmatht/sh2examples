const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Assign() []const u8 {
    sh2TODO("assign");
    return "";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}


fn is_prime(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    var i: i64 = 0;
    _ = sh2Run(&[_][]const u8{"local", "n=$1"});
    if ((sh2ToInt(sh2GetVar()) < 2)) {
        return "1";
    }
    if ((sh2ToInt(sh2GetVar()) == 2)) {
        return "0";
    }
    if ((@rem(sh2ToInt(sh2GetVar()), 2) == 0)) {
        return "1";
    }
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    _ = sh2Run(&[_][]const u8{"local", "sqrt_n=", sh2Capture()});
    i = 3;
    while ((i <= sh2ToInt(sh2GetVar()))) {
        if (sh2Test()) {
            return "1";
        }
        i = (i + 2);
    }
    return "0";
}
pub fn main(init: std.process.Init) !void {

    var candidate: i64 = 0;
    var count: i64 = 0;
    var primes: [1024]i64 = undefined;
    var primes_len: usize = 0;
    var y: []const u8 = "";
    var z: [1024]i64 = undefined;
    var z_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    // TODO(unsupported): assign → sh2.assign // line 7
    // TODO(unsupported): assign → sh2.assign
    y = sh2Assign();
    z[z_len] = sh2ToInt("a"); // line 8
    z_len += 1;
    z[z_len] = sh2ToInt("b");
    z_len += 1;
    // TODO(unsupported): assign → sh2.assign // line 9
    // TODO(unsupported): assign → sh2.assign
    z = sh2Assign();
    try stdout.print("{s}\n", .{"=== Prime Number Generator (first 1000 primes) ==="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Finding first 100 prime numbers..."});
    try stdout.flush();
    try stdout.print("{s}\n", .{"This may take a while..."});
    try stdout.flush();
    primes_len = 0; // line 45
    primes[primes_len] = 2;
    primes_len += 1;
    count = 1; // line 46
    candidate = 3; // line 47
    while ((count < 100)) {
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        if (sh2Truthy(sh2Exec())) {
            primes[primes_len] = sh2ToInt(sh2IntStr(candidate));
            primes_len += 1;
            count = (count + 1);
            if ((@rem(count, 10) == 0)) {
                try stdout.print("{s}{d}{s}\n", .{"Found ", count, " primes so far..."});
    try stdout.flush();
            }
        }
        candidate = (candidate + 2);
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"First 1000 prime numbers found!"});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Count: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 65
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"First 10: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 66
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Last 10: ", sh2Join()});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Prime number generation complete!"});
    try stdout.flush();
    _ = &primes;
    _ = &y;
    _ = &z;
}
// 14 construct(s) lowered to TODO markers