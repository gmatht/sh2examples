const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {
    _ = init;

    // TODO(unsupported): pipeline → sh2.pipeline // line 5
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
}
// 2 construct(s) lowered to TODO markers