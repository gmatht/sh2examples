const std = @import("std");

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

pub fn main(init: std.process.Init) !void {

    var a: i64 = 0;
    var b: i64 = 0;
    var c: i64 = 0;
    var d: i64 = 0;
    var e: i64 = 0;
    var f: i64 = 0;
    var g: i64 = 0;
    var h: i64 = 0;
    var i: i64 = 0;
    var j: i64 = 0;
    var k: i64 = 0;
    var l: i64 = 0;
    var m: i64 = 0;
    var n: i64 = 0;
    var result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    a = 10; // line 4
    b = 3;
    c = 8;
    d = 2;
    e = 7;
    f = 2;
    g = 2;
    h = 3;
    i = 4;
    j = 1;
    k = 6;
    l = 3;
    m = 5;
    n = 2;
    result = sh2IntStr((((@divTrunc(((a + b) * (c - d)), @rem(e, f)) + std.math.pow(i64, g, h)) - (i << j)) | ((k & l) ^ (m | n)))); // line 5
    try stdout.print("{s}{s}\n", .{"Deeply nested arithmetic result: ", result});
    try stdout.flush(); // line 6
}