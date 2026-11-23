const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
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

fn sh2ArrayLen() []const u8 {
    sh2TODO("arrayLen");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var arr: [1024]i64 = undefined;
    var arr_len: usize = 0;
    var x: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== Indexed arrays =="});
    try stdout.flush();
    arr_len = 0; // line 7
    arr[arr_len] = sh2ToInt("one");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("two");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("three");
    arr_len += 1;
    // TODO(unsupported): param → sh2.param // line 8
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): arrayLen → sh2.arrayLen
    // TODO(unsupported): arrayLen → sh2.arrayLen
    try stdout.print("{s}\n", .{sh2ArrayLen()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    {
        const sh2_items0 = [_][]const u8{sh2Param()};
        for (sh2_items0[0..]) |sh2_v1| {
            x = sh2_v1;
            try stdout.print("{s}{s}", .{x, " "});
    try stdout.flush();
        }
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 10
    _ = &arr;
}
// 6 construct(s) lowered to TODO markers