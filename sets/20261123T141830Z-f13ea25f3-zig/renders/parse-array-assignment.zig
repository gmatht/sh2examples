const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var arr: [1024]i64 = undefined;
    var arr_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    arr_len = 0; // line 3
    arr[arr_len] = sh2ToInt("one");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("two");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("three");
    arr_len += 1;
    // TODO(unsupported): param → sh2.param // line 4
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    _ = &arr;
}
// 2 construct(s) lowered to TODO markers