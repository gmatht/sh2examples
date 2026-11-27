const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}


fn factorial(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    var i: i64 = 0;
    var result: i64 = 0;
    _ = sh2Run(&[_][]const u8{"local", "n=$1"});
    result = 1;
    // declare i
    i = 2;
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    while (sh2Truthy(sh2Exec())) {
        result = (result * i);
        i += 1;
    }
    return sh2IntStr(result);
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{factorial(init, &[_][]const u8{ "5" })});
    try stdout.flush();
    try stdout.print("{s}\n", .{factorial(init, &[_][]const u8{ "6" })});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers