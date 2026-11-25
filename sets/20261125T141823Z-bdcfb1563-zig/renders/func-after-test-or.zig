const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Assign() []const u8 {
    sh2TODO("assign");
    return "";
}


fn my_func(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    return "hello";
}
pub fn main(init: std.process.Init) !void {

    const VAR: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    _ = (!std.mem.eql(u8, VAR, "") or sh2Truthy(sh2Assign()));
    try stdout.print("{s}\n", .{my_func(init, &[_][]const u8{  })});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers