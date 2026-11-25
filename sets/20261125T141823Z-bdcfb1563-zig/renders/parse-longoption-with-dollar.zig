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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var X: []const u8 = "";
    _ = init;

    X = "test";
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Assign()));
    _ = &X;
}
// 4 construct(s) lowered to TODO markers