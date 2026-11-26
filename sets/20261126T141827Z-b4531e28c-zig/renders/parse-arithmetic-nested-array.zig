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

fn sh2ArithStub() i64 {
    sh2TODO("arith");
    return 0;
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var arr: [1024]i64 = undefined;
    var arr_len: usize = 0;
    var j: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    j = "0"; // line 3
    arr_len = 0; // line 4
    arr[arr_len] = sh2ToInt("a");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("b");
    arr_len += 1;
    arr[arr_len] = sh2ToInt("c");
    arr_len += 1;
    j = sh2IntStr(sh2ArithStub()); // line 5
    try stdout.print("{s}\n", .{j});
    try stdout.flush(); // line 6
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"arr", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    _ = &arr;
}
// 2 construct(s) lowered to TODO markers