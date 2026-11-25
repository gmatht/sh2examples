const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Shopt() []const u8 {
    sh2TODO("shopt");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): shopt → sh2.shopt // line 3
    // TODO(unsupported): shopt → sh2.shopt
    _ = sh2Shopt();
    if (std.mem.eql(u8, "foo", "@(foo|bar)")) {
        try stdout.print("{s}\n", .{"matched"});
    try stdout.flush();
    }
    try stdout.print("{s}", .{"parsed OK\n"});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers