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

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

fn sh2StrHasSuffix() []const u8 {
    sh2TODO("strHasSuffix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== [[ pattern and regex ]]"});
    try stdout.flush();
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2StrHasSuffix()) and sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (std.mem.eql(u8, "file.txt", sh2GetVar()) and sh2Truthy(sh2Exec()));
}
// 6 construct(s) lowered to TODO markers