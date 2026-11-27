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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var ch: []const u8 = "";
    var i: []const u8 = "";
    const len: i64 = 0;
    var s: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    s = "HELLOWORLD";
    i = "0"; // line 11
    while ((sh2ToInt(i) < 10)) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        ch = sh2Param();
        try stdout.print("{s}", .{ch});
    try stdout.flush();
        i = sh2IntStr((sh2ToInt(i) + 1));
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    _ = &len;
    _ = &s;
}
// 2 construct(s) lowered to TODO markers