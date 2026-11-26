const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2ListVar() []const u8 {
    sh2TODO("listVar");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var X_SET: i64 = 0;
    var arg: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    X_SET = 0; // line 4
    // TODO(unsupported): listVar → sh2.listVar // line 5
    // TODO(unsupported): listVar → sh2.listVar
    {
        const sh2_items0 = [_][]const u8{sh2ListVar()};
        for (sh2_items0[0..]) |sh2_v1| {
            arg = sh2_v1;
            // TODO(unsupported): pipeline → sh2.pipeline
            // TODO(unsupported): pipeline → sh2.pipeline
            if (sh2Truthy(sh2Pipeline())) {
                X_SET = 1;
            }
        }
    }
    try stdout.print("{d}\n", .{X_SET});
    try stdout.flush(); // line 10
    _ = &arg;
}
// 4 construct(s) lowered to TODO markers