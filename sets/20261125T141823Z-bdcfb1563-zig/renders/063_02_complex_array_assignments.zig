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

fn sh2ArrayIndex() []const u8 {
    sh2TODO("arrayIndex");
    return "";
}

fn sh2ArrayItems() []const u8 {
    sh2TODO("arrayItems");
    return "";
}

fn sh2ArrayLen() []const u8 {
    sh2TODO("arrayLen");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var matrix_0_0_: []const u8 = "";
    var matrix_1_1_: []const u8 = "";
    var matrix_2_2_: []const u8 = "";
    var matrix_3_3_: []const u8 = "";
    const x: []const u8 = "";
    const y: []const u8 = "";
    const z: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"declare", "-A", "matrix"}); // line 4
    matrix_0_0_ = sh2IntStr(((sh2ToInt(x) + sh2ToInt(y)) * sh2ToInt(z))); // line 5
    // TODO(unsupported): arrayIndex → sh2.arrayIndex // line 6
    // TODO(unsupported): arrayIndex → sh2.arrayIndex
    matrix_1_1_ = sh2ArrayIndex();
    // TODO(unsupported): arrayItems → sh2.arrayItems // line 7
    // TODO(unsupported): arrayItems → sh2.arrayItems
    matrix_2_2_ = sh2ArrayItems();
    // TODO(unsupported): arrayLen → sh2.arrayLen // line 8
    // TODO(unsupported): arrayLen → sh2.arrayLen
    matrix_3_3_ = sh2ArrayLen();
    try stdout.print("{s}\n", .{"Matrix assignments completed"});
    try stdout.flush();
    _ = &matrix_0_0_;
    _ = &matrix_1_1_;
    _ = &matrix_2_2_;
    _ = &matrix_3_3_;
}
// 6 construct(s) lowered to TODO markers