const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var var_: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    { // line 4
        const sh2_items0 = [_][]const u8{"BASH_ENV", "ENV", "MAIL"};
        for (sh2_items0[0..]) |sh2_v1| {
            var_ = sh2_v1;
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            // TODO(unsupported): exec → sh2.exec
            _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
        }
    }
    try stdout.print("{s}", .{"parsed OK\n"});
    try stdout.flush();
    _ = &var_;
}
// 6 construct(s) lowered to TODO markers