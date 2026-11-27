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

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var array: [1024]i64 = undefined;
    var array_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"Testing complex array operations..."});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"declare", "-a", [_] []const u8{"item1", "item2", "item3"}});
    array[array_len] = sh2ToInt("item4");
    array_len += 1;
    // TODO(unsupported): join → sh2.join // line 7
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Array: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Length: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 9
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"First item: ", sh2Param()});
    try stdout.flush();
    _ = &array;
}
// 6 construct(s) lowered to TODO markers