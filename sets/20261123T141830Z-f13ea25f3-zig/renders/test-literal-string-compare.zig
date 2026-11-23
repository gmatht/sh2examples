const std = @import("std");

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (std.mem.eql(u8, "a", "a")) { // line 5
        try stdout.print("{s}\n", .{"eq"});
    try stdout.flush();
    }
    if (!std.mem.eql(u8, "a", "a")) { // line 6
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"ne"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "a", "b")) { // line 7
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"not-equal"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, "hello", "hello")) {
        try stdout.print("{s}\n", .{"var-eq"});
    try stdout.flush();
    }
    if (!std.mem.eql(u8, "hello", "other")) {
        try stdout.print("{s}\n", .{"var-ne"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, sh2IntStr(10), sh2IntStr(10))) { // line 13
        try stdout.print("{s}\n", .{"num-eq"});
    try stdout.flush();
    }
    if (std.mem.eql(u8, sh2IntStr(10), sh2IntStr(10))) { // line 14
        try stdout.print("{s}\n", .{"bad"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"num-string-distinct"});
    try stdout.flush();
    }
}