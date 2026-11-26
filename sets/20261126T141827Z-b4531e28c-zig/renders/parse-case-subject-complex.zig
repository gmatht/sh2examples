const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2StrHasPrefix() []const u8 {
    sh2TODO("strHasPrefix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __sh2_casesubj_0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    __sh2_casesubj_0 = std.fmt.allocPrint(std.heap.page_allocator, "{s}-{s}", .{sh2Env("1"), sh2Capture()}) catch "";
    if ((!!std.mem.eql(u8, __sh2_casesubj_0, "regex-Linux"))) {
        try stdout.print("{s}\n", .{"Linux"});
    try stdout.flush();
    // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
    // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
    } else if (sh2Truthy(sh2StrHasPrefix())) {
        try stdout.print("{s}{s}\n", .{"Other: ", sh2Capture()});
    try stdout.flush();
    }
}
// 2 construct(s) lowered to TODO markers