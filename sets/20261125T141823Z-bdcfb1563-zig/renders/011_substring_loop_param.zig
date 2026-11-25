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

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn print_chars(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    var i: []const u8 = "";
    var len: []const u8 = "";
    var s: []const u8 = "";
    s = (if (__args.len >= 1) __args[1 - 1] else "");
    i = "0";
    len = (if (__args.len >= 2) __args[2 - 1] else "");
    while ((sh2ToInt(i) < sh2ToInt(len))) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        ch = sh2Param();
        try stdout.print("{s}", .{ch});
    try stdout.flush();
        i = sh2IntStr((sh2ToInt(i) + 1));
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    const ch: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"print_chars", "HELLOWORLD", "10"});
}
// 2 construct(s) lowered to TODO markers