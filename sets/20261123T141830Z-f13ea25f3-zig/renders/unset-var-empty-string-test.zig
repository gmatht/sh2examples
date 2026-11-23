const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

pub fn main(init: std.process.Init) !void {

    const never_set2: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (std.mem.eql(u8, sh2GetVar(), "")) { // line 4
        try stdout.print("{s}\n", .{"unset-is-empty"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"unset-not-empty"});
    try stdout.flush();
    }
    if ((!!std.mem.eql(u8, never_set2, ""))) {
        try stdout.print("{s}\n", .{"zero-len"});
    try stdout.flush();
    }
}