const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2StrHasSuffix() []const u8 {
    sh2TODO("strHasSuffix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    if (sh2Truthy(sh2StrHasSuffix())) {
        try stdout.print("{s}\n", .{"case=match"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"case=no"});
    try stdout.flush();
    }
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    // TODO(unsupported): strHasSuffix → sh2.strHasSuffix
    if (sh2Truthy(sh2StrHasSuffix())) {
        try stdout.print("{s}\n", .{"case2=match"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"case2=no"});
    try stdout.flush();
    }
}
// 4 construct(s) lowered to TODO markers